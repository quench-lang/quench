use crate::{deps, estree};

pub struct Codegen {
    js_runtime: deno_core::JsRuntime,
}

impl Codegen {
    pub fn new() -> Self {
        let mut js_runtime = deno_core::JsRuntime::new(deno_core::RuntimeOptions::default());
        js_runtime
            .execute(deps::ASTRING, deps::ASTRING_SOURCE)
            .unwrap(); // this shouldn't fail since the Astring source is a compile-time constant
        Codegen { js_runtime }
    }

    pub fn gen(&mut self, ast: &estree::Program) -> Option<String> {
        let context = self.js_runtime.global_context();
        let scope = &mut rusty_v8::HandleScope::with_context(self.js_runtime.v8_isolate(), context);
        let context = scope.get_current_context();
        let global = context.global(scope);

        // these are also compile-time constants and thus should also all be guaranteed to be fine
        let key1 = rusty_v8::String::new(scope, "astring").unwrap();
        let key2 = rusty_v8::String::new(scope, "generate").unwrap();
        let generate = global
            .get(scope, key1.into())
            .unwrap()
            .to_object(scope)
            .unwrap()
            .get(scope, key2.into())
            .unwrap();
        assert!(generate.is_function());
        // should be safe since we just asserted that it's a function
        let generate_fn = unsafe { rusty_v8::Local::<rusty_v8::Function>::cast(generate) };

        serde_v8::to_v8(scope, ast)
            .ok()
            .and_then(|arg| generate_fn.call(scope, generate, &[arg]))
            .map(|result| result.to_rust_string_lossy(scope))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use either::Either;

    #[test]
    fn test_hello_world() {
        let ast = estree::Program {
            sourceType: estree::SourceType::Module,
            body: vec![Either::Left(estree::Statement::Expression {
                expression: Box::new(estree::Expression::Call {
                    callee: Either::Left(Box::new(estree::Expression::Member {
                        object: Either::Left(Box::new(estree::Expression::Identifier {
                            name: String::from("console"),
                        })),
                        property: Box::new(estree::Expression::Identifier {
                            name: String::from("log"),
                        }),
                        computed: false,
                    })),
                    arguments: vec![Either::Left(estree::Expression::Literal {
                        value: estree::Value::String(String::from("Hello, world!")),
                    })],
                }),
            })],
        };
        let mut codegen = Codegen::new();
        let code = codegen.gen(&ast).unwrap();
        assert_eq!(code, "console.log(\"Hello, world!\");\n");
    }
}
