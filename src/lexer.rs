use crate::common::{Span, Symbol};

#[derive(Debug, Clone)]
pub enum TokenKind {
    Val, Var, Fn, If, Else, Return, True, False,

    LBrace, RBrace, LParen, RParen, LBracket, RBracket,
    SemiColon, Colon, Arrow,

    Plus, Minus, Star, Slash, Percent, Eq, EqEq, Pipeline,

    Int(i64),
    Float(f64),
    Char(char),
    Ident(String),
    String(String),
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
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            input,
            chars: input.chars().peekable(),
            pos: 0,
        }
    }

    fn advance(&mut self) -> Option<char> {
        let c = self.chars.next()?;
        self.pos += 1;
        Some(c)
    }

    fn consume(&mut self, expected: char) {
        if let Some(c) = self.advance() {
            if c != expected {
                panic!("Expected char: {}", expected);
            }
        } else {
            panic!("Unexpected end of input");
        }
    }

    fn peek(&mut self) -> Option<&char> {
        self.chars.peek()
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        let start = self.pos;
        let c = match self.advance() {
            Some(c) => c,
            None => return Token { kind: TokenKind::EOF, span: Span::new(start, start), }
        };

        let kind = match c {
            '{' => TokenKind::LBrace,
            '}' => TokenKind::RBrace,
            '(' => TokenKind::LParen,
            ')' => TokenKind::RParen,
            '[' => TokenKind::LBracket,
            ']' => TokenKind::RBracket,
            ';' => TokenKind::SemiColon,
            ':' => TokenKind::Colon,
            '+' => TokenKind::Plus,
            '-' => if let Some(&'>') = self.peek() {
                self.advance();
                TokenKind::Arrow
            } else {
                TokenKind::Minus
            },
            '*' => TokenKind::Star,
            '/' => TokenKind::Slash,
            '%' => TokenKind::Percent,
            '|' => if let Some(&'>') = self.peek() {
                self.advance();
                TokenKind::Pipeline
            } else {
                panic!("Unexpected char '|'"); // TODO: Error handling
            },
            '=' => if let Some(&'=') = self.peek() {
                self.advance();
                TokenKind::EqEq
            } else {
                TokenKind::Eq
            },
            '0'..='9' => self.parse_number(c),
            'a'..='z' | 'A'..='Z' | '_' => self.parse_ident(c),
            '"' => self.parse_string(),
            '\'' => {
                if let Some(ch) = self.advance() {
                    let tk = TokenKind::Char(ch);
                    self.consume('\'');
                    tk
                } else {
                    panic!("Unterminated char literal");
                }
            },
            _ => panic!("Unknown char: {}", c),
        };
        Token { kind, span: Span::new(start, self.pos) }
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

    fn parse_ident(&mut self, c: char) -> TokenKind {
        let mut ident = String::new();
        ident.push(c);
        while let Some(&c) = self.peek() {
            if matches!(c, '0'..='9' | 'A'..='Z' | 'a'..='z' | '_') {
                ident.push(c);
                self.advance();
            } else {
                break;
            }
        }

        match ident.as_str() {
            "val" => TokenKind::Val,
            "var" => TokenKind::Var,
            "fn" => TokenKind::Fn,
            "if" => TokenKind::If,
            "else" => TokenKind::Else,
            "return" => TokenKind::Return,
            "true" => TokenKind::True,
            "false" => TokenKind::False,
            _ => TokenKind::Ident(ident)
        }
    }
    fn parse_number(&mut self, c: char) -> TokenKind {
        let mut number = String::new();
        let mut is_float = false;
        number.push(c);

        while let Some(&c) = self.peek() {
            match c {
                '0'..='9' => {
                    self.advance();
                    number.push(c);
                },
                '_' => {
                    self.advance();
                },
                '.' => {
                    if is_float {
                        break
                    } else {
                        if let Some(&ch) = self.peek() { 
                            if matches!(ch, '0'..='9') {
                                self.advance();
                                number.push(c);
                                number.push(ch);
                                is_float = true;
                            } else { 
                                break;
                            }
                        } else { 
                            break;
                        }
                    }
                }
                _ => break
            }
        }

        if is_float {
            let f = number.parse::<f64>();
            if f.is_err() { panic!("Invalid float literal"); }
            TokenKind::Float(f.unwrap())
        } else {
            let i = number.parse::<i64>();
            if i.is_err() { panic!("Invalid integer literal"); }
            TokenKind::Int(i.unwrap())
        }
    }

    fn parse_string(&mut self) -> TokenKind {
        let mut string = String::new();
        let mut closed = false;
        while let Some(c) = self.advance() {
            match c {
                '\\' => {
                    if let Some(next) = self.advance() {
                        match next {
                            'n' => string.push('\n'),
                            't' => string.push('\t'),
                            'r' => string.push('\r'),
                            '"' => string.push('"'),
                            '\\' => string.push('\\'),
                            _ => {
                                string.push('\\');
                                string.push(next);
                            }
                        }
                    } else {
                        panic!("Unterminated string literal");
                    }
                },
                '"' => {
                    closed = true;
                    break;
                },
                _ => string.push(c)
            }
        }
        if !closed { panic!("Unterminated string literal"); }
        TokenKind::String(string)
    }
}