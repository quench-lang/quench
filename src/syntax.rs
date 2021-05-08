use crate::diagnosis::Diagnostic;
use lspower::lsp::DiagnosticSeverity;
use std::fmt::Debug;
use tree_sitter::Range;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct File {
    pub range: Range,
    pub body: Vec<Stmt>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Stmt {
    pub range: Range,
    pub expression: Expr,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Expr {
    Lit(Lit),
    Id(Id),
    Call(Call),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Lit {
    Str(Str),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Str {
    pub range: Range,
    pub value: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Id {
    pub range: Range,
    pub name: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Call {
    pub range: Range,
    pub function: Id,
    pub arguments: Vec<Expr>,
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
            body: make_children(text, node, "body")?,
        })
    }

    fn range(&self) -> Range {
        self.range
    }
}

impl Node for Stmt {
    fn make(text: &str, node: &tree_sitter::Node) -> Result<Self, im::Vector<Diagnostic>> {
        let expression = node.child_by_field_name("expression").unwrap();
        Ok(Stmt {
            range: node.range(),
            expression: Expr::make(text, &expression)?,
        })
    }

    fn range(&self) -> Range {
        self.range
    }
}

impl Node for Expr {
    fn make(text: &str, node: &tree_sitter::Node) -> Result<Self, im::Vector<Diagnostic>> {
        match node.kind() {
            "string" => Lit::make(text, node).map(Expr::Lit),
            "identifier" => Id::make(text, node).map(Expr::Id),
            "call" => Call::make(text, node).map(Expr::Call),
            kind => Err(im::vector![Diagnostic {
                range: node.range(),
                severity: DiagnosticSeverity::Error,
                message: format!("did not expect a node of kind {}", kind),
            }]),
        }
    }

    fn range(&self) -> Range {
        match self {
            Expr::Lit(x) => x.range(),
            Expr::Id(x) => x.range(),
            Expr::Call(x) => x.range(),
        }
    }
}

impl Node for Lit {
    fn make(text: &str, node: &tree_sitter::Node) -> Result<Self, im::Vector<Diagnostic>> {
        Str::make(text, node).map(Lit::Str)
    }

    fn range(&self) -> Range {
        match self {
            Lit::Str(x) => x.range(),
        }
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
            value: String::from(value),
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

impl Node for Call {
    fn make(text: &str, node: &tree_sitter::Node) -> Result<Self, im::Vector<Diagnostic>> {
        let function = node.child_by_field_name("function").unwrap();
        let arguments = node.child_by_field_name("arguments").unwrap();
        Ok(Call {
            range: node.range(),
            function: Id::make(text, &function)?,
            arguments: make_children(text, &arguments, "argument")?,
        })
    }

    fn range(&self) -> Range {
        self.range
    }
}
