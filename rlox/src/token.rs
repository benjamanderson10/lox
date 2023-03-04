use std::fmt;
use std::fmt::Display;

#[derive(Debug, PartialEq, PartialOrd)]
pub enum TokenType {
    // Single Char
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    LeftCurlyBrace,
    RightCurlyBrace,
    Comma,
    Dot,
    Semicolon,

    // Double
    Minus,
    MinusEqual,
    Plus,
    PlusEqual,
    Star,
    StarEqual,
    Slash,
    SlashEqual,
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    And,
    AndAnd,
    Or,
    OrOr,

    // Literals
    Identifier(String),
    String(String),
    Number(f64),

    // Keywords
    Class,
    Else,
    False,
    True,
    Fn,
    For,
    If,
    Nil,
    Print,
    Return,
    Super,
    This,
    Var,
    While,
}

#[derive(Debug)]
pub struct Token<'a> {
    pub tokentype: TokenType,
    pub src: &'a str,
    pub line: usize,
    pub column: usize,
}

impl fmt::Display for Token<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.src)
    }
}
