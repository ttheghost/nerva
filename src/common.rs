use std::collections::HashMap;
use std::hash::Hash;

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
    map: HashMap<String, u32>
}

impl Interner {
    pub fn new() -> Self {
        Self { strings: vec![], map: HashMap::new() }
    }

    pub fn intern(&mut self, s: &str) -> Symbol {
        if let Some(idx) = self.map.get(s) {
            Symbol(*idx)
        } else {
            let idx = self.strings.len() as u32;
            self.strings.push(s.to_string());
            self.map.insert(s.to_string(), idx);
            Symbol(idx)
        }
    }
    
    pub fn resolve(&self, symbol: Symbol) -> &str {
        debug_assert!(symbol.0 < self.strings.len() as u32);
        &self.strings[symbol.0 as usize]
    }
}