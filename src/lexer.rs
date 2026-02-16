use crate::common::Span;

#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    // Literals
    IntegerLit(i64),
    FloatLit(f64),
    Char(char),
    StringLit(String),
    BoolLit(bool),
    Null,

    // Identifiers
    Ident(String),

    // Keywords
    Fn,
    Struct,
    Enum,
    Union,
    Impl,
    Const,
    Extern,
    Val,
    Var,
    Defer,
    While,
    For,
    In,
    Loop,
    If,
    Else,
    Match,
    Break,
    Return,
    Void,
    Undefined,

    // Operators
    Assign,      // =
    PlusAssign,  // +=
    MinusAssign, // -=
    StarAssign,  // *=
    SlashAssign, // /=

    // Bit Operators
    // & already used in unary as Ampersand.
    BitOr,  // |
    BitXor, // ^
    Shl,    // <<
    Shr,    // >>
    BitNot, // ~

    // Logical
    OrOr,   // ||
    AndAnd, // &&

    // Equality
    EqEq,  // ==
    NotEq, // !=

    // Comparison
    Lt,   // <
    Gt,   // >
    LtEq, // <=
    GtEq, // >=

    // Arithmetic
    Plus,    // +
    Minus,   // -
    Star,    // *
    Slash,   // /
    Percent, // %

    // Unary
    Bang,      // !
    Ampersand, // &
    At,        // @

    // Pipeline
    PipeGreater, // |>

    // Arrow
    Arrow,    // ->
    FatArrow, // =>

    // Delimiters
    LParen,   // (
    RParen,   // )
    LBrace,   // {
    RBrace,   // }
    LBracket, // [
    RBracket, // ]

    Comma,     // ,
    Dot,       // .
    Colon,     // :
    Semicolon, // ;
    Question,  // ?
    Eof,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
}

pub struct Lexer<'a> {
    input: &'a str,
    chars: std::iter::Peekable<std::str::CharIndices<'a>>,
    pos: u32,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            input,
            chars: input.char_indices().peekable(),
            pos: 0,
        }
    }

    fn advance(&mut self) -> Option<char> {
        let (byte_idx, c) = self.chars.next()?;
        self.pos = (byte_idx + c.len_utf8()) as u32;
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

    fn peek(&mut self) -> Option<char> {
        Some(self.chars.peek()?.1)
    }

    fn peek_next(&mut self) -> Option<char> {
        let mut c = self.chars.clone();
        c.next()?;
        Some(c.peek()?.1)
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        let start = self.pos;
        let c = match self.advance() {
            Some(c) => c,
            None => {
                return Token {
                    kind: TokenKind::Eof,
                    span: Span::new(start, start),
                };
            }
        };

        let kind = match c {
            '(' => TokenKind::LParen,
            ')' => TokenKind::RParen,
            '{' => TokenKind::LBrace,
            '}' => TokenKind::RBrace,
            '[' => TokenKind::LBracket,
            ']' => TokenKind::RBracket,
            ',' => TokenKind::Comma,
            '.' => TokenKind::Dot,
            ':' => TokenKind::Colon,
            ';' => TokenKind::Semicolon,
            '?' => TokenKind::Question,
            '+' => {
                if let Some('=') = self.peek() {
                    self.advance();
                    TokenKind::PlusAssign
                } else {
                    TokenKind::Plus
                }
            }
            '-' => {
                if let Some(n) = self.peek() {
                    match n {
                        '=' => {
                            self.advance();
                            TokenKind::MinusAssign
                        }
                        '>' => {
                            self.advance();
                            TokenKind::Arrow
                        }
                        _ => TokenKind::Minus,
                    }
                } else {
                    TokenKind::Minus
                }
            }
            '*' => {
                if let Some('=') = self.peek() {
                    self.advance();
                    TokenKind::StarAssign
                } else {
                    TokenKind::Star
                }
            }
            '/' => {
                if let Some('=') = self.peek() {
                    self.advance();
                    TokenKind::SlashAssign
                } else {
                    TokenKind::Slash
                }
            }
            '%' => TokenKind::Percent,
            '!' => {
                if let Some('=') = self.peek() {
                    self.advance();
                    TokenKind::NotEq
                } else {
                    TokenKind::Bang
                }
            }
            '@' => TokenKind::At,
            '|' => {
                if let Some(n) = self.peek() {
                    match n {
                        '|' => {
                            self.advance();
                            TokenKind::OrOr
                        }
                        '>' => {
                            self.advance();
                            TokenKind::PipeGreater
                        }
                        _ => TokenKind::BitOr,
                    }
                } else {
                    TokenKind::BitOr
                }
            }
            '&' => {
                if let Some('&') = self.peek() {
                    self.advance();
                    TokenKind::AndAnd
                } else {
                    TokenKind::Ampersand
                }
            }
            '^' => TokenKind::BitXor,
            '>' => {
                if let Some(n) = self.peek() {
                    match n {
                        '=' => {
                            self.advance();
                            TokenKind::GtEq
                        }
                        '>' => {
                            self.advance();
                            TokenKind::Shr
                        }
                        _ => TokenKind::Gt,
                    }
                } else {
                    TokenKind::Gt
                }
            }
            '<' => {
                if let Some(n) = self.peek() {
                    match n {
                        '=' => {
                            self.advance();
                            TokenKind::LtEq
                        }
                        '<' => {
                            self.advance();
                            TokenKind::Shl
                        }
                        _ => TokenKind::Lt,
                    }
                } else {
                    TokenKind::Lt
                }
            }
            '~' => TokenKind::BitNot,
            '=' => {
                if let Some(n) = self.peek() {
                    match n {
                        '=' => {
                            self.advance();
                            TokenKind::EqEq
                        }
                        '>' => {
                            self.advance();
                            TokenKind::FatArrow
                        }
                        _ => TokenKind::Assign,
                    }
                } else {
                    TokenKind::Assign
                }
            }
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
            }
            _ => panic!("Unknown char: {}", c),
        };
        Token {
            kind,
            span: Span::new(start, self.pos),
        }
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
        while let Some(c) = self.peek() {
            if matches!(c, '0'..='9' | 'A'..='Z' | 'a'..='z' | '_') {
                ident.push(c);
                self.advance();
            } else {
                break;
            }
        }

        match ident.as_str() {
            "fn" => TokenKind::Fn,
            "struct" => TokenKind::Struct,
            "enum" => TokenKind::Enum,
            "union" => TokenKind::Union,
            "impl" => TokenKind::Impl,
            "const" => TokenKind::Const,
            "extern" => TokenKind::Extern,
            "val" => TokenKind::Val,
            "var" => TokenKind::Var,
            "defer" => TokenKind::Defer,
            "while" => TokenKind::While,
            "for" => TokenKind::For,
            "in" => TokenKind::In,
            "loop" => TokenKind::Loop,
            "if" => TokenKind::If,
            "else" => TokenKind::Else,
            "match" => TokenKind::Match,
            "break" => TokenKind::Break,
            "return" => TokenKind::Return,
            "void" => TokenKind::Void,
            "undefined" => TokenKind::Undefined,
            "true" => TokenKind::BoolLit(true),
            "false" => TokenKind::BoolLit(false),
            "null" => TokenKind::Null,
            _ => TokenKind::Ident(ident),
        }
    }
    fn parse_number(&mut self, c: char) -> TokenKind {
        let mut number = String::new();
        let mut is_float = false;
        number.push(c);

        while let Some(c) = self.peek() {
            match c {
                '0'..='9' => {
                    self.advance();
                    number.push(c);
                }
                '_' => {
                    self.advance();
                }
                '.' => {
                    if is_float {
                        break;
                    } else {
                        if let Some(ch) = self.peek_next() {
                            if matches!(ch, '0'..='9') {
                                self.advance(); // .
                                self.advance(); // ch
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
                _ => break,
            }
        }

        if is_float {
            let f = number.parse::<f64>();
            if f.is_err() {
                panic!("Invalid float literal");
            }
            TokenKind::FloatLit(f.unwrap())
        } else {
            let i = number.parse::<i64>();
            if i.is_err() {
                panic!("Invalid integer literal");
            }
            TokenKind::IntegerLit(i.unwrap())
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
                }
                '"' => {
                    closed = true;
                    break;
                }
                _ => string.push(c),
            }
        }
        if !closed {
            panic!("Unterminated string literal");
        }
        TokenKind::StringLit(string)
    }
}
