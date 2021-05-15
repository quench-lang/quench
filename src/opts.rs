use crate::deps;

#[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
pub struct Opts {
    pub stdlib_placeholder: bool,
}

impl Opts {
    pub fn stdlib(&self) -> &str {
        if self.stdlib_placeholder {
            deps::STDLIB_PLACEHOLDER
        } else {
            deps::STDLIB
        }
    }
}
