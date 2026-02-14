use crate::common::{Interner, Span, Symbol};
use crate::ast::AstArena;

#[derive(Debug)]
pub enum DiagnosticLevel { Info, Warning, Error }

#[derive(Debug)]
pub struct Diagnostic {
    pub level: DiagnosticLevel,
    pub message: String,
    pub span: Span
}

pub struct CompilerCtx {
    pub interner: Interner,
    pub ast: AstArena,

    pub diagnostics: Vec<Diagnostic>,

    pub target: String,

    pub current_pass: &'static str,
}

impl CompilerCtx {
    pub fn new(target: &str) -> Self {
        CompilerCtx {
            interner: Interner::new(),
            ast: AstArena::new(),
            diagnostics: Vec::new(),
            target: target.to_string(),
            current_pass: "Init"
        }
    }

    pub fn report(&mut self, level: DiagnosticLevel, message: &str, span: Span) {
        self.diagnostics.push(Diagnostic { level, message: message.to_string(), span })
    }

    pub fn intern(&mut self, s: &str) -> Symbol {
        self.interner.intern(s)
    }

    pub fn has_errors(&self) -> bool {
        self.diagnostics.iter().any(|d| matches!(d.level, DiagnosticLevel::Error))
    }
}