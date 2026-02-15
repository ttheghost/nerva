use crate::common::{Span, Symbol};

#[derive(Debug, Clone)]
pub enum TokenKind {
    Val, Var, Fn, If, Else, Return, True, False,

    LBrace, RBrace, LParen, RParen, LBracket, RBracket,
    SemiColon, Colon, Arrow,

    Plus, Minus, Star, Slash, Percent, Eq, EqEq, Pipeline,

    Int(i64),
    Float(f64),
    Ident(Symbol),
    EOF
}

#[derive(Debug, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
}

pub struct Lexer<'a> {
    input: &'a str,
    chars: std::iter::Peekable<std::str::Chars<'a>>,
    pos: u32,
    ctx: &'a mut crate::context::CompilerCtx, // We need context to intern strings
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str, ctx: &'a mut crate::context::CompilerCtx) -> Self {
        Self {
            input,
            chars: input.chars().peekable(),
            pos: 0,
            ctx,
        }
    }

    fn advance(&mut self) -> Option<char> {
        let c = self.chars.next()?;
        self.pos += 1;
        Some(c)
    }

    fn peek(&mut self) -> Option<&char> {
        self.chars.peek()
    }

    pub fn next_token(&mut self) -> Token {
        todo!()
    }

    fn skip_whitespace(&mut self) {
        while let Some(c) = self.peek() {
            if c.is_whitespace() {
                self.advance();
            } else {
                break;
            }
        }
    }
}