use crate::{diagnosis::Diagnostic, estree as js, opts::Opts, syntax as qn};
use either::Either;
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

const MAIN: &str = "main";

fn mangle(name: &str) -> String {
    format!("${}", name)
}

pub fn compile_file(file: &qn::File, opts: &Opts) -> Result<js::Program, im::Vector<Diagnostic>> {
    let mut body = vec![Either::Right(js::ModuleDeclaration::Import {
        specifiers: vec![js::ImportSpecifier::ImportDefault {
            local: js::Identifier {
                name: String::from("Immutable"),
            },
        }],
        source: js::Literal::Literal {
            value: js::Value::String(String::from(opts.stdlib())),
        },
    })];
    body.extend(
        gather(compile_declaration, file.decls.iter())?
            .into_iter()
            .map(Either::Left),
    );
    if file.decls.iter().any(|decl| decl.name.name == MAIN) {
        body.push(Either::Left(js::Statement::Expression {
            expression: Box::new(js::Expression::Call {
                callee: Either::Left(Box::new(js::Expression::Identifier { name: mangle(MAIN) })),
                arguments: vec![],
            }),
        }));
    }
    Ok(js::Program {
        sourceType: js::SourceType::Module,
        body,
    })
}

fn compile_declaration(decl: &qn::Decl) -> Result<js::Statement, im::Vector<Diagnostic>> {
    Ok(js::Statement::VariableDeclaration {
        declarations: vec![js::VariableDeclarator {
            id: Box::new(js::Pattern::Identifier {
                name: mangle(&decl.name.name),
            }),
            init: Some(Box::new(compile_expression(&decl.val)?)),
        }],
        kind: js::DeclarationKind::Const,
    })
}

fn compile_statement(stmt: &qn::Stmt) -> Result<js::Statement, im::Vector<Diagnostic>> {
    match stmt {
        qn::Stmt::Decl(decl) => compile_declaration(decl),
        qn::Stmt::ExprStmt(qn::ExprStmt { expr, .. }) => Ok(js::Statement::Expression {
            expression: Box::new(compile_expression(expr)?),
        }),
    }
}

fn compile_expression(expr: &qn::Expr) -> Result<js::Expression, im::Vector<Diagnostic>> {
    match expr {
        qn::Expr::Lit(lit) => compile_literal(lit),
        qn::Expr::Id(id) => compile_identifier(id),
        qn::Expr::Block(block) => compile_block(block),
        qn::Expr::Call(qn::Call { func, arg, .. }) => Ok(js::Expression::Call {
            callee: Either::Left(Box::new(compile_expression(func)?)),
            arguments: vec![Either::Left(compile_expression(arg)?)],
        }),
        qn::Expr::Func(qn::Func { param, body, .. }) => Ok(js::Expression::ArrowFunction {
            id: None,
            params: vec![js::Pattern::Identifier {
                name: mangle(&param.name),
            }],
            body: Either::Right(Box::new(compile_expression(body)?)),
            generator: false,
            expression: true,
        }),
        qn::Expr::Index(qn::Index { coll, key, .. }) => Ok(js::Expression::Call {
            callee: Either::Left(Box::new(js::Expression::Member {
                object: Either::Left(Box::new(compile_expression(coll)?)),
                property: Box::new(js::Expression::Identifier {
                    name: String::from("get"),
                }),
                computed: false,
            })),
            arguments: vec![Either::Left(compile_expression(key)?)],
        }),
        qn::Expr::Field(qn::Field { map, key, .. }) => Ok(js::Expression::Call {
            callee: Either::Left(Box::new(js::Expression::Member {
                object: Either::Left(Box::new(compile_expression(map)?)),
                property: Box::new(js::Expression::Identifier {
                    name: String::from("get"),
                }),
                computed: false,
            })),
            arguments: vec![Either::Left(compile_symbol(key)?)],
        }),
    }
}

fn compile_literal(lit: &qn::Lit) -> Result<js::Expression, im::Vector<Diagnostic>> {
    match lit {
        qn::Lit::Null { .. } => Ok(js::Expression::Literal {
            value: js::Value::Null,
        }),
        qn::Lit::Bool(qn::Bool { val, .. }) => Ok(js::Expression::Literal {
            value: js::Value::Boolean(*val),
        }),
        qn::Lit::Int(qn::Int { val, .. }) => Ok(js::Expression::Literal {
            value: js::Value::Number(f64::from(*val)),
        }),
        qn::Lit::Str(qn::Str { val, .. }) => Ok(js::Expression::Literal {
            value: js::Value::String(val.clone()),
        }),
        qn::Lit::Sym(sym) => compile_symbol(sym),
        qn::Lit::List(qn::List { items, .. }) => Ok(js::Expression::Call {
            callee: Either::Left(Box::new(js::Expression::Member {
                object: Either::Left(Box::new(js::Expression::Identifier {
                    name: String::from("Immutable"),
                })),
                property: Box::new(js::Expression::Identifier {
                    name: String::from("List"),
                }),
                computed: false,
            })),
            arguments: vec![Either::Left(js::Expression::Array {
                elements: gather(compile_expression, items.iter())?
                    .into_iter()
                    .map(Either::Left)
                    .map(Some)
                    .collect(),
            })],
        }),
        qn::Lit::Map(qn::Map { entries, .. }) => Ok(js::Expression::Call {
            callee: Either::Left(Box::new(js::Expression::Member {
                object: Either::Left(Box::new(js::Expression::Identifier {
                    name: String::from("Immutable"),
                })),
                property: Box::new(js::Expression::Identifier {
                    name: String::from("Map"),
                }),
                computed: false,
            })),
            arguments: vec![Either::Left(js::Expression::Array {
                elements: gather(compile_entry, entries.iter())?
                    .into_iter()
                    .map(Either::Left)
                    .map(Some)
                    .collect(),
            })],
        }),
    }
}

fn compile_symbol(sym: &qn::Sym) -> Result<js::Expression, im::Vector<Diagnostic>> {
    Ok(js::Expression::Call {
        callee: Either::Left(Box::new(js::Expression::Member {
            object: Either::Left(Box::new(js::Expression::Identifier {
                name: String::from("Symbol"),
            })),
            property: Box::new(js::Expression::Identifier {
                name: String::from("for"),
            }),
            computed: false,
        })),
        arguments: vec![Either::Left(js::Expression::Literal {
            value: js::Value::String(sym.name.clone()),
        })],
    })
}

fn compile_entry(entry: &qn::Entry) -> Result<js::Expression, im::Vector<Diagnostic>> {
    Ok(js::Expression::Array {
        elements: vec![
            Some(Either::Left(compile_expression(&entry.key)?)),
            Some(Either::Left(compile_expression(&entry.val)?)),
        ],
    })
}

fn compile_identifier(id: &qn::Id) -> Result<js::Expression, im::Vector<Diagnostic>> {
    match id.name.as_str() {
        "print" => Ok(js::Expression::Member {
            object: Either::Left(Box::new(js::Expression::Identifier {
                name: String::from("console"),
            })),
            property: Box::new(js::Expression::Identifier {
                name: String::from("log"),
            }),
            computed: false,
        }),
        "args" => Ok(js::Expression::Member {
            object: Either::Left(Box::new(js::Expression::Identifier {
                name: String::from("Deno"),
            })),
            property: Box::new(js::Expression::Identifier {
                name: String::from("args"),
            }),
            computed: false,
        }),
        name => Ok(js::Expression::Identifier { name: mangle(name) }),
    }
}

fn compile_block(block: &qn::Block) -> Result<js::Expression, im::Vector<Diagnostic>> {
    let qn::Block { stmts, expr, .. } = block;
    Ok(js::Expression::Call {
        callee: Either::Left(Box::new(js::Expression::ArrowFunction {
            id: None,
            params: vec![],
            body: Either::Left(js::FunctionBody {
                body: {
                    let mut body = gather(compile_statement, stmts.iter())?;
                    if let Some(expr) = expr {
                        body.push(js::Statement::Return {
                            argument: Some(Box::new(compile_expression(expr)?)),
                        });
                    }
                    body.into_iter().map(Either::Right).collect()
                },
            }),
            generator: false,
            expression: false,
        })),
        arguments: vec![],
    })
}
