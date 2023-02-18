#[derive(Debug)]
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
    MinusMinus,
    MinusEqual,
    Plus,
    PlusPlus,
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
}
