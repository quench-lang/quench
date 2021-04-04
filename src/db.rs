use crate::{parser, text};
use lspower::lsp::{
    Diagnostic, DiagnosticSeverity, DidChangeTextDocumentParams, DidCloseTextDocumentParams,
    DidOpenTextDocumentParams, Position, Range, SemanticToken, SemanticTokenType,
    TextDocumentContentChangeEvent,
};
use std::{fmt::Debug, ptr, rc::Rc};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;
use tree_sitter::{Node, Point, Tree};
use url::Url;

#[derive(Debug)]
pub struct Ast(pub Tree);

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

#[salsa::query_group(Storage)]
pub trait QueryGroup: salsa::Database {
    // we don't track versions because we only allow full text sync
    #[salsa::input]
    fn opened_files(&self) -> im::HashSet<Url>;

    #[salsa::input]
    fn source_text(&self, key: Url) -> Rc<String>;

    fn source_index(&self, key: Url) -> Option<text::Index>;

    fn ast(&self, key: Url) -> Option<Rc<Ast>>;

    fn diagnostics(&self, key: Url) -> im::Vector<Diagnostic>;

    fn semantic_tokens(&self, key: Url) -> im::Vector<SemanticToken>;

    fn compile(&self, key: Url) -> Option<serde_json::Value>;
}

fn source_index(db: &dyn QueryGroup, key: Url) -> Option<text::Index> {
    if db.opened_files().contains(&key) {
        Some(text::Index::new(&db.source_text(key)))
    } else {
        None
    }
}

fn ast(db: &dyn QueryGroup, key: Url) -> Option<Rc<Ast>> {
    if db.opened_files().contains(&key) {
        let mut parser = parser::parser();
        let text: &str = &db.source_text(key);
        // parser::parser guarantees language is set, and we haven't set timeout or cancellation
        let tree = parser.parse(text, None).unwrap();
        Some(Rc::new(Ast(tree)))
    } else {
        None
    }
}

#[salsa::database(Storage)]
pub struct Database {
    storage: salsa::Storage<Self>,
}

impl Default for Database {
    fn default() -> Self {
        let mut db = Database {
            storage: salsa::Storage::default(),
        };
        db.set_opened_files(im::HashSet::new());
        db
    }
}

impl salsa::Database for Database {}

pub trait Processable<T> {
    fn process(self, db: &mut Database) -> T;
}

#[derive(Debug, Eq, thiserror::Error, PartialEq)]
#[error("{uri} was already opened")]
pub struct AlreadyOpenError {
    uri: Url,
}

#[derive(Debug, Eq, thiserror::Error, PartialEq)]
#[error("{uri} not yet opened")]
pub struct NotYetOpenedError {
    uri: Url,
}

impl Database {
    pub fn open_document(&mut self, uri: Url, text: String) -> Result<(), AlreadyOpenError> {
        // we always call set_source_text, even if the file is already opened, because we want to
        // give the client the benefit of the doubt and assume that we've made a bookkeeping
        // mistake, rather than risk possibly dropping data
        self.set_source_text(uri.clone(), Rc::new(text));
        let mut files = self.opened_files();
        if let Some(_) = files.insert(uri.clone()) {
            Err(AlreadyOpenError { uri })
        } else {
            self.set_opened_files(files);
            Ok(())
        }
    }
}

impl Processable<Result<(), AlreadyOpenError>> for DidOpenTextDocumentParams {
    fn process(self, db: &mut Database) -> Result<(), AlreadyOpenError> {
        let doc = self.text_document;
        db.open_document(doc.uri, doc.text)
    }
}

impl Database {
    fn edit_document(&mut self, uri: Url, text: String) -> Result<(), NotYetOpenedError> {
        // we always call set_source_text, even if the file hadn't yet been opened, because we want
        // to give the client the benefit of the doubt and assume that we've made a bookkeeping
        // mistake, rather than risk possibly dropping data
        self.set_source_text(uri.clone(), Rc::new(text));
        let mut files = self.opened_files();
        if !files.contains(&uri) {
            files.insert(uri.clone());
            self.set_opened_files(files);
            Err(NotYetOpenedError { uri })
        } else {
            Ok(())
        }
    }
}

#[derive(Debug, Eq, thiserror::Error, PartialEq)]
pub enum EditError {
    #[error(transparent)]
    NotYetOpened(#[from] NotYetOpenedError),
    #[error("incremental sync (when full text was expected) for version {version} of {uri}")]
    IncrementalSync {
        // same fields as lsp_types::VersionedTextDocumentIdentifier
        uri: Url,
        version: i32,
    },
}

impl Processable<Result<(), EditError>> for DidChangeTextDocumentParams {
    fn process(self, db: &mut Database) -> Result<(), EditError> {
        let doc = self.text_document;
        let mut changes = self.content_changes;
        if let Some(TextDocumentContentChangeEvent {
            range: None,
            range_length: None,
            text,
        }) = changes.pop()
        {
            if changes.is_empty() {
                return Ok(db.edit_document(doc.uri, text)?);
            }
        }
        Err(EditError::IncrementalSync {
            uri: doc.uri,
            version: doc.version,
        })
    }
}

impl Database {
    fn close_document(&mut self, uri: Url) -> Result<(), NotYetOpenedError> {
        // Salsa doesn't seem to support removing inputs https://github.com/salsa-rs/salsa/issues/37
        // so we just free most of the memory (hopefully?) by setting it to the empty string; also,
        // we always call set_source_text, even if the file hadn't yet been opened, because we want
        // to give the client the benefit of the doubt and assume that we've just made a bookkeeping
        // mistake
        self.set_source_text(uri.clone(), Rc::new(String::from("")));
        let mut files = self.opened_files();
        if let None = files.remove(&uri) {
            Err(NotYetOpenedError { uri })
        } else {
            self.set_opened_files(files);
            Ok(())
        }
    }
}

impl Processable<Result<(), NotYetOpenedError>> for DidCloseTextDocumentParams {
    fn process(self, db: &mut Database) -> Result<(), NotYetOpenedError> {
        db.close_document(self.text_document.uri)
    }
}

fn make_diagnostic(range: Range, message: String, severity: DiagnosticSeverity) -> Diagnostic {
    let mut diag = Diagnostic::new_simple(range, message);
    diag.severity = Some(severity);
    diag
}

fn diagnostics_helper(node: &Node, index: &text::Index) -> im::Vector<Diagnostic> {
    if node.is_error() || node.is_missing() {
        im::vector![make_diagnostic(
            Range::new(
                index.to_lsp(node.start_position()),
                index.to_lsp(node.end_position()),
            ),
            format!("syntax {}", node.to_sexp()),
            DiagnosticSeverity::Error,
        )]
    } else {
        let mut diagnostics = im::vector![];
        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            diagnostics.append(diagnostics_helper(&child, index));
        }
        diagnostics
    }
}

fn diagnostics(db: &dyn QueryGroup, key: Url) -> im::Vector<Diagnostic> {
    match (db.source_index(key.clone()), db.ast(key)) {
        (Some(index), Some(tree)) => diagnostics_helper(&tree.0.root_node(), &index),
        _ => im::vector![],
    }
}

pub struct DiagnosticsRequest(pub Url);

impl Processable<im::Vector<Diagnostic>> for DiagnosticsRequest {
    fn process(self, db: &mut Database) -> im::Vector<Diagnostic> {
        let DiagnosticsRequest(uri) = self;
        db.diagnostics(uri)
    }
}

#[derive(Clone)]
struct AbsoluteToken {
    line: u32,
    start: u32,
    length: u32,
    token_type: TokenType,
}

impl AbsoluteToken {
    fn via_index(
        index: &text::Index,
        row: usize,
        column: usize,
        end: Option<usize>,
        token_type: TokenType,
    ) -> AbsoluteToken {
        let Position {
            line,
            character: start,
        } = index.to_lsp(Point { row, column });
        AbsoluteToken {
            line,
            start,
            length: index
                .to_lsp(Point {
                    row,
                    column: end.unwrap_or(usize::MAX), // saturates left to the end of the line
                })
                .character
                - start,
            token_type,
        }
    }
}

fn absolute_tokens(node: &Node, index: &text::Index) -> im::Vector<AbsoluteToken> {
    if let Some(token_type) = match node.kind() {
        "comment" => Some(TokenType::Comment),
        "string" => Some(TokenType::String),
        "identifier" => Some(TokenType::Variable),
        _ => None,
    } {
        let start = node.start_position();
        let end = node.end_position();
        let mut tokens = im::Vector::new();
        let mut column = start.column;
        for row in start.row..end.row {
            tokens.push_back(AbsoluteToken::via_index(
                index, row, column, None, token_type,
            ));
            column = 0;
        }
        tokens.push_back(AbsoluteToken::via_index(
            index,
            end.row,
            column,
            Some(end.column),
            token_type,
        ));
        tokens
    } else {
        let mut tokens = im::vector![];
        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            tokens.append(absolute_tokens(&child, index));
        }
        tokens
    }
}

fn make_relative(tokens: im::Vector<AbsoluteToken>) -> im::Vector<SemanticToken> {
    let mut relative = im::vector![];
    let mut it = tokens.iter();
    match it.next() {
        None => relative,
        Some(first) => {
            relative.push_back(SemanticToken {
                delta_line: first.line,
                delta_start: first.start,
                length: first.length,
                token_type: first.token_type as u32,
                token_modifiers_bitset: 0,
            });
            let mut last_line = first.line;
            let mut last_start = first.start;
            for token in it {
                relative.push_back(SemanticToken {
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

fn semantic_tokens(db: &dyn QueryGroup, key: Url) -> im::Vector<SemanticToken> {
    match (db.source_index(key.clone()), db.ast(key)) {
        (Some(index), Some(tree)) => make_relative(absolute_tokens(&tree.0.root_node(), &index)),
        _ => im::vector![],
    }
}

pub struct TokensRequest(pub Url);

impl Processable<im::Vector<SemanticToken>> for TokensRequest {
    fn process(self, db: &mut Database) -> im::Vector<SemanticToken> {
        let TokensRequest(uri) = self;
        db.semantic_tokens(uri)
    }
}

fn compile_helper(text: &str, node: &Node) -> Option<serde_json::Value> {
    use serde_json::{json, Value};
    match node.kind() {
        "source_file" => {
            let mut cursor = node.walk();
            let body: Vec<Value> = node
                .children(&mut cursor)
                .filter_map(|child| compile_helper(text, &child))
                .map(|expr| {
                    json!({
                        "type": "ExpressionStatement",
                        "expression": expr,
                    })
                })
                .collect();
            Some(json!({
                "type": "Program",
                "body": body,
            }))
        }
        "call" => {
            let func = compile_helper(text, &node.child_by_field_name("function")?)?;
            let args = compile_helper(text, &node.child_by_field_name("arguments")?)?;
            Some(json!({
                "type": "CallExpression",
                "callee": func,
                "arguments": args,
            }))
        }
        "arguments" => {
            let mut cursor = node.walk();
            Some(Value::Array(
                node.children(&mut cursor)
                    .filter_map(|child| compile_helper(text, &child))
                    .collect(),
            ))
        }
        "identifier" => match node.utf8_text(text.as_bytes()).ok()? {
            "print" => Some(json!({
                "type": "MemberExpression",
                "object": {
                    "type": "Identifier",
                    "name": "console",
                },
                "property": {
                    "type": "Identifier",
                    "name": "log",
                },
            })),
            _ => None,
        },
        "string" => {
            let value = node
                .utf8_text(text.as_bytes())
                .ok()?
                .strip_prefix("\"")?
                .strip_suffix("\"")?;
            Some(json!({
                "type": "Literal",
                "value": value,
            }))
        }
        _ => None,
    }
}

pub fn compile(db: &dyn QueryGroup, key: Url) -> Option<serde_json::Value> {
    let tree = db.ast(key.clone())?;
    let source = db.source_text(key); // at this point we already know the key is valid
    compile_helper(&source, &tree.0.root_node())
}

#[cfg(test)]
mod tests {
    use super::*;
    use lspower::lsp::VersionedTextDocumentIdentifier;
    use pretty_assertions::assert_eq;

    fn foo_db(text: String) -> (Database, Url) {
        let mut db = Database::default();
        let uri = Url::parse("file:///tmp/foo.qn").unwrap();
        db.open_document(uri.clone(), String::from(text)).unwrap();
        (db, uri)
    }

    #[test]
    fn test_already_open_error() {
        let (mut db, uri) = foo_db(String::from("foo"));
        assert_eq!(
            db.open_document(uri.clone(), String::from("bar")),
            Err(AlreadyOpenError { uri: uri.clone() }),
        );
        assert_eq!(db.opened_files(), im::hashset![uri.clone()]);
        let new_contents: &str = &db.source_text(uri);
        assert_eq!(new_contents, "bar");
    }

    #[test]
    fn test_edit_not_yet_opened_error() {
        let mut db = Database::default();
        let uri = Url::parse("file:///tmp/foo.qn").unwrap();
        assert_eq!(
            db.edit_document(uri.clone(), String::from("bar")),
            Err(NotYetOpenedError { uri: uri.clone() }),
        );
        assert_eq!(db.opened_files(), im::hashset![uri.clone()]);
        let new_contents: &str = &db.source_text(uri);
        assert_eq!(new_contents, "bar");
    }

    #[test]
    fn test_close_not_yet_opened_error() {
        let mut db = Database::default();
        let uri = Url::parse("file:///tmp/foo.qn").unwrap();
        assert_eq!(
            db.close_document(uri.clone()),
            Err(NotYetOpenedError { uri: uri.clone() }),
        );
        assert_eq!(db.opened_files(), im::hashset![]);
    }

    fn make_range(
        start_line: u32,
        start_character: u32,
        end_line: u32,
        end_character: u32,
    ) -> Range {
        Range::new(
            Position::new(start_line, start_character),
            Position::new(end_line, end_character),
        )
    }

    #[test]
    fn test_incremental_sync() {
        let (mut db, uri) = foo_db(String::from("foo"));
        let params = DidChangeTextDocumentParams {
            text_document: VersionedTextDocumentIdentifier {
                uri: uri.clone(),
                version: 2,
            },
            content_changes: vec![TextDocumentContentChangeEvent {
                range: Some(make_range(0, 0, 0, 3)),
                range_length: None,
                text: String::from("bar"),
            }],
        };
        assert_eq!(
            params.process(&mut db),
            Err(EditError::IncrementalSync {
                uri: uri.clone(),
                version: 2
            }),
        );
        let new_contents: &str = &db.source_text(uri);
        assert_eq!(new_contents, "foo");
    }

    #[test]
    fn test_full_sync() {
        let (mut db, uri) = foo_db(String::from("foo"));
        let params = DidChangeTextDocumentParams {
            text_document: VersionedTextDocumentIdentifier {
                uri: uri.clone(),
                version: 2,
            },
            content_changes: vec![TextDocumentContentChangeEvent {
                range: None,
                range_length: None,
                text: String::from("bar"),
            }],
        };
        params.process(&mut db).unwrap();
        let new_contents: &str = &db.source_text(uri);
        assert_eq!(new_contents, "bar");
    }

    fn make_error(sl: u32, sc: u32, el: u32, ec: u32, message: &str) -> Diagnostic {
        make_diagnostic(
            make_range(sl, sc, el, ec),
            String::from(message),
            DiagnosticSeverity::Error,
        )
    }

    #[test]
    fn test_diagnostics_hello_world() {
        let (db, uri) = foo_db(slurp::read_all_to_string("examples/errors.qn").unwrap());
        let diagnostics = db.diagnostics(uri);
        assert_eq!(
            diagnostics,
            im::vector![
                make_error(0, 6, 0, 14, "syntax (ERROR (string))"),
                make_error(0, 24, 0, 24, "syntax (MISSING \";\")"),
            ],
        );
    }

    fn make_token(
        delta_line: u32,
        delta_start: u32,
        length: u32,
        token_type: TokenType,
    ) -> SemanticToken {
        SemanticToken {
            delta_line,
            delta_start,
            length,
            token_type: token_type as u32,
            token_modifiers_bitset: 0,
        }
    }

    #[test]
    fn test_tokens_hello_world() {
        let (db, uri) = foo_db(slurp::read_all_to_string("examples/hello.qn").unwrap());
        let tokens = db.semantic_tokens(uri);
        assert_eq!(
            tokens,
            im::vector![
                make_token(0, 0, 21, TokenType::Comment),
                make_token(2, 0, 5, TokenType::Variable),
                make_token(0, 6, 15, TokenType::String),
            ],
        );
    }

    #[test]
    fn test_tokens_multiline() {
        let (db, uri) = foo_db(String::from(
            "print(\"long line\nvery long middle line\nshort\");",
        ));
        let tokens = db.semantic_tokens(uri);
        assert_eq!(
            tokens,
            im::vector![
                make_token(0, 0, 5, TokenType::Variable),
                make_token(0, 6, 10, TokenType::String),
                make_token(1, 0, 21, TokenType::String),
                make_token(1, 0, 6, TokenType::String),
            ]
        );
    }

    #[test]
    fn test_compile_hello_world() {
        let (db, uri) = foo_db(slurp::read_all_to_string("examples/hello.qn").unwrap());
        let compiled = db.compile(uri);
        assert_eq!(
            compiled,
            Some(serde_json::json!({
                "type": "Program",
                "body": [
                    {
                        "type": "ExpressionStatement",
                        "expression": {
                            "type": "CallExpression",
                            "callee": {
                                "type": "MemberExpression",
                                "object": {
                                    "type": "Identifier",
                                    "name": "console",
                                },
                                "property": {
                                    "type": "Identifier",
                                    "name": "log",
                                },
                            },
                            "arguments": [
                                {
                                    "type": "Literal",
                                    "value": "Hello, world!",
                                },
                            ],
                        },
                    },
                ],
            })),
        );
    }
}
