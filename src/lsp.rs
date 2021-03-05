use lspower::{
    jsonrpc::{Error, ErrorCode, Result},
    lsp::*,
    Client, LanguageServer, LspService, Server,
};
use quench::db;
use std::sync::Arc;

enum ServerErrorCode {
    // https://microsoft.github.io/language-server-protocol/specifications/specification-3-16/#responseMessage
    DocNotInCache = -31999,
}

struct Backend {
    #[allow(dead_code)]
    client: Client,
    state: Arc<db::State>,
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
        // TODO: figure out how to log instead of unwrap here
        self.state.open_document(params).await.unwrap();
        let diagnostics = self.state.get_diagnostics(uri.clone()).await.unwrap();
        self.client
            .publish_diagnostics(uri, diagnostics, None)
            .await;
    }

    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        let uri = params.text_document.uri.clone();
        // TODO: figure out how to log instead of unwrap here
        self.state.edit_document(params).await.unwrap();
        let diagnostics = self.state.get_diagnostics(uri.clone()).await.unwrap();
        self.client
            .publish_diagnostics(uri, diagnostics, None)
            .await;
    }

    async fn did_close(&self, params: DidCloseTextDocumentParams) {
        // TODO: figure out how to log instead of unwrap here
        self.state.close_document(params).await.unwrap();
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
            .map_err(|_| Error {
                code: ErrorCode::ServerError(ServerErrorCode::DocNotInCache as i64),
                message: format!("URI not in document cache: {}", uri),
                data: None,
            })?;
        Ok(Some(SemanticTokensResult::Tokens(SemanticTokens {
            result_id: None,
            data: tokens,
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
