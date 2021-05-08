use lspower::lsp::DiagnosticSeverity;
use tree_sitter::Range;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Diagnostic {
    pub range: Range,
    pub severity: DiagnosticSeverity,
    pub message: String,
}
