use crate::{compiler, diagnosis::Diagnostic, estree, opts::Opts, parser, syntax, text};
use lspower::lsp::{
    DiagnosticSeverity, DidChangeTextDocumentParams, DidCloseTextDocumentParams,
    DidOpenTextDocumentParams, Position, SemanticToken, SemanticTokenType,
    TextDocumentContentChangeEvent,
};
use std::{fmt::Debug, ptr, rc::Rc, sync::Arc};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;
use tree_sitter::{Node, Point, Tree};
use url::Url;

#[derive(Debug)]
pub struct Cst(pub Tree);

impl PartialEq for Cst {
    fn eq(&self, other: &Self) -> bool {
        // only used by Tree-sitter after an update to try to save work if the result didn't "really
        // change", so we just do the bare minimum to make this an equivalence relation (by
        // comparing the references) rather than wasting time checking the entire tree for equality
        ptr::eq(self, other)
    }
}

impl Eq for Cst {}

#[derive(Clone, Copy, EnumIter)]
enum TokenType {
    Boolean,
    Comment,
    Integer,
    Null,
    String,
    Symbol,
    Variable,
}

fn semantic_token_type(token_type: TokenType) -> SemanticTokenType {
    match token_type {
        TokenType::Boolean => SemanticTokenType::ENUM_MEMBER,
        TokenType::Comment => SemanticTokenType::COMMENT,
        TokenType::Integer => SemanticTokenType::NUMBER,
        TokenType::Null => SemanticTokenType::ENUM_MEMBER,
        TokenType::String => SemanticTokenType::STRING,
        TokenType::Symbol => SemanticTokenType::PROPERTY,
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

    fn cst(&self, key: Url) -> Option<Rc<Cst>>;

    fn cst_diagnostics(&self, key: Url) -> im::Vector<Diagnostic>;

    fn semantic_tokens(&self, key: Url) -> im::Vector<SemanticToken>;

    fn ast(&self, key: Url) -> Result<Rc<syntax::File>, im::Vector<Diagnostic>>;

    fn compile(&self, key: Url, opts: Opts)
        -> Result<Arc<estree::Program>, im::Vector<Diagnostic>>;
}

fn source_index(db: &dyn QueryGroup, key: Url) -> Option<text::Index> {
    if db.opened_files().contains(&key) {
        Some(text::Index::new(&db.source_text(key)))
    } else {
        None
    }
}

pub struct IndexRequest(pub Url);

impl Processable<Option<text::Index>> for IndexRequest {
    fn process(self, db: &mut Database) -> Option<text::Index> {
        let IndexRequest(uri) = self;
        db.source_index(uri)
    }
}

fn cst(db: &dyn QueryGroup, key: Url) -> Option<Rc<Cst>> {
    if db.opened_files().contains(&key) {
        let mut parser = parser::parser();
        let text: &str = &db.source_text(key);
        // parser::parser guarantees language is set, and we haven't set timeout or cancellation
        let tree = parser.parse(text, None).unwrap();
        Some(Rc::new(Cst(tree)))
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

fn diagnostics_helper(node: &Node, index: &text::Index) -> im::Vector<Diagnostic> {
    if node.is_error() || node.is_missing() {
        im::vector![Diagnostic {
            range: node.range(),
            severity: DiagnosticSeverity::Error,
            message: format!("syntax {}", node.to_sexp()),
        }]
    } else {
        let mut diagnostics = im::vector![];
        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            diagnostics.append(diagnostics_helper(&child, index));
        }
        diagnostics
    }
}

fn cst_diagnostics(db: &dyn QueryGroup, key: Url) -> im::Vector<Diagnostic> {
    match (db.source_index(key.clone()), db.cst(key)) {
        (Some(index), Some(tree)) => diagnostics_helper(&tree.0.root_node(), &index),
        _ => im::vector![],
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
        "boolean" => Some(TokenType::Boolean),
        "comment" => Some(TokenType::Comment),
        "integer" => Some(TokenType::Integer),
        "null" => Some(TokenType::Null),
        "string" => Some(TokenType::String),
        "symbol" => Some(TokenType::Symbol),
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
    match (db.source_index(key.clone()), db.cst(key)) {
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

fn ast(db: &dyn QueryGroup, key: Url) -> Result<Rc<syntax::File>, im::Vector<Diagnostic>> {
    let cst_diags = db.cst_diagnostics(key.clone());
    if !cst_diags.is_empty() {
        Err(cst_diags)
    } else {
        let tree = db.cst(key.clone()).ok_or(im::vector![])?;
        let source = db.source_text(key); // at this point we already know the key is valid
        syntax::Node::make(&source, &tree.0.root_node()).map(Rc::new)
    }
}

pub fn compile(
    db: &dyn QueryGroup,
    key: Url,
    opts: Opts,
) -> Result<Arc<estree::Program>, im::Vector<Diagnostic>> {
    let tree = db.ast(key.clone())?;
    compiler::compile_file(&tree, &opts).map(Arc::new)
}

pub struct CompileRequest(pub Url, pub Opts);

impl Processable<Result<Arc<estree::Program>, im::Vector<Diagnostic>>> for CompileRequest {
    fn process(self, db: &mut Database) -> Result<Arc<estree::Program>, im::Vector<Diagnostic>> {
        let CompileRequest(uri, opts) = self;
        db.compile(uri, opts)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use lspower::lsp::{Range, VersionedTextDocumentIdentifier};
    use pretty_assertions::assert_eq;
    use std::fs;

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

    fn lsp_range(
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
                range: Some(lsp_range(0, 0, 0, 3)),
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

    fn ts_range(
        sb: usize,
        sp: (usize, usize),
        eb: usize,
        ep: (usize, usize),
    ) -> tree_sitter::Range {
        tree_sitter::Range {
            start_byte: sb,
            start_point: Point::new(sp.0, sp.1),
            end_byte: eb,
            end_point: Point::new(ep.0, ep.1),
        }
    }

    fn make_error(
        sb: usize,
        sp: (usize, usize),
        eb: usize,
        ep: (usize, usize),
        message: &str,
    ) -> Diagnostic {
        Diagnostic {
            range: ts_range(sb, sp, eb, ep),
            severity: DiagnosticSeverity::Error,
            message: String::from(message),
        }
    }

    #[test]
    fn test_diagnostics_hello_world() {
        let (db, uri) = foo_db(fs::read_to_string("examples/errors.qn").unwrap());
        let diagnostics = db.cst_diagnostics(uri);
        assert_eq!(
            diagnostics,
            im::vector![
                make_error(10, (0, 10), 12, (0, 12), "syntax (ERROR)"),
                make_error(34, (0, 34), 34, (0, 34), "syntax (MISSING \";\")"),
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
        let (db, uri) = foo_db(fs::read_to_string("examples/hello.qn").unwrap());
        let tokens = db.semantic_tokens(uri);
        assert_eq!(
            tokens,
            im::vector![
                make_token(0, 0, 21, TokenType::Comment),
                make_token(1, 0, 4, TokenType::Variable),
                make_token(0, 8, 1, TokenType::Variable),
                make_token(0, 5, 5, TokenType::Variable),
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
    fn test_ast_hello_world() {
        let (db, uri) = foo_db(fs::read_to_string("examples/hello.qn").unwrap());
        let tree = db.ast(uri).unwrap();
        let expected = syntax::File {
            range: ts_range(0, (0, 0), 58, (2, 0)),
            decls: vec![syntax::Decl {
                range: ts_range(22, (1, 0), 57, (1, 35)),
                name: syntax::Id {
                    range: ts_range(22, (1, 0), 26, (1, 4)),
                    name: String::from("main"),
                },
                val: syntax::Expr::Func(syntax::Func {
                    range: ts_range(30, (1, 8), 56, (1, 34)),
                    param: syntax::Id {
                        range: ts_range(30, (1, 8), 31, (1, 9)),
                        name: String::from("_"),
                    },
                    body: Box::new(syntax::Expr::Call(syntax::Call {
                        range: ts_range(35, (1, 13), 56, (1, 34)),
                        func: Box::new(syntax::Expr::Id(syntax::Id {
                            range: ts_range(35, (1, 13), 40, (1, 18)),
                            name: String::from("print"),
                        })),
                        arg: Box::new(syntax::Expr::Lit(syntax::Lit::Str(syntax::Str {
                            range: ts_range(41, (1, 19), 56, (1, 34)),
                            val: String::from("Hello, world!"),
                        }))),
                    })),
                }),
            }],
        };
        assert_eq!(tree.as_ref(), &expected);
    }

    #[test]
    fn test_compile_hello_world() {
        let (db, uri) = foo_db(fs::read_to_string("examples/hello.qn").unwrap());
        let compiled = db
            .compile(
                uri,
                Opts {
                    stdlib_placeholder: true,
                },
            )
            .unwrap();
        assert_eq!(
            serde_json::to_value(compiled.as_ref()).unwrap(),
            serde_json::json!({
                "type": "Program",
                "sourceType": "module",
                "body": [
                    {
                        "type": "ImportDeclaration",
                        "specifiers": [
                            {
                                "type": "ImportDefaultSpecifier",
                                "local": {
                                    "type": "Identifier",
                                    "name": "Immutable"
                                }
                            }
                        ],
                        "source": {
                            "type": "Literal",
                            "value": "https://example.com/quench.js"
                        }
                    },
                    {
                        "type": "VariableDeclaration",
                        "declarations": [
                            {
                                "type": "VariableDeclarator",
                                "id": {
                                    "type": "Identifier",
                                    "name": "$main"
                                },
                                "init": {
                                    "type": "ArrowFunctionExpression",
                                    "id": null,
                                    "params": [
                                        {
                                            "type": "Identifier",
                                            "name": "$_"
                                        }
                                    ],
                                    "body": {
                                        "type": "CallExpression",
                                        "callee": {
                                            "type": "MemberExpression",
                                            "object": {
                                                "type": "Identifier",
                                                "name": "console"
                                            },
                                            "property": {
                                                "type": "Identifier",
                                                "name": "log"
                                            },
                                            "computed": false,
                                        },
                                        "arguments": [
                                            {
                                                "type": "Literal",
                                                "value": "Hello, world!",
                                            }
                                        ],
                                    },
                                    "generator": false,
                                    "expression": true
                                }
                            }
                        ],
                        "kind": "const"
                    },
                    {
                        "type": "ExpressionStatement",
                        "expression": {
                            "type": "CallExpression",
                            "callee": {
                                "type": "Identifier",
                                "name": "$main"
                            },
                            "arguments": [],
                        }
                    }
                ],
            }),
        );
    }
}
