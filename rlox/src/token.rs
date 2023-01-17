pub enum TokenType<'a> {
    
    // Single Char
    LeftParen, RightParen,
    LeftBrace, RightBrace,
    LeftCurlyBrace, RightCurlyBrace,
    Comma, Dot, Minus, Plus, Semicolon, Slash, Star,

    // Double
    Bang, BangEqual,
    Equal, EqualEqual,
    Greater, GreaterEqual,
    Less, LessEqual,
    And, AndAnd,
    Or, OrOr,

    // Literals
    Identifier(&'a str),
    String(&'a str),
    Number(&'a str),
    
    // Keywords
    Class, Else, False, True, Fn, For, If, Nil, Print, Return, Super, This, Var, While,
}

pub struct Token<'a> {
    pub tokentype: TokenType<'a>,
    pub idx: usize,
    pub offset: usize,
}
