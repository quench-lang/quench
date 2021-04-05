use either::Either;
use serde::{Serialize, Serializer};

// https://github.com/estree/estree/blob/0fa6c005fa452f1f970b3923d5faa38178906d08/es5.md

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(tag = "type")]
pub struct Identifier {
    pub name: String,
}

#[derive(Clone, Debug, Serialize)]
#[serde(untagged)]
pub enum Value {
    String(String),

    Boolean(bool),

    Null,

    Number(f64),
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (&self, &other) {
            (&Value::String(a), &Value::String(b)) => a == b,
            (&Value::Boolean(a), &Value::Boolean(b)) => a == b,
            (&Value::Null, &Value::Null) => true,
            // this should only be used for Salsa purposes, so we just compare representations
            (&Value::Number(a), &Value::Number(b)) => a.to_ne_bytes() == b.to_ne_bytes(),
            _ => false,
        }
    }
}

impl Eq for Value {}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(tag = "type")]
pub enum Literal {
    Literal {
        value: Value,
    },

    #[serde(rename = "RegExpLiteral")]
    RegExp {
        regex: RegExp,
    },
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct RegExp {
    pub pattern: String,
    pub flags: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(tag = "type")]
pub struct Program {
    #[serde(serialize_with = "serialize_vec_either_untagged")]
    pub body: Vec<Either<Directive, Statement>>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(tag = "type")]
pub enum Statement {
    #[serde(rename = "ExpressionStatement")]
    Expression { expression: Box<Expression> },

    #[serde(rename = "BlockStatement")]
    Block { body: Vec<Statement> },

    #[serde(rename = "EmptyStatement")]
    Empty,

    #[serde(rename = "DebuggerStatement")]
    Debugger,

    #[serde(rename = "WIthStatement")]
    WIth {
        object: Box<Expression>,
        body: Box<Statement>,
    },

    #[serde(rename = "ReturnStatement")]
    Return { argument: Option<Box<Expression>> },

    #[serde(rename = "LabeledStatement")]
    Labeled {
        label: Identifier,
        body: Box<Statement>,
    },

    #[serde(rename = "BreakStatement")]
    Break { label: Option<Identifier> },

    #[serde(rename = "ContinueStatement")]
    Continue { label: Option<Identifier> },

    #[serde(rename = "IfStatement")]
    If {
        test: Box<Expression>,
        consequent: Box<Statement>,
        alternate: Option<Box<Statement>>,
    },

    #[serde(rename = "SwitchStatement")]
    Switch {
        discriminant: Box<Expression>,
        cases: Vec<SwitchCase>,
    },

    #[serde(rename = "ThrowStatement")]
    Throw { argument: Box<Expression> },

    #[serde(rename = "TryStatement")]
    Try {
        block: BlockStatement,
        handler: Option<CatchClause>,
        finalizer: Option<BlockStatement>,
    },

    #[serde(rename = "WhileStatement")]
    While {
        test: Box<Expression>,
        body: Box<Statement>,
    },

    #[serde(rename = "DoWhileStatement")]
    DoWhile {
        body: Box<Statement>,
        test: Box<Expression>,
    },

    #[serde(rename = "ForStatement")]
    For {
        #[serde(with = "either::serde_untagged_optional")]
        init: Option<Either<VariableDeclaration, Box<Expression>>>,
        test: Option<Box<Expression>>,
        update: Option<Box<Expression>>,
        body: Box<Statement>,
    },

    #[serde(rename = "ForInStatement")]
    ForIn {
        #[serde(with = "either::serde_untagged")]
        left: Either<VariableDeclaration, Pattern>,
        right: Box<Expression>,
        body: Box<Statement>,
    },

    FunctionDeclaration {
        id: Identifier,
        params: Vec<Pattern>,
        body: FunctionBody,
    },

    VariableDeclaration {
        declarations: Vec<VariableDeclarator>,
        kind: DeclarationKind,
    },
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(rename = "ExpressionStatement", tag = "type")]
pub struct Directive {
    pub expression: Literal,
    pub directive: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(tag = "type")]
pub struct BlockStatement {
    pub body: Vec<Statement>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(rename = "BlockStatement", tag = "type")]
pub struct FunctionBody {
    #[serde(serialize_with = "serialize_vec_either_untagged")]
    pub body: Vec<Either<Directive, Statement>>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(tag = "type")]
pub struct SwitchCase {
    pub test: Option<Box<Expression>>,
    pub consequent: Vec<Statement>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(tag = "type")]
pub struct CatchClause {
    pub param: Pattern,
    pub body: BlockStatement,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(tag = "type")]
pub struct VariableDeclaration {
    pub declarations: Vec<VariableDeclarator>,
    pub kind: DeclarationKind,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub enum DeclarationKind {
    #[serde(rename = "var")]
    Var,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(tag = "type")]
pub struct VariableDeclarator {
    pub id: Pattern,
    pub init: Option<Box<Expression>>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(tag = "type")]
pub enum Expression {
    Identifier {
        name: String,
    },

    Literal {
        value: Value,
    },

    RegExpLiteral {
        regex: RegExp,
    },

    #[serde(rename = "ThisExpression")]
    This,

    #[serde(rename = "ArrayExpression")]
    Array {
        elements: Vec<Option<Expression>>,
    },

    #[serde(rename = "ObjectExpression")]
    Object {
        properties: Vec<Property>,
    },

    #[serde(rename = "FunctionExpression")]
    Function {
        id: Option<Identifier>,
        params: Vec<Pattern>,
        body: FunctionBody,
    },

    #[serde(rename = "UnaryExpression")]
    Unary {
        operator: UnaryOperator,
        prefix: bool,
        argument: Box<Expression>,
    },

    #[serde(rename = "UpdateExpression")]
    Update {
        operator: UpdateOperator,
        argument: Box<Expression>,
        prefix: bool,
    },

    #[serde(rename = "BinaryExpression")]
    Binary {
        operator: BinaryOperator,
        left: Box<Expression>,
        right: Box<Expression>,
    },

    #[serde(rename = "AssignmentExpression")]
    Assignment {
        operator: AssignmentOperator,
        #[serde(with = "either::serde_untagged")]
        left: Either<Pattern, Box<Expression>>,
        right: Box<Expression>,
    },

    #[serde(rename = "LogicalExpression")]
    Logical {
        operator: LogicalOperator,
        left: Box<Expression>,
        right: Box<Expression>,
    },

    #[serde(rename = "MemberExpression")]
    Member {
        object: Box<Expression>,
        property: Box<Expression>,
        computed: bool,
    },

    #[serde(rename = "ConditionalExpression")]
    Conditional {
        test: Box<Expression>,
        alternate: Box<Expression>,
        consequent: Box<Expression>,
    },

    #[serde(rename = "CallExpression")]
    Call {
        callee: Box<Expression>,
        arguments: Vec<Expression>,
    },

    #[serde(rename = "NewExpression")]
    New {
        callee: Box<Expression>,
        arguments: Vec<Expression>,
    },

    #[serde(rename = "SequenceExpression")]
    Sequence {
        expressions: Vec<Expression>,
    },
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(tag = "type")]
pub struct Property {
    #[serde(with = "either::serde_untagged")]
    pub key: Either<Literal, Identifier>,
    pub value: Box<Expression>,
    pub kind: PropertyKind,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub enum PropertyKind {
    #[serde(rename = "init")]
    Init,

    #[serde(rename = "get")]
    Get,

    #[serde(rename = "set")]
    Set,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub enum UnaryOperator {
    #[serde(rename = "-")]
    Negative,

    #[serde(rename = "+")]
    Positive,

    #[serde(rename = "!")]
    Not,

    #[serde(rename = "~")]
    BitwiseNot,

    #[serde(rename = "typeof")]
    Typeof,

    #[serde(rename = "void")]
    Void,

    #[serde(rename = "delete")]
    Delete,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub enum UpdateOperator {
    #[serde(rename = "++")]
    Increment,

    #[serde(rename = "--")]
    Decrement,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub enum BinaryOperator {
    #[serde(rename = "==")]
    DoubleEqual,

    #[serde(rename = "!=")]
    NotDoubleEqual,

    #[serde(rename = "===")]
    TripleEqual,

    #[serde(rename = "!==")]
    NotTripleEqual,

    #[serde(rename = "<")]
    Less,

    #[serde(rename = "<=")]
    LessEqual,

    #[serde(rename = ">")]
    Greater,

    #[serde(rename = ">=")]
    GreaterEqual,

    #[serde(rename = "<<")]
    LeftShift,

    #[serde(rename = ">>")]
    RightShift,

    #[serde(rename = ">>>")]
    UnsignedRightShift,

    #[serde(rename = "+")]
    Add,

    #[serde(rename = "-")]
    Subtract,

    #[serde(rename = "*")]
    Multiply,

    #[serde(rename = "/")]
    Divide,

    #[serde(rename = "%")]
    Modulus,

    #[serde(rename = "|")]
    BitwiseOr,

    #[serde(rename = "^")]
    BitwiseXor,

    #[serde(rename = "&")]
    BitwiseAnd,

    #[serde(rename = "in")]
    In,

    #[serde(rename = "instanceof")]
    Instanceof,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub enum AssignmentOperator {
    #[serde(rename = "=")]
    Equal,

    #[serde(rename = "+=")]
    AddEqual,

    #[serde(rename = "-=")]
    SubtractEqual,

    #[serde(rename = "*=")]
    MultiplyEqual,

    #[serde(rename = "/=")]
    DivideEqual,

    #[serde(rename = "%=")]
    ModulusEqual,

    #[serde(rename = "<<=")]
    LeftShiftEqual,

    #[serde(rename = ">>=")]
    RightShiftEqual,

    #[serde(rename = ">>>=")]
    UnsignedRightShiftEqual,

    #[serde(rename = "|=")]
    BitwiseOrEqual,

    #[serde(rename = "^=")]
    BitwiseXorEqual,

    #[serde(rename = "&=")]
    BitwiseAndEqual,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub enum LogicalOperator {
    #[serde(rename = "||")]
    Or,

    #[serde(rename = "&&")]
    And,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(tag = "type")]
pub enum Pattern {
    Identifier {
        name: String,
    },

    MemberExpression {
        object: Box<Expression>,
        property: Box<Expression>,
        computed: bool,
    },
}

// https://github.com/bluss/either/blob/1.6.1/src/serde_untagged_optional.rs

#[derive(Serialize)]
#[serde(untagged)]
enum UntaggedEither<L, R> {
    Left(L),
    Right(R),
}

fn serialize_vec_either_untagged<L, R, S>(
    this: &Vec<Either<L, R>>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
    L: Serialize,
    R: Serialize,
{
    let untagged: Vec<_> = this
        .iter()
        .map(|either| match either {
            Either::Left(ref left) => UntaggedEither::Left(left),
            Either::Right(ref right) => UntaggedEither::Right(right),
        })
        .collect();
    untagged.serialize(serializer)
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_null() {
        let ast = Literal::Literal { value: Value::Null };
        let expected = serde_json::json!({"type": "Literal", "value": null});
        assert_eq!(serde_json::to_value(ast).unwrap(), expected)
    }

    #[test]
    fn test_regex() {
        let ast = Literal::RegExp {
            regex: RegExp {
                pattern: String::from("foo"),
                flags: String::from("yu"),
            },
        };
        let expected = serde_json::json!({
            "type": "RegExpLiteral",
            "regex": {"pattern": "foo", "flags": "yu"},
        });
        assert_eq!(serde_json::to_value(ast).unwrap(), expected)
    }

    #[test]
    fn test_directive() {
        let ast = Program {
            body: vec![Either::Left(Directive {
                expression: Literal::Literal {
                    value: Value::String(String::from("use strict")),
                },
                directive: String::from("use strict"),
            })],
        };
        let expected = serde_json::json!({
            "type": "Program",
            "body": [{
                "type": "ExpressionStatement",
                "expression": {"type": "Literal", "value": "use strict"},
                "directive": "use strict",
            }],
        });
        assert_eq!(serde_json::to_value(ast).unwrap(), expected)
    }

    #[test]
    fn test_empty() {
        let ast = Statement::Empty;
        let expected = serde_json::json!({"type": "EmptyStatement"});
        assert_eq!(serde_json::to_value(ast).unwrap(), expected)
    }

    #[test]
    fn test_function_body() {
        let ast = Statement::FunctionDeclaration {
            id: Identifier {
                name: String::from("foo"),
            },
            params: vec![],
            body: FunctionBody {
                body: vec![Either::Left(Directive {
                    expression: Literal::Literal {
                        value: Value::String(String::from("use strict")),
                    },
                    directive: String::from("use strict"),
                })],
            },
        };
        let expected = serde_json::json!({
            "type": "FunctionDeclaration",
            "id": {"type": "Identifier", "name": "foo"},
            "params": [],
            "body": {
                "type": "BlockStatement",
                "body": [{
                    "type": "ExpressionStatement",
                    "expression": {"type": "Literal", "value": "use strict"},
                    "directive": "use strict",
                }],
            },
        });
        assert_eq!(serde_json::to_value(ast).unwrap(), expected)
    }

    #[test]
    fn test_switch() {
        let ast = Statement::Switch {
            discriminant: Box::new(Expression::Identifier {
                name: String::from("x"),
            }),
            cases: vec![
                SwitchCase {
                    test: Some(Box::new(Expression::Literal {
                        value: Value::Number(42.0),
                    })),
                    consequent: vec![],
                },
                SwitchCase {
                    test: None,
                    consequent: vec![],
                },
            ],
        };
        let expected = serde_json::json!({
            "type": "SwitchStatement",
            "discriminant": {"type": "Identifier", "name": "x"},
            "cases": [
                {
                    "type": "SwitchCase",
                    "test": {"type": "Literal", "value": 42.0},
                    "consequent": [],
                },
                {
                    "type": "SwitchCase",
                    "test": null,
                    "consequent": [],
                },
            ],
        });
        assert_eq!(serde_json::to_value(ast).unwrap(), expected)
    }

    #[test]
    fn test_variable_declaration() {
        let ast = VariableDeclaration {
            declarations: vec![
                VariableDeclarator {
                    id: Pattern::Identifier {
                        name: String::from("x"),
                    },
                    init: None,
                },
                VariableDeclarator {
                    id: Pattern::Identifier {
                        name: String::from("y"),
                    },
                    init: Some(Box::new(Expression::Literal {
                        value: Value::Number(42.0),
                    })),
                },
            ],
            kind: DeclarationKind::Var,
        };
        let expected = serde_json::json!({
            "type": "VariableDeclaration",
            "declarations": [
                {
                    "type": "VariableDeclarator",
                    "id": {"type": "Identifier", "name": "x"},
                    "init": null,
                },
                {
                    "type": "VariableDeclarator",
                    "id": {"type": "Identifier", "name": "y"},
                    "init": {"type": "Literal", "value": 42.0},
                },
            ],
            "kind": "var",
        });
        assert_eq!(serde_json::to_value(ast).unwrap(), expected)
    }

    #[test]
    fn test_assign_object() {
        let ast = Expression::Assignment {
            operator: AssignmentOperator::Equal,
            left: Either::Right(Box::new(Expression::Identifier {
                name: String::from("x"),
            })),
            right: Box::new(Expression::Object {
                properties: vec![
                    Property {
                        key: Either::Right(Identifier {
                            name: String::from("a"),
                        }),
                        value: Box::new(Expression::Literal {
                            value: Value::Number(1.0),
                        }),
                        kind: PropertyKind::Init,
                    },
                    Property {
                        key: Either::Left(Literal::Literal {
                            value: Value::String(String::from("b")),
                        }),
                        value: Box::new(Expression::Identifier {
                            name: String::from("y"),
                        }),
                        kind: PropertyKind::Init,
                    },
                ],
            }),
        };
        let expected = serde_json::json!({
            "type": "AssignmentExpression",
            "operator": "=",
            "left": {"type": "Identifier", "name": "x"},
            "right": {
                "type": "ObjectExpression",
                "properties": [
                    {
                        "type": "Property",
                        "key": {"type": "Identifier", "name": "a"},
                        "value": {"type": "Literal", "value": 1.0},
                        "kind": "init",
                    },
                    {
                        "type": "Property",
                        "key": {"type": "Literal", "value": "b"},
                        "value": {"type": "Identifier", "name": "y"},
                        "kind": "init",
                    },
                ]
            }
        });
        assert_eq!(serde_json::to_value(ast).unwrap(), expected)
    }

    #[test]
    fn test_binop() {
        let ast = Expression::Binary {
            operator: BinaryOperator::NotTripleEqual,
            left: Box::new(Expression::Literal {
                value: Value::Number(1.0),
            }),
            right: Box::new(Expression::Literal {
                value: Value::String(String::from("1")),
            }),
        };
        let expected = serde_json::json!({
            "type": "BinaryExpression",
            "operator": "!==",
            "left": {"type": "Literal", "value": 1.0},
            "right": {"type": "Literal", "value": "1"},
        });
        assert_eq!(serde_json::to_value(ast).unwrap(), expected)
    }
}
