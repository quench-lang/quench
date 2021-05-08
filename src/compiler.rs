use crate::{diagnosis::Diagnostic, estree, syntax};
use either::Either;
use lspower::lsp::DiagnosticSeverity;
use std::fmt::Debug;

fn gather<T, U: Debug>(
    f: fn(T) -> Result<U, im::Vector<Diagnostic>>,
    v: impl Iterator<Item = T>,
) -> Result<Vec<U>, im::Vector<Diagnostic>> {
    let (successes, diagnosticses): (Vec<_>, Vec<_>) = v.map(f).partition(Result::is_ok);
    if diagnosticses.is_empty() {
        Ok(successes.into_iter().map(Result::unwrap).collect())
    } else {
        let mut flattened = im::vector![];
        for diagnostics in diagnosticses.into_iter().map(Result::unwrap_err) {
            flattened.append(diagnostics);
        }
        Err(flattened)
    }
}

fn compile_identifier(id: &syntax::Id) -> Result<estree::Expression, im::Vector<Diagnostic>> {
    match id.name.as_str() {
        "print" => Ok(estree::Expression::Member {
            object: Box::new(estree::Expression::Identifier {
                name: String::from("console"),
            }),
            property: Box::new(estree::Expression::Identifier {
                name: String::from("log"),
            }),
            computed: false,
        }),
        "args" => Ok(estree::Expression::Member {
            object: Box::new(estree::Expression::Identifier {
                name: String::from("Deno"),
            }),
            property: Box::new(estree::Expression::Identifier {
                name: String::from("args"),
            }),
            computed: false,
        }),
        name => Err(im::vector![Diagnostic {
            range: id.range,
            severity: DiagnosticSeverity::Error,
            message: format!("unexpected identifier {}", name),
        }]),
    }
}

fn compile_expression(expr: &syntax::Expr) -> Result<estree::Expression, im::Vector<Diagnostic>> {
    match expr {
        syntax::Expr::Call(syntax::Call {
            function,
            arguments,
            ..
        }) => Ok(estree::Expression::Call {
            callee: Box::new(compile_identifier(function)?),
            arguments: gather(compile_expression, arguments.iter())?,
        }),
        syntax::Expr::Id(id) => compile_identifier(id),
        syntax::Expr::Lit(syntax::Lit::Str(syntax::Str { value, .. })) => {
            Ok(estree::Expression::Literal {
                value: estree::Value::String(value.clone()),
            })
        }
    }
}

fn compile_statement(stmt: &syntax::Stmt) -> Result<estree::Statement, im::Vector<Diagnostic>> {
    Ok(estree::Statement::Expression {
        expression: Box::new(compile_expression(&stmt.expression)?),
    })
}

pub fn compile_file(file: &syntax::File) -> Result<estree::Program, im::Vector<Diagnostic>> {
    Ok(estree::Program {
        body: gather(compile_statement, file.body.iter())?
            .into_iter()
            .map(Either::Right)
            .collect(),
    })
}
