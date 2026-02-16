mod arena;
mod ast;
mod common;
mod context;
mod lexer;

use crate::lexer::TokenKind;
use context::CompilerCtx;

fn main() {
    let mut l = lexer::Lexer::new(
        "val y = {
    val a = 6_000.9;
    val b = 8;
    a + b
}",
    );
    loop {
        let k = l.next_token().kind;
        println!("{:?}", k);
        if k == TokenKind::Eof {
            break;
        }
    }
}
