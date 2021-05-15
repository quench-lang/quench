use crate::deps;
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
        let specifier_str = module_specifier.as_str();
        let result = if specifier_str == deps::IMMUTABLE {
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
        } else if specifier_str == self.main_module.as_str() {
            Ok(ModuleSource {
                code: self.main_source.clone(),
                module_url_specified: module_specifier.to_string(),
                module_url_found: self.main_module.to_string(),
            })
        } else {
            Err(LoadError {
                module_specifier: module_specifier.clone(),
            })
            .map_err(|e| anyhow::anyhow!(e))
        };
        async move { result }.boxed_local()
    }
}
