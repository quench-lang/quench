use crate::diagnosis::Diagnostic;
use lspower::lsp::DiagnosticSeverity;
use std::{fmt::Debug, str::FromStr};
use tree_sitter::Range;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct File {
    pub range: Range,
    pub decls: Vec<Decl>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Decl {
    pub range: Range,
    pub name: Id,
    pub val: Expr,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Stmt {
    Decl(Decl),
    ExprStmt(ExprStmt),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ExprStmt {
    pub range: Range,
    pub expr: Expr,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Expr {
    Lit(Lit),
    Id(Id),
    Block(Block),
    Call(Call),
    Func(Func),
    Index(Index),
    Field(Field),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Lit {
    Null { range: Range },
    Bool(Bool),
    Int(Int),
    Str(Str),
    Sym(Sym),
    List(List),
    Map(Map),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Bool {
    pub range: Range,
    pub val: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Int {
    pub range: Range,
    pub val: i32,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Str {
    pub range: Range,
    pub val: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Sym {
    pub range: Range,
    pub name: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct List {
    pub range: Range,
    pub items: Vec<Expr>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Map {
    pub range: Range,
    pub entries: Vec<Entry>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Entry {
    pub range: Range,
    pub key: Expr,
    pub val: Expr,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Id {
    pub range: Range,
    pub name: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Block {
    pub range: Range,
    pub stmts: Vec<Stmt>, // nonempty
    pub expr: Option<Box<Expr>>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Call {
    pub range: Range,
    pub func: Box<Expr>,
    pub arg: Box<Expr>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Func {
    pub range: Range,
    pub param: Id,
    pub body: Box<Expr>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Index {
    pub range: Range,
    pub coll: Box<Expr>,
    pub key: Box<Expr>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Field {
    pub range: Range,
    pub map: Box<Expr>,
    pub key: Sym,
}

pub trait Node {
    fn make(text: &str, node: &tree_sitter::Node) -> Result<Self, im::Vector<Diagnostic>>
    where
        Self: Sized;

    fn range(&self) -> Range;
}

fn make_children<T: Debug + Node>(
    text: &str,
    parent: &tree_sitter::Node,
    field: &str,
) -> Result<Vec<T>, im::Vector<Diagnostic>> {
    let mut cursor = parent.walk();
    let (children, diagnosticses): (Vec<_>, Vec<_>) = parent
        .children_by_field_name(field, &mut cursor)
        .map(|child| T::make(text, &child))
        .partition(Result::is_ok);
    if diagnosticses.is_empty() {
        Ok(children.into_iter().map(Result::unwrap).collect())
    } else {
        let mut flattened = im::vector![];
        for diagnostics in diagnosticses.into_iter().map(Result::unwrap_err) {
            flattened.append(diagnostics);
        }
        Err(flattened)
    }
}

impl Node for File {
    fn make(text: &str, node: &tree_sitter::Node) -> Result<Self, im::Vector<Diagnostic>> {
        Ok(File {
            range: node.range(),
            decls: make_children(text, node, "declaration")?,
        })
    }

    fn range(&self) -> Range {
        self.range
    }
}

impl Node for Decl {
    fn make(text: &str, node: &tree_sitter::Node) -> Result<Self, im::Vector<Diagnostic>> {
        Ok(Decl {
            range: node.range(),
            name: Id::make(text, &node.child_by_field_name("name").unwrap())?,
            val: Expr::make(text, &node.child_by_field_name("value").unwrap())?,
        })
    }

    fn range(&self) -> Range {
        self.range
    }
}

impl Node for Stmt {
    fn make(text: &str, node: &tree_sitter::Node) -> Result<Self, im::Vector<Diagnostic>> {
        match node.kind() {
            "declaration" => Decl::make(text, node).map(Stmt::Decl),
            "expression_statement" => ExprStmt::make(text, node).map(Stmt::ExprStmt),
            kind => Err(im::vector![Diagnostic {
                range: node.range(),
                severity: DiagnosticSeverity::Error,
                message: format!("did not expect a node of kind {}", kind),
            }]),
        }
    }

    fn range(&self) -> Range {
        match self {
            Stmt::Decl(x) => x.range(),
            Stmt::ExprStmt(x) => x.range(),
        }
    }
}

impl Node for ExprStmt {
    fn make(text: &str, node: &tree_sitter::Node) -> Result<Self, im::Vector<Diagnostic>> {
        Ok(ExprStmt {
            range: node.range(),
            expr: Expr::make(text, &node.child_by_field_name("expression").unwrap())?,
        })
    }

    fn range(&self) -> Range {
        self.range
    }
}

impl Node for Expr {
    fn make(text: &str, node: &tree_sitter::Node) -> Result<Self, im::Vector<Diagnostic>> {
        match node.kind() {
            "parenthesized" => Expr::make(text, &node.child_by_field_name("expression").unwrap()),
            "identifier" => Id::make(text, node).map(Expr::Id),
            "block" => Block::make(text, node).map(Expr::Block),
            "call" => Call::make(text, node).map(Expr::Call),
            "function" => Func::make(text, node).map(Expr::Func),
            "index" => Index::make(text, node).map(Expr::Index),
            "field" => Field::make(text, node).map(Expr::Field),
            _ => Lit::make(text, node).map(Expr::Lit),
        }
    }

    fn range(&self) -> Range {
        match self {
            Expr::Lit(x) => x.range(),
            Expr::Id(x) => x.range(),
            Expr::Block(x) => x.range(),
            Expr::Call(x) => x.range(),
            Expr::Func(x) => x.range(),
            Expr::Index(x) => x.range(),
            Expr::Field(x) => x.range(),
        }
    }
}

impl Node for Lit {
    fn make(text: &str, node: &tree_sitter::Node) -> Result<Self, im::Vector<Diagnostic>> {
        match node.kind() {
            "null" => Ok(Lit::Null {
                range: node.range(),
            }),
            "boolean" => Bool::make(text, node).map(Lit::Bool),
            "integer" => Int::make(text, node).map(Lit::Int),
            "string" => Str::make(text, node).map(Lit::Str),
            "symbol" => Sym::make(text, node).map(Lit::Sym),
            "list" => List::make(text, node).map(Lit::List),
            "map" => Map::make(text, node).map(Lit::Map),
            kind => Err(im::vector![Diagnostic {
                range: node.range(),
                severity: DiagnosticSeverity::Error,
                message: format!("did not expect a node of kind {}", kind),
            }]),
        }
    }

    fn range(&self) -> Range {
        match self {
            Lit::Null { range } => *range,
            Lit::Bool(x) => x.range(),
            Lit::Int(x) => x.range(),
            Lit::Str(x) => x.range(),
            Lit::Sym(x) => x.range(),
            Lit::List(x) => x.range(),
            Lit::Map(x) => x.range(),
        }
    }
}

impl Node for Bool {
    fn make(text: &str, node: &tree_sitter::Node) -> Result<Self, im::Vector<Diagnostic>> {
        Ok(Bool {
            range: node.range(),
            val: match node.utf8_text(text.as_bytes()).unwrap() {
                "true" => true,
                "false" => false,
                _ => unreachable!(),
            },
        })
    }

    fn range(&self) -> Range {
        self.range
    }
}

impl Node for Int {
    fn make(text: &str, node: &tree_sitter::Node) -> Result<Self, im::Vector<Diagnostic>> {
        Ok(Int {
            range: node.range(),
            val: {
                let s = node.utf8_text(text.as_bytes()).unwrap();
                i32::from_str(s).map_err(|e| {
                    im::vector![Diagnostic {
                        range: node.range(),
                        severity: DiagnosticSeverity::Error,
                        message: format!("{}", e),
                    }]
                })?
            },
        })
    }

    fn range(&self) -> Range {
        self.range
    }
}

impl Node for Str {
    fn make(text: &str, node: &tree_sitter::Node) -> Result<Self, im::Vector<Diagnostic>> {
        // we assume UTF-8, and our grammar guarantees that the string starts and ends with quotes
        let value = node
            .utf8_text(text.as_bytes())
            .unwrap()
            .strip_prefix("\"")
            .unwrap()
            .strip_suffix("\"")
            .unwrap();
        Ok(Str {
            range: node.range(),
            val: String::from(value),
        })
    }

    fn range(&self) -> Range {
        self.range
    }
}

impl Node for Sym {
    fn make(text: &str, node: &tree_sitter::Node) -> Result<Self, im::Vector<Diagnostic>> {
        Ok(Sym {
            range: node.range(),
            name: node
                .utf8_text(text.as_bytes())
                .unwrap()
                .strip_prefix(".")
                .unwrap()
                .to_string(),
        })
    }

    fn range(&self) -> Range {
        self.range
    }
}

impl Node for List {
    fn make(text: &str, node: &tree_sitter::Node) -> Result<Self, im::Vector<Diagnostic>> {
        Ok(List {
            range: node.range(),
            items: make_children(text, node, "item")?,
        })
    }

    fn range(&self) -> Range {
        self.range
    }
}

impl Node for Map {
    fn make(text: &str, node: &tree_sitter::Node) -> Result<Self, im::Vector<Diagnostic>> {
        Ok(Map {
            range: node.range(),
            entries: make_children(text, node, "entry")?,
        })
    }

    fn range(&self) -> Range {
        self.range
    }
}

impl Node for Entry {
    fn make(text: &str, node: &tree_sitter::Node) -> Result<Self, im::Vector<Diagnostic>> {
        Ok(Entry {
            range: node.range(),
            key: Expr::make(text, &node.child_by_field_name("key").unwrap())?,
            val: Expr::make(text, &node.child_by_field_name("value").unwrap())?,
        })
    }

    fn range(&self) -> Range {
        self.range
    }
}

impl Node for Id {
    fn make(text: &str, node: &tree_sitter::Node) -> Result<Self, im::Vector<Diagnostic>> {
        Ok(Id {
            range: node.range(),
            name: String::from(node.utf8_text(text.as_bytes()).unwrap()), // we assume UTF-8
        })
    }

    fn range(&self) -> Range {
        self.range
    }
}

impl Node for Block {
    fn make(text: &str, node: &tree_sitter::Node) -> Result<Self, im::Vector<Diagnostic>> {
        Ok(Block {
            range: node.range(),
            stmts: make_children(text, node, "statement")?,
            expr: match node.child_by_field_name("expression") {
                Some(expr) => Some(Box::new(Expr::make(text, &expr)?)),
                None => None,
            },
        })
    }

    fn range(&self) -> Range {
        self.range
    }
}

impl Node for Call {
    fn make(text: &str, node: &tree_sitter::Node) -> Result<Self, im::Vector<Diagnostic>> {
        Ok(Call {
            range: node.range(),
            func: Box::new(Expr::make(
                text,
                &node.child_by_field_name("function").unwrap(),
            )?),
            arg: Box::new(Expr::make(
                text,
                &node.child_by_field_name("argument").unwrap(),
            )?),
        })
    }

    fn range(&self) -> Range {
        self.range
    }
}

impl Node for Func {
    fn make(text: &str, node: &tree_sitter::Node) -> Result<Self, im::Vector<Diagnostic>> {
        Ok(Func {
            range: node.range(),
            param: Id::make(text, &node.child_by_field_name("parameter").unwrap())?,
            body: Box::new(Expr::make(
                text,
                &node.child_by_field_name("body").unwrap(),
            )?),
        })
    }

    fn range(&self) -> Range {
        self.range
    }
}

impl Node for Index {
    fn make(text: &str, node: &tree_sitter::Node) -> Result<Self, im::Vector<Diagnostic>> {
        Ok(Index {
            range: node.range(),
            coll: Box::new(Expr::make(
                text,
                &node.child_by_field_name("collection").unwrap(),
            )?),
            key: Box::new(Expr::make(text, &node.child_by_field_name("key").unwrap())?),
        })
    }

    fn range(&self) -> Range {
        self.range
    }
}

impl Node for Field {
    fn make(text: &str, node: &tree_sitter::Node) -> Result<Self, im::Vector<Diagnostic>> {
        Ok(Field {
            range: node.range(),
            map: Box::new(Expr::make(text, &node.child_by_field_name("map").unwrap())?),
            key: Sym::make(text, &node.child_by_field_name("key").unwrap())?,
        })
    }

    fn range(&self) -> Range {
        self.range
    }
}
