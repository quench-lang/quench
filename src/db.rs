use crate::parser;
use lspower::lsp::{
    Diagnostic, DiagnosticSeverity, DidChangeTextDocumentParams, DidCloseTextDocumentParams,
    DidOpenTextDocumentParams, Position, Range, SemanticToken, SemanticTokenType,
};
use std::convert::TryFrom;
use std::{ptr, rc::Rc, thread};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;
use tokio::sync::{mpsc, oneshot};
use tree_sitter::{Node, Tree};
use url::Url;

#[derive(Debug)]
struct Ast(Tree);

impl PartialEq for Ast {
    fn eq(&self, other: &Self) -> bool {
        // only used by Tree-sitter after an update to try to save work if the result didn't "really
        // change", so we just do the bare minimum to make this an equivalence relation (by
        // comparing the references) rather than wasting time checking the entire tree for equality
        ptr::eq(self, other)
    }
}

impl Eq for Ast {}

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

pub fn token_types() -> Vec<SemanticTokenType> {
    TokenType::iter().map(semantic_token_type).collect()
}

struct AbsoluteToken {
    line: u32,
    start: u32,
    length: u32,
    token_type: TokenType,
}

#[salsa::query_group(Storage)]
trait QueryGroup: salsa::Database {
    #[salsa::input]
    fn source_text(&self, key: Url) -> Rc<String>;

    fn ast(&self, key: Url) -> Rc<Ast>;

    fn diagnostics(&self, key: Url) -> Rc<Vec<Diagnostic>>;

    fn semantic_tokens(&self, key: Url) -> Rc<Vec<SemanticToken>>;
}

fn ast(db: &dyn QueryGroup, key: Url) -> Rc<Ast> {
    let mut parser = parser::parser();
    let text: &str = &db.source_text(key);
    let tree = parser.parse(text, None).unwrap();
    Rc::new(Ast(tree))
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
fn diagnostics_helper(node: &Node) -> Vec<Diagnostic> {
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
            .map(|child| diagnostics_helper(&child))
            .flatten()
            .collect()
    }
}

fn diagnostics(db: &dyn QueryGroup, key: Url) -> Rc<Vec<Diagnostic>> {
    Rc::new(diagnostics_helper(&db.ast(key).0.root_node()))
}

// TODO: test this function
fn absolute_tokens(node: &Node) -> Vec<AbsoluteToken> {
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
        .map(|child| absolute_tokens(&child))
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

fn semantic_tokens(db: &dyn QueryGroup, key: Url) -> Rc<Vec<SemanticToken>> {
    Rc::new(make_relative(absolute_tokens(&db.ast(key).0.root_node())))
}

#[salsa::database(Storage)]
#[derive(Default)]
struct Database {
    storage: salsa::Storage<Self>,
}

impl salsa::Database for Database {}

#[derive(Debug)]
enum Request {
    Open {
        params: DidOpenTextDocumentParams,
        tx: oneshot::Sender<()>,
    },
    Edit {
        params: DidChangeTextDocumentParams,
        tx: oneshot::Sender<()>,
    },
    Close {
        params: DidCloseTextDocumentParams,
        tx: oneshot::Sender<()>,
    },
    Diagnostics {
        uri: Url,
        tx: oneshot::Sender<Vec<Diagnostic>>,
    },
    Tokens {
        uri: Url,
        tx: oneshot::Sender<Vec<SemanticToken>>,
    },
}

pub struct State {
    tx: mpsc::Sender<Request>,
}

impl State {
    pub fn new() -> Self {
        let (tx, mut rx) = mpsc::channel::<Request>(1);
        // we do this in a non-async thread because our db isn't thread-safe
        thread::spawn(move || {
            let mut db = Database::default();
            // https://stackoverflow.com/a/52521592/5044950
            while let Some(request) = futures::executor::block_on(rx.recv()) {
                match request {
                    Request::Open { params, tx } => {
                        let doc = params.text_document;
                        db.set_source_text(doc.uri, Rc::new(doc.text));
                        // TODO: warn if the document was already opened
                        let _ = tx.send(());
                    }
                    Request::Edit { params, tx } => {
                        let uri = params.text_document.uri;
                        let text = params
                            .content_changes
                            .into_iter()
                            .map(|x| x.text)
                            .collect::<Vec<_>>()
                            .join("");
                        db.set_source_text(uri, Rc::new(text));
                        // TODO: warn if the document wasn't already opened
                        let _ = tx.send(());
                    }
                    Request::Close { params: _, tx } => {
                        // TODO: figure out how to remove Salsa inputs
                        let _ = tx.send(());
                    }
                    Request::Tokens { uri, tx } => {
                        let tokens = db.semantic_tokens(uri);
                        let _ = tx.send((*tokens).clone());
                    }
                    Request::Diagnostics { uri, tx } => {
                        let diagnostics = db.diagnostics(uri);
                        let _ = tx.send((*diagnostics).clone());
                    }
                }
            }
        });
        State { tx }
    }

    pub async fn open_document(&self, params: DidOpenTextDocumentParams) -> anyhow::Result<()> {
        let (tx, rx) = oneshot::channel();
        self.tx.send(Request::Open { params, tx }).await?;
        let () = rx.await?;
        Ok(())
    }

    pub async fn edit_document(&self, params: DidChangeTextDocumentParams) -> anyhow::Result<()> {
        let (tx, rx) = oneshot::channel();
        self.tx.send(Request::Edit { params, tx }).await?;
        let () = rx.await?;
        Ok(())
    }

    pub async fn close_document(&self, params: DidCloseTextDocumentParams) -> anyhow::Result<()> {
        let (tx, rx) = oneshot::channel();
        self.tx.send(Request::Close { params, tx }).await?;
        let () = rx.await?;
        Ok(())
    }

    pub async fn get_diagnostics(&self, uri: Url) -> anyhow::Result<Vec<Diagnostic>> {
        let (tx, rx) = oneshot::channel();
        self.tx.send(Request::Diagnostics { uri, tx }).await?;
        let diagnostics = rx.await?;
        Ok(diagnostics)
    }

    pub async fn get_semantic_tokens(&self, uri: Url) -> anyhow::Result<Vec<SemanticToken>> {
        let (tx, rx) = oneshot::channel();
        self.tx.send(Request::Tokens { uri, tx }).await?;
        let tokens = rx.await?;
        Ok(tokens)
    }
}
