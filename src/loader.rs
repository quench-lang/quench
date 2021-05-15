use deno_core::{
    error::AnyError, ModuleLoader, ModuleSource, ModuleSourceFuture, ModuleSpecifier, OpState,
};
use futures::future::FutureExt;
use std::{cell::RefCell, pin::Pin, rc::Rc};
use url::Url;

#[derive(Debug, thiserror::Error)]
#[error("unrecognized module specifier {module_specifier}")]
pub struct LoadError {
    module_specifier: ModuleSpecifier,
}

pub struct FixedLoader {
    pub main_module: Url,
    pub main_source: String,
}

impl ModuleLoader for FixedLoader {
    fn resolve(
        &self,
        _op_state: Rc<RefCell<OpState>>,
        specifier: &str,
        referrer: &str,
        _is_main: bool,
    ) -> Result<ModuleSpecifier, AnyError> {
        Ok(deno_core::resolve_import(specifier, referrer)?)
    }

    fn load(
        &self,
        _op_state: Rc<RefCell<OpState>>,
        module_specifier: &ModuleSpecifier,
        _maybe_referrer: Option<ModuleSpecifier>,
        _is_dyn_import: bool,
    ) -> Pin<Box<ModuleSourceFuture>> {
        let main_module = self.main_module.clone();
        let main_source = self.main_source.clone(); // TODO
        let module_specifier = module_specifier.clone();
        async move {
            let specifier_str = module_specifier.as_str();
            if specifier_str == "https://deno.land/x/immutable@4.0.0-rc.12-deno/mod.ts" {
                Ok(ModuleSource {
                    code: include_str!("../jsdeps/node_modules/immutable/dist/immutable.es.js")
                        .to_string(),
                    module_url_specified: module_specifier.to_string(),
                    module_url_found: concat!(
                        "https://github.com/quench-lang/quench/raw/",
                        env!("VERGEN_GIT_SHA"),
                        "/jsdeps/node_modules/immutable/dist/immutable.es.js",
                    )
                    .to_string(),
                })
            } else if specifier_str == main_module.as_str() {
                Ok(ModuleSource {
                    code: main_source,
                    module_url_specified: module_specifier.to_string(),
                    module_url_found: main_module.to_string(),
                })
            } else {
                Err(LoadError { module_specifier })?
            }
        }
        .boxed_local()
    }
}
