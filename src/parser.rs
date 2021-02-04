use tree_sitter::{Language, Parser};

extern "C" {
    fn tree_sitter_quench() -> Language;
}

pub fn parser() -> Parser {
    let language = unsafe { tree_sitter_quench() };
    let mut parser = Parser::new();
    // Err means we're using two incompatible versions of tree-sitter, meaning we built wrong
    parser.set_language(language).unwrap();
    parser
}
