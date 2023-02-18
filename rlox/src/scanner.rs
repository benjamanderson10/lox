use std::fmt::Display;

use crate::{
    error::{Error, ErrorHandler},
    token::{Token, TokenType},
};

pub struct Scanner<'a, 'b> {
    pub source: &'a String,
    pub tokens: Vec<Token<'a>>,
    pub errors: &'b mut ErrorHandler<String>,
}

impl<'a> Scanner<'a> {

    pub fn new(source: &'a String) -> Self {
        Scanner { source, tokens: Vec::new(), errors: Vec::new() }
    }

    pub fn scan(&mut self) {
        let mut idx: usize = 0;

        for (line, lines) in self.source.lines().enumerate() {
            for (column, c) in lines.chars().enumerate() {
                match c {
                    
                    // Single
                    '(' => { self.tokens.push(Token { tokentype: TokenType::LeftParen, idx, offset: 1}) },
                    ')' => { self.tokens.push(Token { tokentype: TokenType::RightParen, idx, offset: 1}) },
                    '[' => { self.tokens.push(Token { tokentype: TokenType::LeftBrace, idx, offset: 1}) },
                    ']' => { self.tokens.push(Token { tokentype: TokenType::RightBrace, idx, offset: 1}) },
                    '{' => { self.tokens.push(Token { tokentype: TokenType::LeftCurlyBrace, idx, offset: 1}) },
                    '}' => { self.tokens.push(Token { tokentype: TokenType::RightCurlyBrace, idx, offset: 1}) },
                    ',' => { self.tokens.push(Token { tokentype: TokenType::Comma, idx, offset: 1}) },
                    '.' => { self.tokens.push(Token { tokentype: TokenType::Dot, idx, offset: 1}) },
                    '-' => { self.tokens.push(Token { tokentype: TokenType::Minus, idx, offset: 1}) },
                    '+' => { self.tokens.push(Token { tokentype: TokenType::Plus, idx, offset: 1}) },
                    ';' => { self.tokens.push(Token { tokentype: TokenType::Semicolon, idx, offset: 1}) },
                    '/' => { self.tokens.push(Token { tokentype: TokenType::Slash, idx, offset: 1}) },
                    '*' => { self.tokens.push(Token { tokentype: TokenType::Star, idx, offset: 1}) },

                    // Double
                    '-' => { self.tokens.push(Token { tokentype: TokenType::LeftParen, idx, offset: 1}) },
                    '-' => { self.tokens.push(Token { tokentype: TokenType::LeftParen, idx, offset: 1}) },
                    '-' => { self.tokens.push(Token { tokentype: TokenType::LeftParen, idx, offset: 1}) },
                    '-' => { self.tokens.push(Token { tokentype: TokenType::LeftParen, idx, offset: 1}) },
                    '-' => { self.tokens.push(Token { tokentype: TokenType::LeftParen, idx, offset: 1}) },
                    '-' => { self.tokens.push(Token { tokentype: TokenType::LeftParen, idx, offset: 1}) },
                    '-' => { self.tokens.push(Token { tokentype: TokenType::LeftParen, idx, offset: 1}) },
                    '-' => { self.tokens.push(Token { tokentype: TokenType::LeftParen, idx, offset: 1}) },
                    '-' => { self.tokens.push(Token { tokentype: TokenType::LeftParen, idx, offset: 1}) },
                    '-' => { self.tokens.push(Token { tokentype: TokenType::LeftParen, idx, offset: 1}) },
                    '-' => { self.tokens.push(Token { tokentype: TokenType::LeftParen, idx, offset: 1}) },
                    '-' => { self.tokens.push(Token { tokentype: TokenType::LeftParen, idx, offset: 1}) },
                    '-' => { self.tokens.push(Token { tokentype: TokenType::LeftParen, idx, offset: 1}) },

                    _ => { self.errors.push(Error { line, column, message: format!("Unexpected character '{}'", c)}) },
                }

                idx += 1;
            }
        }
    }
}
