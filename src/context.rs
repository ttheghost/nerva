use crate::common::{SymbolInterner, Span, Symbol};
use crate::ast::Ast;

#[derive(Debug)]
pub enum DiagnosticLevel { Info, Warning, Error }

#[derive(Debug)]
pub struct Diagnostic {
    pub level: DiagnosticLevel,
    pub message: String,
    pub span: Span
}

pub struct CompilerCtx {
    pub symbol_interner: SymbolInterner,
    pub ast: Ast,

    pub diagnostics: Vec<Diagnostic>,

    pub target: String,

    pub current_pass: &'static str,
}

impl CompilerCtx {
    pub fn new(target: &str, arena_chunk_size: usize) -> Self {
        CompilerCtx {
            symbol_interner: SymbolInterner::new(),
            ast: Ast::new(arena_chunk_size),
            diagnostics: Vec::new(),
            target: target.to_string(),
            current_pass: "Init"
        }
    }

    pub fn report(&mut self, level: DiagnosticLevel, message: &str, span: Span) {
        self.diagnostics.push(Diagnostic { level, message: message.to_string(), span })
    }

    pub fn has_errors(&self) -> bool {
        self.diagnostics.iter().any(|d| matches!(d.level, DiagnosticLevel::Error))
    }
}