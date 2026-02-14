#[derive(Debug, Clone, Copy)]
pub struct Span {
    pub start: u32,
    pub end: u32,
}

impl Span {
    pub fn new(start: u32, end: u32) -> Self {
        Self { start, end }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Symbol(pub u32);

pub struct Interner {
    strings: Vec<String>,
}

impl Interner {
    pub fn new() -> Self {
        Self { strings: vec![] }
    }

    pub fn intern(&mut self, s: &str) -> Symbol {
        if let Some(idx) = self.strings.iter().position(|r| r == s) {
            Symbol(idx as u32)
        } else {
            let idx = self.strings.len();
            self.strings.push(s.to_string());
            Symbol(idx as u32)
        }
    }
    
    pub fn resolve(&self, symbol: Symbol) -> &str {
        &self.strings[symbol.0 as usize]
    }
}