use lspower::{
    jsonrpc::{Error, ErrorCode, Result},
    lsp::*,
    Client, LanguageServer, LspService, Server,
};
use quench::db;
use std::sync::Arc;

enum ServerErrorCode {
    // https://microsoft.github.io/language-server-protocol/specifications/specification-3-16/#responseMessage
    DbAsyncError = -31999,
}

struct Backend {
    #[allow(dead_code)]
    client: Client,
    state: Arc<db::State>,
}

impl Backend {
    async fn push_diagnostics(&self, uri: Url) {
        match self.state.get_diagnostics(uri.clone()).await {
            Err(error) => {
                self.client.show_message(MessageType::Error, error).await;
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
            self.client.show_message(MessageType::Error, error).await;
        }
        self.push_diagnostics(uri).await;
    }

    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        let uri = params.text_document.uri.clone();
        if let Err(error) = self.state.edit_document(params).await {
            // could join this with following await
            self.client.show_message(MessageType::Error, error).await;
        }
        self.push_diagnostics(uri).await;
    }

    async fn did_close(&self, params: DidCloseTextDocumentParams) {
        if let Err(error) = self.state.close_document(params).await {
            self.client.show_message(MessageType::Error, error).await;
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
async fn main() {
    let state = db::State::new();
    let (service, messages) = LspService::new(|client| Backend {
        client,
        state: Arc::new(state),
    });
    Server::new(tokio::io::stdin(), tokio::io::stdout())
        .interleave(messages)
        .serve(service)
        .await;
}
