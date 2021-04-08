#[derive(Clone, Debug, Eq, PartialEq)]
pub struct File {
    pub body: Vec<Statement>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Statement {
    Expr(Expression),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Expression {
    Lit(Literal),
    Id(Identifier),
    Call(Call),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Literal {
    Str(String),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Identifier {
    pub name: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Call {
    pub function: Identifier,
    pub arguments: Vec<Expression>,
}

pub trait Node {
    fn make(text: &str, node: &tree_sitter::Node) -> Option<Self>
    where
        Self: Sized;
}

impl Node for File {
    fn make(text: &str, node: &tree_sitter::Node) -> Option<Self> {
        let mut cursor = node.walk();
        Some(File {
            body: node
                .children(&mut cursor)
                .filter_map(|child| Statement::make(text, &child))
                .collect(),
        })
    }
}

impl Node for Statement {
    fn make(text: &str, node: &tree_sitter::Node) -> Option<Self> {
        Expression::make(text, node).map(Statement::Expr)
    }
}

impl Node for Expression {
    fn make(text: &str, node: &tree_sitter::Node) -> Option<Self> {
        match node.kind() {
            "string" => Literal::make(text, node).map(Expression::Lit),
            "identifier" => Identifier::make(text, node).map(Expression::Id),
            "call" => Call::make(text, node).map(Expression::Call),
            _ => None,
        }
    }
}

impl Node for Literal {
    fn make(text: &str, node: &tree_sitter::Node) -> Option<Self> {
        let value = node
            .utf8_text(text.as_bytes())
            .ok()?
            .strip_prefix("\"")?
            .strip_suffix("\"")?;
        Some(Literal::Str(String::from(value)))
    }
}

impl Node for Identifier {
    fn make(text: &str, node: &tree_sitter::Node) -> Option<Self> {
        Some(Identifier {
            name: String::from(node.utf8_text(text.as_bytes()).ok()?),
        })
    }
}

impl Node for Call {
    fn make(text: &str, node: &tree_sitter::Node) -> Option<Self> {
        Some(Call {
            function: Identifier::make(text, &node.child_by_field_name("function")?)?,
            arguments: {
                let args_child = node.child_by_field_name("arguments")?;
                let mut cursor = args_child.walk();
                args_child
                    .children(&mut cursor)
                    .filter_map(|child| Expression::make(text, &child))
                    .collect()
            },
        })
    }
}
