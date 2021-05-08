mod state {
    use crate::db::{self, EditError};
    use lspower::lsp::{
        Diagnostic, DidChangeTextDocumentParams, DidCloseTextDocumentParams,
        DidOpenTextDocumentParams, MessageType, Range, SemanticToken,
    };
    use std::{fmt::Debug, thread};
    use tokio::sync::{mpsc, oneshot};
    use url::Url;

    trait Request {
        fn handle(&mut self, db: &mut db::Database);
    }

    // https://stackoverflow.com/a/48066387
    impl<T, U> Request for Option<(T, oneshot::Sender<U>)>
    where
        T: db::Processable<U>,
    {
        fn handle(&mut self, mut db: &mut db::Database) {
            let (params, tx) = self.take().unwrap();
            let _ = tx.send(params.process(&mut db));
        }
    }

    type BoxedRequest = Box<dyn Request + Send>;

    #[derive(Debug, thiserror::Error)]
    pub enum AsyncError {
        #[error("state loop ended prematurely")]
        Send,
        #[error("failed to receive result from state loop")]
        Recv,
    }

    impl From<mpsc::error::SendError<BoxedRequest>> for AsyncError {
        fn from(_: mpsc::error::SendError<BoxedRequest>) -> Self {
            AsyncError::Send
        }
    }

    impl From<oneshot::error::RecvError> for AsyncError {
        fn from(_: oneshot::error::RecvError) -> Self {
            AsyncError::Recv
        }
    }

    pub trait LspMessage {
        fn message_type(&self) -> MessageType;
    }

    impl LspMessage for AsyncError {
        fn message_type(&self) -> MessageType {
            MessageType::Error
        }
    }

    #[derive(Debug, thiserror::Error)]
    pub enum OpError<T: 'static + Debug + std::error::Error> {
        #[error(transparent)]
        Async(#[from] AsyncError),
        #[error("{0}")] // I think this is equivalent to #[error(transparent)]
        Op(#[source] T),
    }

    impl<T: 'static + Debug + std::error::Error + LspMessage> LspMessage for OpError<T> {
        fn message_type(&self) -> MessageType {
            match self {
                OpError::Async(error) => error.message_type(),
                OpError::Op(error) => error.message_type(),
            }
        }
    }

    impl LspMessage for db::AlreadyOpenError {
        fn message_type(&self) -> MessageType {
            MessageType::Warning
        }
    }

    impl LspMessage for db::NotYetOpenedError {
        fn message_type(&self) -> MessageType {
            MessageType::Warning
        }
    }

    impl LspMessage for EditError {
        fn message_type(&self) -> MessageType {
            match self {
                EditError::NotYetOpened(error) => error.message_type(),
                EditError::IncrementalSync { .. } => MessageType::Error,
            }
        }
    }

    pub struct State {
        tx: mpsc::Sender<BoxedRequest>,
    }

    impl State {
        pub fn new() -> Self {
            let (tx, mut rx) = mpsc::channel::<BoxedRequest>(1);
            // we do this in a non-async thread because our db isn't thread-safe
            thread::spawn(move || {
                let mut db = db::Database::default();
                // https://stackoverflow.com/a/52521592
                while let Some(mut request) = futures::executor::block_on(rx.recv()) {
                    request.handle(&mut db);
                }
            });
            State { tx }
        }

        // confusing given that the Processable trait has a different method with the same name
        async fn process<T, U>(&self, params: T) -> Result<U, AsyncError>
        where
            T: db::Processable<U> + Send + 'static,
            U: Send + 'static,
        {
            let (tx, rx) = oneshot::channel();
            self.tx.send(Box::new(Some((params, tx)))).await?;
            let result = rx.await?;
            Ok(result)
        }

        pub async fn open_document(
            &self,
            params: DidOpenTextDocumentParams,
        ) -> Result<(), OpError<db::AlreadyOpenError>> {
            self.process(params).await?.map_err(OpError::Op)
        }

        pub async fn edit_document(
            &self,
            params: DidChangeTextDocumentParams,
        ) -> Result<(), OpError<EditError>> {
            self.process(params).await?.map_err(OpError::Op)
        }

        pub async fn close_document(
            &self,
            params: DidCloseTextDocumentParams,
        ) -> Result<(), OpError<db::NotYetOpenedError>> {
            self.process(params).await?.map_err(OpError::Op)
        }

        pub async fn get_compile_diagnostics(
            &self,
            uri: Url,
        ) -> Result<im::Vector<Diagnostic>, AsyncError> {
            Ok(match self.process(db::CompileRequest(uri.clone())).await? {
                Ok(_) => im::vector![],
                Err(diagnostics) => {
                    let index = self.process(db::IndexRequest(uri)).await?.unwrap();
                    diagnostics
                        .into_iter()
                        .map(|diag| {
                            let mut lsp_diag = Diagnostic::new_simple(
                                Range::new(
                                    index.to_lsp(diag.range.start_point),
                                    index.to_lsp(diag.range.end_point),
                                ),
                                diag.message,
                            );
                            lsp_diag.severity = Some(diag.severity);
                            lsp_diag
                        })
                        .collect()
                }
            })
        }

        pub async fn get_semantic_tokens(
            &self,
            uri: Url,
        ) -> Result<im::Vector<SemanticToken>, AsyncError> {
            self.process(db::TokensRequest(uri)).await
        }
    }
}

use crate::db;
use lspower::{
    jsonrpc::{Error, ErrorCode, Result},
    lsp::{
        DidChangeTextDocumentParams, DidCloseTextDocumentParams, DidOpenTextDocumentParams,
        InitializeParams, InitializeResult, SemanticTokens, SemanticTokensFullOptions,
        SemanticTokensLegend, SemanticTokensOptions, SemanticTokensParams, SemanticTokensResult,
        SemanticTokensServerCapabilities, ServerCapabilities, TextDocumentSyncCapability,
        TextDocumentSyncKind, WorkDoneProgressOptions,
    },
    Client, LanguageServer, LspService, Server,
};
use state::LspMessage;
use std::sync::Arc;
use url::Url;

enum ServerErrorCode {
    // https://microsoft.github.io/language-server-protocol/specifications/specification-3-16/#responseMessage
    DbAsyncError = -31999,
}

struct Backend {
    #[allow(dead_code)]
    client: Client,
    state: Arc<state::State>,
}

impl Backend {
    async fn push_diagnostics(&self, uri: Url) {
        match self.state.get_compile_diagnostics(uri.clone()).await {
            Err(error) => {
                self.client.show_message(error.message_type(), error).await;
            }
            Ok(diagnostics) => {
                self.client
                    .publish_diagnostics(uri, diagnostics.into_iter().collect(), None)
                    .await;
            }
        }
    }
}

#[lspower::async_trait]
impl LanguageServer for Backend {
    async fn initialize(&self, _: InitializeParams) -> Result<InitializeResult> {
        Ok(InitializeResult {
            capabilities: ServerCapabilities {
                text_document_sync: Some(TextDocumentSyncCapability::Kind(
                    TextDocumentSyncKind::Full,
                )),
                semantic_tokens_provider: Some(
                    SemanticTokensServerCapabilities::SemanticTokensOptions(
                        SemanticTokensOptions {
                            work_done_progress_options: WorkDoneProgressOptions {
                                work_done_progress: Some(false),
                            },
                            legend: SemanticTokensLegend {
                                token_types: db::token_types(),
                                token_modifiers: vec![],
                            },
                            range: Some(false),
                            full: Some(SemanticTokensFullOptions::Delta { delta: Some(false) }),
                        },
                    ),
                ),
                ..ServerCapabilities::default()
            },
            server_info: None,
        })
    }

    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        let uri = params.text_document.uri.clone();
        if let Err(error) = self.state.open_document(params).await {
            // could join this with following await
            self.client.show_message(error.message_type(), error).await;
        }
        self.push_diagnostics(uri).await;
    }

    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        let uri = params.text_document.uri.clone();
        if let Err(error) = self.state.edit_document(params).await {
            // could join this with following await
            self.client.show_message(error.message_type(), error).await;
        }
        self.push_diagnostics(uri).await;
    }

    async fn did_close(&self, params: DidCloseTextDocumentParams) {
        if let Err(error) = self.state.close_document(params).await {
            self.client.show_message(error.message_type(), error).await;
        }
    }

    async fn semantic_tokens_full(
        &self,
        params: SemanticTokensParams,
    ) -> Result<Option<SemanticTokensResult>> {
        let uri = params.text_document.uri;
        let tokens = self
            .state
            .get_semantic_tokens(uri.clone())
            .await
            .map_err(|error| Error {
                code: ErrorCode::ServerError(ServerErrorCode::DbAsyncError as i64),
                message: format!("{}", error),
                data: None,
            })?;
        Ok(Some(SemanticTokensResult::Tokens(SemanticTokens {
            result_id: None,
            data: tokens.into_iter().collect(),
        })))
    }
}

#[tokio::main]
pub async fn main() {
    let state = state::State::new();
    let (service, messages) = LspService::new(|client| Backend {
        client,
        state: Arc::new(state),
    });
    Server::new(tokio::io::stdin(), tokio::io::stdout())
        .interleave(messages)
        .serve(service)
        .await;
}
