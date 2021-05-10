#![allow(dead_code)]
#![allow(non_snake_case)]

use either::Either;
use serde::{Serialize, Serializer};

// https://github.com/estree/estree/blob/14df8a024956ea289bd55b9c2226a1d5b8a473ee/es5.md
// https://github.com/estree/estree/blob/70d58d73f69a3a72b51ed3fb540fae2550160619/es2015.md

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

    #[serde(rename = "Literal")]
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
    pub sourceType: SourceType,
    #[serde(serialize_with = "serialize_vec_either_untagged")]
    pub body: Vec<Either<Statement, ModuleDeclaration>>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub enum SourceType {
    #[serde(rename = "script")]
    Script,

    #[serde(rename = "module")]
    Module,
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
        left: Either<VariableDeclaration, Box<Pattern>>,
        right: Box<Expression>,
        body: Box<Statement>,
    },

    #[serde(rename = "ForOfStatement")]
    ForOf {
        #[serde(with = "either::serde_untagged")]
        left: Either<VariableDeclaration, Box<Pattern>>,
        right: Box<Expression>,
        body: Box<Statement>,
    },

    FunctionDeclaration {
        id: Identifier,
        params: Vec<Pattern>,
        body: FunctionBody,
        generator: bool,
    },

    VariableDeclaration {
        declarations: Vec<VariableDeclarator>,
        kind: DeclarationKind,
    },

    ClassDeclaration {
        id: Identifier,
        superClass: Option<Box<Expression>>,
        body: ClassBody,
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
    pub param: Box<Pattern>,
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

    #[serde(rename = "let")]
    Let,

    #[serde(rename = "const")]
    Const,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(tag = "type")]
pub struct VariableDeclarator {
    pub id: Box<Pattern>,
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

    #[serde(rename = "Literal")]
    RegExpLiteral {
        regex: RegExp,
    },

    #[serde(rename = "ThisExpression")]
    This,

    #[serde(rename = "ArrayExpression")]
    Array {
        #[serde(serialize_with = "serialize_vec_either_option_untagged")]
        elements: Vec<Option<Either<Expression, SpreadElement>>>,
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
        generator: bool,
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
        left: Box<Pattern>,
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
        #[serde(with = "either::serde_untagged")]
        object: Either<Box<Expression>, Super>,
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
        #[serde(with = "either::serde_untagged")]
        callee: Either<Box<Expression>, Super>,
        #[serde(serialize_with = "serialize_vec_either_untagged")]
        arguments: Vec<Either<Expression, SpreadElement>>,
    },

    #[serde(rename = "NewExpression")]
    New {
        callee: Box<Expression>,
        #[serde(serialize_with = "serialize_vec_either_untagged")]
        arguments: Vec<Either<Expression, SpreadElement>>,
    },

    #[serde(rename = "SequenceExpression")]
    Sequence {
        expressions: Vec<Expression>,
    },

    #[serde(rename = "ArrowFunctionExpression")]
    ArrowFunction {
        id: Option<Identifier>,
        params: Vec<Pattern>,
        #[serde(with = "either::serde_untagged")]
        body: Either<FunctionBody, Box<Expression>>,
        generator: bool,
        expression: bool,
    },

    #[serde(rename = "YieldExpression")]
    Yield {
        argument: Option<Box<Expression>>,
        delegate: bool,
    },

    TemplateLiteral {
        quasis: Vec<TemplateElement>,
        expressions: Vec<Expression>,
    },

    #[serde(rename = "TaggedTemplateExpression")]
    TaggedTemplate {
        tag: Box<Expression>,
        quasi: TemplateLiteral,
    },

    #[serde(rename = "ClassExpression")]
    Class {
        id: Option<Identifier>,
        superClass: Option<Box<Expression>>,
        body: ClassBody,
    },

    MetaProperty {
        meta: Identifier,
        property: Identifier,
    },
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(tag = "type")]
pub struct Property {
    pub key: Box<Expression>,
    pub value: Box<Expression>,
    pub kind: PropertyKind,
    pub method: bool,
    pub shorthand: bool,
    pub computed: bool,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(rename = "Property", tag = "type")]
pub struct AssignmentProperty {
    pub key: Box<Expression>,
    pub value: Box<Pattern>,
    pub kind: PropertyKind, // always "init"
    pub method: bool,       // always false
    pub shorthand: bool,
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
#[serde(tag = "type")]
pub struct Super {}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(tag = "type")]
pub struct SpreadElement {
    pub argument: Box<Expression>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(tag = "type")]
pub struct TemplateLiteral {
    pub quasis: Vec<TemplateElement>,
    pub expressions: Vec<Expression>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(tag = "type")]
pub struct TemplateElement {
    pub tail: bool,
    pub value: TemplateValue,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct TemplateValue {
    pub cooked: String,
    pub raw: String,
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

    #[serde(rename = "ObjectPattern")]
    Object {
        properties: Vec<AssignmentProperty>,
    },

    #[serde(rename = "ArrayPattern")]
    Array {
        properties: Vec<Option<Pattern>>,
    },

    RestElement {
        argument: Box<Pattern>,
    },

    #[serde(rename = "AssignmentPattern")]
    Assignment {
        left: Box<Pattern>,
        right: Box<Expression>,
    },
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(tag = "type")]
pub struct ClassBody {
    pub body: Vec<MethodDefinition>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(tag = "type")]
pub struct MethodDefinition {
    pub key: Box<Expression>,
    pub value: FunctionExpression,
    pub kind: MethodKind,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(tag = "type")]
pub struct FunctionExpression {
    pub id: Option<Identifier>,
    pub params: Vec<Pattern>,
    pub body: FunctionBody,
    pub generator: bool,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub enum MethodKind {
    #[serde(rename = "constructor")]
    Constructor,

    #[serde(rename = "method")]
    Method,

    #[serde(rename = "get")]
    Get,

    #[serde(rename = "set")]
    Set,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(tag = "type")]
pub enum ModuleDeclaration {
    #[serde(rename = "ImportDeclaration")]
    Import {
        specifiers: Vec<ImportSpecifier>,
        source: Literal,
    },

    #[serde(rename = "ExportNamedDeclaration")]
    ExportNamed {
        declaration: Option<Declaration>,
        specifiers: Vec<ExportSpecifier>,
        source: Option<Literal>,
    },

    #[serde(rename = "ExportDefaultDeclaration")]
    ExportDefault {
        declaration: ExportDefaultDeclaration,
    },

    #[serde(rename = "ExportDefaultDeclaration")]
    ExportDefaultExpression { declaration: Expression },

    #[serde(rename = "ExportAllDeclaration")]
    ExportAll { source: Literal },
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(tag = "type")]
pub enum Declaration {
    #[serde(rename = "FunctionDeclaration")]
    Function {
        id: Identifier,
        params: Vec<Pattern>,
        body: FunctionBody,
        generator: bool,
    },

    #[serde(rename = "VariableDeclaration")]
    Variable {
        declarations: Vec<VariableDeclarator>,
        kind: DeclarationKind,
    },

    #[serde(rename = "ClassDeclaration")]
    Class {
        id: Identifier,
        superClass: Option<Box<Expression>>,
        body: ClassBody,
    },
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(tag = "type")]
pub enum ImportSpecifier {
    #[serde(rename = "ImportSpecifier")]
    Import {
        local: Identifier,
        imported: Identifier,
    },

    #[serde(rename = "ImportDefaultSpecifier")]
    ImportDefault { local: Identifier },

    #[serde(rename = "ImportNamespaceSpecifier")]
    ImportNamespace { local: Identifier },
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(tag = "type")]
pub struct ExportSpecifier {
    pub local: Identifier,
    pub exported: Identifier,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(tag = "type")]
pub enum ExportDefaultDeclaration {
    #[serde(rename = "AnonymousDefaultExportedFunctionDeclaration")]
    AnonymousDefaultExportedFunction {
        id: Option<Identifier>, // always null
        params: Vec<Pattern>,
        body: FunctionBody,
        generator: bool,
    },

    #[serde(rename = "FunctionDeclaration")]
    Function {
        id: Identifier,
        params: Vec<Pattern>,
        body: FunctionBody,
        generator: bool,
    },

    #[serde(rename = "AnonymousDefaultExportedClassDeclaration")]
    AnonymousDefaultExportedClass {
        id: Option<Identifier>, // always null
        superClass: Option<Box<Expression>>,
        body: ClassBody,
    },

    #[serde(rename = "ClassDeclaration")]
    Class {
        id: Identifier,
        superClass: Option<Box<Expression>>,
        body: ClassBody,
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
        .map(|x| match x {
            Either::Left(ref left) => UntaggedEither::Left(left),
            Either::Right(ref right) => UntaggedEither::Right(right),
        })
        .collect();
    untagged.serialize(serializer)
}

fn serialize_vec_either_option_untagged<L, R, S>(
    this: &Vec<Option<Either<L, R>>>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
    L: Serialize,
    R: Serialize,
{
    let untagged: Vec<_> = this
        .iter()
        .map(|x| match x {
            None => None,
            Some(Either::Left(ref left)) => Some(UntaggedEither::Left(left)),
            Some(Either::Right(ref right)) => Some(UntaggedEither::Right(right)),
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
            "type": "Literal",
            "regex": {"pattern": "foo", "flags": "yu"},
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
            generator: false,
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
            "generator": false,
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
                    id: Box::new(Pattern::Identifier {
                        name: String::from("x"),
                    }),
                    init: None,
                },
                VariableDeclarator {
                    id: Box::new(Pattern::Identifier {
                        name: String::from("y"),
                    }),
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
    fn test_array_sparse() {
        let ast = Expression::Array {
            elements: vec![
                Some(Either::Left(Expression::Literal {
                    value: Value::Number(1.0),
                })),
                None,
                Some(Either::Left(Expression::Literal {
                    value: Value::Number(2.0),
                })),
            ],
        };
        let expected = serde_json::json!({
            "type": "ArrayExpression",
            "elements": [
                {"type": "Literal", "value": 1.0},
                null,
                {"type": "Literal", "value": 2.0},
            ],
        });
        assert_eq!(serde_json::to_value(ast).unwrap(), expected)
    }

    #[test]
    fn test_array_spread() {
        let ast = Expression::Array {
            elements: vec![
                Some(Either::Left(Expression::Identifier {
                    name: String::from("head"),
                })),
                Some(Either::Right(SpreadElement {
                    argument: Box::new(Expression::Identifier {
                        name: String::from("iter"),
                    }),
                })),
                Some(Either::Left(Expression::Identifier {
                    name: String::from("tail"),
                })),
            ],
        };
        let expected = serde_json::json!({
            "type": "ArrayExpression",
            "elements": [
                {"type": "Identifier", "name": "head"},
                {"type": "SpreadElement", "argument": {"type": "Identifier", "name": "iter"}},
                {"type": "Identifier", "name": "tail"},
            ],
        });
        assert_eq!(serde_json::to_value(ast).unwrap(), expected)
    }

    #[test]
    fn test_assign_object() {
        let ast = Expression::Assignment {
            operator: AssignmentOperator::Equal,
            left: Box::new(Pattern::Identifier {
                name: String::from("x"),
            }),
            right: Box::new(Expression::Object {
                properties: vec![
                    Property {
                        key: Box::new(Expression::Identifier {
                            name: String::from("a"),
                        }),
                        value: Box::new(Expression::Literal {
                            value: Value::Number(1.0),
                        }),
                        kind: PropertyKind::Init,
                        method: false,
                        shorthand: false,
                        computed: false,
                    },
                    Property {
                        key: Box::new(Expression::Literal {
                            value: Value::String(String::from("b")),
                        }),
                        value: Box::new(Expression::Identifier {
                            name: String::from("y"),
                        }),
                        kind: PropertyKind::Init,
                        method: false,
                        shorthand: false,
                        computed: false,
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
                        "method": false,
                        "shorthand": false,
                        "computed": false,
                    },
                    {
                        "type": "Property",
                        "key": {"type": "Literal", "value": "b"},
                        "value": {"type": "Identifier", "name": "y"},
                        "kind": "init",
                        "method": false,
                        "shorthand": false,
                        "computed": false,
                    },
                ]
            }
        });
        assert_eq!(serde_json::to_value(ast).unwrap(), expected)
    }

    #[test]
    fn test_super() {
        let ast = Super {};
        let expected = serde_json::json!({"type": "Super"});
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
