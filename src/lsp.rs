use lspower::{
    jsonrpc::{Error, ErrorCode, Result},
    lsp::*,
    Client, LanguageServer, LspService, Server,
};
use quench::parser;
use std::{
    collections::HashMap,
    convert::TryFrom,
    sync::{Arc, RwLock},
};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;
use tree_sitter::Node;

enum ServerErrorCode {
    // https://microsoft.github.io/language-server-protocol/specifications/specification-3-16/#responseMessage
    DocNotInCache = -31999,
}

struct Backend {
    #[allow(dead_code)]
    client: Client,
    documents: Arc<RwLock<HashMap<Url, String>>>,
}

// TODO: account for UTF-8 vs UTF-16
fn node_lsp_range(node: &Node) -> Range {
    let start = node.start_position();
    let end = node.end_position();
    Range {
        // TODO: figure out an alternative to unwrap here
        start: Position {
            line: u32::try_from(start.row).unwrap(),
            character: u32::try_from(start.column).unwrap(),
        },
        end: Position {
            line: u32::try_from(end.row).unwrap(),
            character: u32::try_from(end.column).unwrap(),
        },
    }
}

// TODO: test this function
fn diagnostics(node: &Node) -> Vec<Diagnostic> {
    if let Some(message) = {
        if node.is_error() {
            Some("error")
        } else if node.is_missing() {
            Some("missing")
        } else {
            None
        }
    } {
        vec![Diagnostic {
            range: node_lsp_range(node),
            severity: Some(DiagnosticSeverity::Error),
            code: None,
            code_description: None,
            source: None,
            message: format!("syntax {}", message),
            related_information: None,
            tags: None,
            data: None,
        }]
    } else {
        let mut cursor = node.walk();
        node.children(&mut cursor)
            .map(|child| diagnostics(&child))
            .flatten()
            .collect()
    }
}

#[derive(Clone, Copy, EnumIter)]
enum TokenType {
    Comment,
    String,
    Variable,
}

fn semantic_token_type(token_type: TokenType) -> SemanticTokenType {
    match token_type {
        TokenType::Comment => SemanticTokenType::COMMENT,
        TokenType::String => SemanticTokenType::STRING,
        TokenType::Variable => SemanticTokenType::VARIABLE,
    }
}

struct AbsoluteToken {
    line: u32,
    start: u32,
    length: u32,
    token_type: TokenType,
}

// TODO: test this function
fn semantic_tokens(node: &Node) -> Vec<AbsoluteToken> {
    if let Some(token_type) = match node.kind() {
        // TODO: make these not stringly typed
        "comment" => Some(TokenType::Comment),
        "string" => Some(TokenType::String),
        "identifier" => Some(TokenType::Variable),
        _ => None,
    } {
        let range = node_lsp_range(node);
        if range.start.line == range.end.line {
            return vec![AbsoluteToken {
                line: range.start.line,
                start: range.start.character,
                length: range.end.character - range.start.character,
                token_type,
            }];
        }
    }
    let mut cursor = node.walk();
    node.children(&mut cursor)
        .map(|child| semantic_tokens(&child))
        .flatten()
        .collect()
}

// TODO: test this function
fn make_relative(tokens: Vec<AbsoluteToken>) -> Vec<SemanticToken> {
    let mut relative = vec![];
    let mut it = tokens.iter();
    match it.next() {
        None => relative,
        Some(first) => {
            relative.push(SemanticToken {
                delta_line: first.line,
                delta_start: first.start,
                length: first.length,
                token_type: first.token_type as u32,
                token_modifiers_bitset: 0,
            });
            let mut last_line = first.line;
            let mut last_start = first.start;
            for token in it {
                relative.push(SemanticToken {
                    delta_line: token.line - last_line,
                    delta_start: if token.line == last_line {
                        token.start - last_start
                    } else {
                        token.start
                    },
                    length: token.length,
                    token_type: token.token_type as u32,
                    token_modifiers_bitset: 0,
                });
                last_line = token.line;
                last_start = token.start;
            }
            relative
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
                                token_types: TokenType::iter().map(semantic_token_type).collect(),
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
        let doc = params.text_document;
        let docs = Arc::clone(&self.documents);
        let mut docs = docs.write().unwrap(); // TODO: figure out how to log instead of unwrap here
        docs.insert(doc.uri, doc.text);
    }

    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        let uri = params.text_document.uri;
        let text = params
            .content_changes
            .into_iter()
            .map(|x| x.text)
            .collect::<Vec<_>>()
            .join("");
        let docs = Arc::clone(&self.documents);
        let mut docs = docs.write().unwrap(); // TODO: figure out how to log instead of unwrap here
        docs.insert(uri, text);
    }

    async fn did_close(&self, params: DidCloseTextDocumentParams) {
        let docs = Arc::clone(&self.documents);
        let mut docs = docs.write().unwrap(); // TODO: figure out how to log instead of unwrap here
        docs.remove(&params.text_document.uri);
    }

    async fn semantic_tokens_full(
        &self,
        params: SemanticTokensParams,
    ) -> Result<Option<SemanticTokensResult>> {
        let uri = params.text_document.uri;
        let (diags, tokens) = {
            let docs = Arc::clone(&self.documents);
            let docs = docs.read().unwrap(); // TODO: figure out how to log instead of unwrap here
            let text = docs.get(&uri).ok_or(Error {
                code: ErrorCode::ServerError(ServerErrorCode::DocNotInCache as i64),
                message: format!("URI not in document cache: {}", uri),
                data: None,
            })?;
            let mut parser = parser::parser();
            // TODO: refactor this to encapsulate the unwrap
            let tree = parser.parse(text, None).unwrap();
            (
                diagnostics(&tree.root_node()),
                make_relative(semantic_tokens(&tree.root_node())),
            )
        };
        self.client.publish_diagnostics(uri, diags, None).await;
        Ok(Some(SemanticTokensResult::Tokens(SemanticTokens {
            result_id: None,
            data: tokens,
        })))
    }
}

#[tokio::main]
async fn main() {
    let (service, messages) = LspService::new(|client| Backend {
        client,
        documents: Arc::new(RwLock::new(HashMap::new())),
    });
    Server::new(tokio::io::stdin(), tokio::io::stdout())
        .interleave(messages)
        .serve(service)
        .await;
}
