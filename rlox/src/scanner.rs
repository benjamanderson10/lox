
use crate::{token::{TokenType, Token}, error::Error};


pub struct Scanner<'a> {
    pub source: &'a String,
    pub tokens: Vec<Token<'a>>,
    pub errors: Vec<Error<String>>,
}

impl<'a> Scanner<'a> {

    pub fn new(source: &'a String) -> Self {
        Scanner { source, tokens: Vec::new(), errors: Vec::new() }
    }
    
    pub fn scan(&mut self) {
        let mut idx: usize = 0;
                    
                    // Single
                    '(' => { self.tokens.push(Token { tokentype: TokenType::LeftParen, idx, offset: 1}) },
                    ')' => { self.tokens.push(Token { tokentype: TokenType::RightParen, idx, offset: 1}) },
                    '[' => { self.tokens.push(Token { tokentype: TokenType::LeftBrace, idx, offset: 1}) },
                    ']' => { self.tokens.push(Token { tokentype: TokenType::RightBrace, idx, offset: 1}) },
                    '{' => { self.tokens.push(Token { tokentype: TokenType::LeftCurlyBrace, idx, offset: 1}) },
                    '}' => { self.tokens.push(Token { tokentype: TokenType::RightCurlyBrace, idx, offset: 1}) },
                    ',' => { self.tokens.push(Token { tokentype: TokenType::Comma, idx, offset: 1}) },
                    '.' => { self.tokens.push(Token { tokentype: TokenType::Dot, idx, offset: 1}) },
                    ';' => { self.tokens.push(Token { tokentype: TokenType::Semicolon, idx, offset: 1}) },
                    '/' => { self.tokens.push(Token { tokentype: TokenType::Slash, idx, offset: 1}) },
                    '*' => { self.tokens.push(Token { tokentype: TokenType::Star, idx, offset: 1}) },

                    // Double
                    '+' => { self.tokens.push(Token { tokentype: TokenType::LeftParen, idx, offset: 1}) },
                    '-' => { self.tokens.push(Token { tokentype: TokenType::LeftParen, idx, offset: 1}) },
                    '-' => { self.tokens.push(Token { tokentype: TokenType::LeftParen, idx, offset: 1}) },
                    '-' => { self.tokens.push(Token { tokentype: TokenType::LeftParen, idx, offset: 1}) },
                    '-' => { self.tokens.push(Token { tokentype: TokenType::LeftParen, idx, offset: 1}) },
                    '-' => { self.tokens.push(Token { tokentype: TokenType::LeftParen, idx, offset: 1}) },
                    '-' => { self.tokens.push(Token { tokentype: TokenType::LeftParen, idx, offset: 1}) },
                    '-' => { self.tokens.push(Token { tokentype: TokenType::LeftParen, idx, offset: 1}) },
                    '-' => { self.tokens.push(Token { tokentype: TokenType::LeftParen, idx, offset: 1}) },
                    '-' => { self.tokens.push(Token { tokentype: TokenType::LeftParen, idx, offset: 1}) },

                    // Long
                    '"' => { self.tokens.push(Token { tokentype: TokenType::LeftParen, idx, offset: 1}) },
                    'a'..='z' | 'A'..='Z' | '_' => { 
                        while 

                        self.tokens.push(Token { tokentype: TokenType::Identifier(()), idx, offset: 1}) },
                    '-' => { self.tokens.push(Token { tokentype: TokenType::LeftParen, idx, offset: 1}) },

                    _ => { self.errors.push(Error { line, column, message: format!("Unexpected character '{}'", c)}) },
                    idx += 1;
                }

    }

