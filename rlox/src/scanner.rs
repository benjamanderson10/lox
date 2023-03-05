use crate::{
    error::{Error, ErrorHandler, ErrorType},
    token::{self, Token, TokenType},
};
use std::cell::{RefCell, RefMut};

pub struct Scanner<'token, 'errorhandler> {
    pub source: &'token String,
    pub tokens: Vec<Token<'token>>,
    pub errors: &'errorhandler mut ErrorHandler<String>,
}

impl<'token, 'errorhandler> Scanner<'token, 'errorhandler> {
    pub fn new(source: &'token String, errorhandler: &'errorhandler mut ErrorHandler<String>) -> Self {
        Scanner {
            source,
            tokens: Vec::new(),
            errors: errorhandler,
        }
    }

    pub fn scan(&mut self) {
        let mut idx: usize = 0;
        let mut column: usize = 1;
        let mut line: usize = 1;
        let mut char_iter = self.source.chars().peekable();

        while let Some(c) = char_iter.next() {
            let mut src: &str = "";

            // the token sorter
            let t: Option<TokenType> = match c {
                // Whitespace
                '\n' => {
                    column = 0;
                    line += 1;
                    None
                }
                c if c.is_whitespace() => None,

                // Single
                '(' => {
                    src = &self.source[idx..=idx];
                    Some(TokenType::LeftParen)
                }
                ')' => {
                    src = &self.source[idx..=idx];
                    Some(TokenType::RightParen)
                }
                '[' => {
                    src = &self.source[idx..=idx];
                    Some(TokenType::LeftBrace)
                }
                ']' => {
                    src = &self.source[idx..=idx];
                    Some(TokenType::RightBrace)
                }
                '{' => {
                    src = &self.source[idx..=idx];
                    Some(TokenType::LeftCurlyBrace)
                }
                '}' => {
                    src = &self.source[idx..=idx];
                    Some(TokenType::RightCurlyBrace)
                }
                ',' => {
                    src = &self.source[idx..=idx];
                    Some(TokenType::Comma)
                }
                '.' => {
                    src = &self.source[idx..=idx];
                    Some(TokenType::Dot)
                }
                ';' => {
                    src = &self.source[idx..=idx];
                    Some(TokenType::Semicolon)
                }

                // Double
                '-' => {
                    if char_iter.peek() == Some(&'=') {
                        char_iter.next();
                        column += 1;
                        idx += 1;

                        src = &self.source[idx - 1..=idx];
                        Some(TokenType::MinusEqual)
                    } else {
                        src = &self.source[idx..=idx];
                        Some(TokenType::Minus)
                    }
                }

                '+' => {
                    if char_iter.peek() == Some(&'=') {
                        char_iter.next();
                        column += 1;
                        idx += 1;

                        src = &self.source[idx - 1..=idx];
                        Some(TokenType::PlusEqual)
                    } else {
                        src = &self.source[idx..=idx];
                        Some(TokenType::Plus)
                    }
                }

                '*' => {
                    if char_iter.peek() == Some(&'=') {
                        char_iter.next();
                        column += 1;
                        idx += 1;

                        src = &self.source[idx - 1..=idx];
                        Some(TokenType::StarEqual)
                    } else {
                        src = &self.source[idx..=idx];
                        Some(TokenType::Star)
                    }
                }

                '/' => {
                    if char_iter.peek() == Some(&'=') {
                        char_iter.next();
                        column += 1;
                        idx += 1;

                        src = &self.source[idx - 1..=idx];
                        Some(TokenType::SlashEqual)
                    } else if char_iter.peek() == Some(&'/') {
                        while let Some(c) = char_iter.next() {
                            if c == '\n' {
                                line += 1;
                                column = 0;
                                idx += 1;
                                break;
                            }
                            idx += 1;
                        }
                        None
                    } else {
                        src = &self.source[idx..=idx];
                        Some(TokenType::Slash)
                    }
                }

                '!' => {
                    if char_iter.peek() == Some(&'=') {
                        char_iter.next();
                        column += 1;
                        idx += 1;

                        src = &self.source[idx - 1..=idx];
                        Some(TokenType::BangEqual)
                    } else {
                        src = &self.source[idx..=idx];
                        Some(TokenType::Bang)
                    }
                }

                '=' => {
                    if char_iter.peek() == Some(&'=') {
                        char_iter.next();
                        column += 1;
                        idx += 1;

                        src = &self.source[idx - 1..=idx];
                        Some(TokenType::EqualEqual)
                    } else {
                        src = &self.source[idx..=idx];
                        Some(TokenType::Equal)
                    }
                }

                '>' => {
                    if char_iter.peek() == Some(&'=') {
                        char_iter.next();
                        column += 1;
                        idx += 1;

                        src = &self.source[idx - 1..=idx];
                        Some(TokenType::GreaterEqual)
                    } else {
                        src = &self.source[idx..=idx];
                        Some(TokenType::Greater)
                    }
                }

                '<' => {
                    if char_iter.peek() == Some(&'=') {
                        char_iter.next();
                        column += 1;
                        idx += 1;

                        src = &self.source[idx - 1..=idx];
                        Some(TokenType::LessEqual)
                    } else {
                        src = &self.source[idx..=idx];
                        Some(TokenType::Less)
                    }
                }

                '&' => {
                    if char_iter.peek() == Some(&'&') {
                        char_iter.next();
                        column += 1;
                        idx += 1;

                        src = &self.source[idx - 1..=idx];
                        Some(TokenType::AndAnd)
                    } else {
                        src = &self.source[idx..=idx];
                        Some(TokenType::And)
                    }
                }

                '|' => {
                    if char_iter.peek() == Some(&'|') {
                        char_iter.next();
                        column += 1;
                        idx += 1;

                        src = &self.source[idx - 1..=idx];
                        Some(TokenType::OrOr)
                    } else {
                        src = &self.source[idx..=idx];
                        Some(TokenType::Or)
                    }
                }

                c if c.is_alphabetic() || c == '_' => {
                    let start = idx;
                    let mut tokentype: Option<TokenType> = None;
                    while let Some(&c) = char_iter.peek() {
                        if !(c.is_alphanumeric() || c == '_') {
                            tokentype = match &self.source[start..=idx] {
                                "class" => {
                                    src = &self.source[start..=idx];
                                    Some(TokenType::Class)
                                }
                                "else" => {
                                    src = &self.source[start..=idx];
                                    Some(TokenType::Else)
                                }
                                "false" => {
                                    src = &self.source[start..=idx];
                                    Some(TokenType::False)
                                }
                                "true" => {
                                    src = &self.source[start..=idx];
                                    Some(TokenType::True)
                                }
                                "fn" => {
                                    src = &self.source[start..=idx];
                                    Some(TokenType::Fn)
                                }
                                "for" => {
                                    src = &self.source[start..=idx];
                                    Some(TokenType::For)
                                }
                                "if" => {
                                    src = &self.source[start..=idx];
                                    Some(TokenType::If)
                                }
                                "nil" => {
                                    src = &self.source[start..=idx];
                                    Some(TokenType::Nil)
                                }
                                "print" => {
                                    src = &self.source[start..=idx];
                                    Some(TokenType::Print)
                                }
                                "return" => {
                                    src = &self.source[start..=idx];
                                    Some(TokenType::Return)
                                }
                                "super" => {
                                    src = &self.source[start..=idx];
                                    Some(TokenType::Super)
                                }
                                "this" => {
                                    src = &self.source[start..=idx];
                                    Some(TokenType::This)
                                }
                                "var" => {
                                    src = &self.source[start..=idx];
                                    Some(TokenType::Var)
                                }
                                "while" => {
                                    src = &self.source[start..=idx];
                                    Some(TokenType::While)
                                }
                                _ => {
                                    src = &self.source[start..=idx];
                                    Some(TokenType::Identifier(self.source[start..=idx].to_string()))
                                }
                            };
                            break;
                        }
                        column += 1;
                        idx += 1;
                        char_iter.next();
                    }
                    tokentype
                }

                c if c.is_numeric() => {
                    let start = idx;
                    let mut tokentype: Option<TokenType> = None;
                    while let Some(&c) = char_iter.peek() {
                        if !(c.is_alphanumeric() || c == '_' || c == '.') {
                            src = &self.source[start..=idx];
                            tokentype = match src.parse::<f64>() {
                                Ok(num) => Some(TokenType::Number(num)),
                                Err(_) => {
                                    self.errors.push(Error {
                                        line,
                                        column,
                                        message: format!("Failed to parse number \"{}\".", src),
                                        errortype: ErrorType::Scanning,
                                    });
                                    None
                                }
                            };
                            break;
                        }
                        column += 1;
                        idx += 1;
                        char_iter.next();
                    };
                    tokentype
                }

                '\"' => {
                    let start = idx;
                    let mut tt = None;

                    while let c = char_iter.next() {
                        match c {
                            Some(c) => {
                                if c == '\"' {
                                    src = &self.source[start..=idx + 1];
                                    column += 1;
                                    idx += 1;
                                    tt = Some(TokenType::String(self.source[start..=idx + 1].to_string()));
                                    break;
                                } else {
                                    column += 1;
                                    idx += 1;
                                }
                            }
                            None => {
                                self.errors.push(Error {
                                    line,
                                    column,
                                    message: "Missing closing quote".to_string(),
                                    errortype: ErrorType::Scanning,
                                });
                                break;
                            }
                        };
                    }
                    tt
                }

                _ => {
                    self.errors.push(Error {
                        line,
                        column,
                        message: format!("Unexpected character '{}'", c),
                        errortype: ErrorType::Scanning,
                    });
                    None
                }
            };

            match t {
                Some(t) => {
                    let token = Token { column, tokentype: t, src, line };
                    self.tokens.push(token);
                }
                None => {}
            }

            column += 1;
            idx += 1;
        }
    }
}
