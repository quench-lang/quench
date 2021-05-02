use crate::{estree, syntax};
use either::Either;

fn compile_identifier(id: &syntax::Identifier) -> Option<estree::Expression> {
    match id.name.as_str() {
        "print" => Some(estree::Expression::Member {
            object: Box::new(estree::Expression::Identifier {
                name: String::from("console"),
            }),
            property: Box::new(estree::Expression::Identifier {
                name: String::from("log"),
            }),
            computed: false,
        }),
        "args" => Some(estree::Expression::Member {
            object: Box::new(estree::Expression::Identifier {
                name: String::from("Deno"),
            }),
            property: Box::new(estree::Expression::Identifier {
                name: String::from("args"),
            }),
            computed: false,
        }),
        _ => None,
    }
}

fn compile_expression(expr: &syntax::Expression) -> Option<estree::Expression> {
    match expr {
        syntax::Expression::Call(syntax::Call {
            function,
            arguments,
        }) => Some(estree::Expression::Call {
            callee: Box::new(compile_identifier(function)?),
            arguments: arguments.iter().filter_map(compile_expression).collect(),
        }),
        syntax::Expression::Id(id) => compile_identifier(id),
        syntax::Expression::Lit(syntax::Literal::Str(value)) => Some(estree::Expression::Literal {
            value: estree::Value::String(value.clone()),
        }),
    }
}

fn compile_statement(stmt: &syntax::Statement) -> Option<estree::Statement> {
    match stmt {
        syntax::Statement::Expr(expr) => {
            let compiled = compile_expression(expr)?;
            Some(estree::Statement::Expression {
                expression: Box::new(compiled),
            })
        }
    }
}

pub fn compile_file(file: &syntax::File) -> Option<estree::Program> {
    Some(estree::Program {
        body: file
            .body
            .iter()
            .filter_map(compile_statement)
            .map(Either::Right)
            .collect(),
    })
}
