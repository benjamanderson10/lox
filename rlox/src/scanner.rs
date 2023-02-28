use crate::{
    error::{Error, ErrorHandler, ErrorType},
    token::{Token, TokenType},
};

pub struct Scanner<'a, 'b> {
    pub source: &'a String,
    pub tokens: Vec<Token<'a>>,
    pub errors: &'b mut ErrorHandler<String>,
}

impl<'a, 'b> Scanner<'a, 'b> {
    pub fn new(source: &'a String, errorhandler: &'b mut ErrorHandler<String>) -> Self {
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
            match c {
                // Whitespace
                '\n' => {
                    column = 0;
                    line += 1;
                }
                c if c.is_whitespace() => {}

                // Single
                '(' => self.tokens.push(Token {
                    tokentype: TokenType::LeftParen,
                    src: &self.source[idx..=idx],
                }),
                ')' => self.tokens.push(Token {
                    tokentype: TokenType::RightParen,
                    src: &self.source[idx..=idx],
                }),
                '[' => self.tokens.push(Token {
                    tokentype: TokenType::LeftBrace,
                    src: &self.source[idx..=idx],
                }),
                ']' => self.tokens.push(Token {
                    tokentype: TokenType::RightBrace,
                    src: &self.source[idx..=idx],
                }),
                '{' => self.tokens.push(Token {
                    tokentype: TokenType::LeftCurlyBrace,
                    src: &self.source[idx..=idx],
                }),
                '}' => self.tokens.push(Token {
                    tokentype: TokenType::RightCurlyBrace,
                    src: &self.source[idx..=idx],
                }),
                ',' => self.tokens.push(Token {
                    tokentype: TokenType::Comma,
                    src: &self.source[idx..=idx],
                }),
                '.' => self.tokens.push(Token {
                    tokentype: TokenType::Dot,
                    src: &self.source[idx..=idx],
                }),
                ';' => self.tokens.push(Token {
                    tokentype: TokenType::Semicolon,
                    src: &self.source[idx..=idx],
                }),

                // Double
                '-' => {
                    if char_iter.peek() == Some(&'=') {
                        char_iter.next();
                        column += 1;
                        idx += 1;
                        self.tokens.push(Token {
                            tokentype: TokenType::MinusEqual,
                            src: &self.source[idx - 1..=idx],
                        });
                    } else {
                        self.tokens.push(Token {
                            tokentype: TokenType::Minus,
                            src: &self.source[idx..=idx],
                        });
                    }
                }

                '+' => {
                    if char_iter.peek() == Some(&'=') {
                        char_iter.next();
                        column += 1;
                        idx += 1;
                        self.tokens.push(Token {
                            tokentype: TokenType::PlusEqual,
                            src: &self.source[idx - 1..=idx],
                        });
                    } else {
                        self.tokens.push(Token {
                            tokentype: TokenType::Plus,
                            src: &self.source[idx..=idx],
                        });
                    }
                }

                '*' => {
                    if char_iter.peek() == Some(&'=') {
                        char_iter.next();
                        column += 1;
                        idx += 1;
                        self.tokens.push(Token {
                            tokentype: TokenType::StarEqual,
                            src: &self.source[idx - 1..=idx],
                        });
                    } else {
                        self.tokens.push(Token {
                            tokentype: TokenType::Star,
                            src: &self.source[idx..=idx],
                        });
                    }
                }

                '/' => {
                    if char_iter.peek() == Some(&'=') {
                        char_iter.next();
                        column += 1;
                        idx += 1;
                        self.tokens.push(Token {
                            tokentype: TokenType::SlashEqual,
                            src: &self.source[idx - 1..=idx],
                        });
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
                    } else {
                        self.tokens.push(Token {
                            tokentype: TokenType::Slash,
                            src: &self.source[idx..=idx],
                        });
                    }
                }

                '!' => {
                    if char_iter.peek() == Some(&'=') {
                        char_iter.next();
                        column += 1;
                        idx += 1;
                        self.tokens.push(Token {
                            tokentype: TokenType::BangEqual,
                            src: &self.source[idx - 1..=idx],
                        });
                    } else {
                        self.tokens.push(Token {
                            tokentype: TokenType::Bang,
                            src: &self.source[idx..=idx],
                        });
                    }
                }

                '=' => {
                    if char_iter.peek() == Some(&'=') {
                        char_iter.next();
                        column += 1;
                        idx += 1;
                        self.tokens.push(Token {
                            tokentype: TokenType::EqualEqual,
                            src: &self.source[idx - 1..=idx],
                        });
                    } else {
                        self.tokens.push(Token {
                            tokentype: TokenType::Equal,
                            src: &self.source[idx..=idx],
                        });
                    }
                }

                '>' => {
                    if char_iter.peek() == Some(&'=') {
                        char_iter.next();
                        column += 1;
                        idx += 1;
                        self.tokens.push(Token {
                            tokentype: TokenType::GreaterEqual,
                            src: &self.source[idx - 1..=idx],
                        });
                    } else {
                        self.tokens.push(Token {
                            tokentype: TokenType::Greater,
                            src: &self.source[idx..=idx],
                        });
                    }
                }

                '<' => {
                    if char_iter.peek() == Some(&'=') {
                        char_iter.next();
                        column += 1;
                        idx += 1;
                        self.tokens.push(Token {
                            tokentype: TokenType::LessEqual,
                            src: &self.source[idx - 1..=idx],
                        });
                    } else {
                        self.tokens.push(Token {
                            tokentype: TokenType::Less,
                            src: &self.source[idx..=idx],
                        });
                    }
                }

                '&' => {
                    if char_iter.peek() == Some(&'&') {
                        char_iter.next();
                        column += 1;
                        idx += 1;
                        self.tokens.push(Token {
                            tokentype: TokenType::AndAnd,
                            src: &self.source[idx - 1..=idx],
                        });
                    } else {
                        self.tokens.push(Token {
                            tokentype: TokenType::And,
                            src: &self.source[idx..=idx],
                        });
                    }
                }

                '|' => {
                    if char_iter.peek() == Some(&'|') {
                        char_iter.next();
                        column += 1;
                        idx += 1;
                        self.tokens.push(Token {
                            tokentype: TokenType::OrOr,
                            src: &self.source[idx - 1..=idx],
                        });
                    } else {
                        self.tokens.push(Token {
                            tokentype: TokenType::Or,
                            src: &self.source[idx..=idx],
                        });
                    }
                }

                c if c.is_alphabetic() || c == '_' => {
                    let start = idx;

                    while let Some(&c) = char_iter.peek() {
                        if !(c.is_alphanumeric() || c == '_') {
                            match &self.source[start..=idx] {
                                "class" => self.tokens.push(Token {
                                    tokentype: TokenType::Class,
                                    src: &self.source[start..=idx],
                                }),
                                "else" => self.tokens.push(Token {
                                    tokentype: TokenType::Else,
                                    src: &self.source[start..=idx],
                                }),
                                "false" => self.tokens.push(Token {
                                    tokentype: TokenType::False,
                                    src: &self.source[start..=idx],
                                }),
                                "true" => self.tokens.push(Token {
                                    tokentype: TokenType::True,
                                    src: &self.source[start..=idx],
                                }),
                                "fn" => self.tokens.push(Token {
                                    tokentype: TokenType::Fn,
                                    src: &self.source[start..=idx],
                                }),
                                "for" => self.tokens.push(Token {
                                    tokentype: TokenType::For,
                                    src: &self.source[start..=idx],
                                }),
                                "if" => self.tokens.push(Token {
                                    tokentype: TokenType::If,
                                    src: &self.source[start..=idx],
                                }),
                                "nil" => self.tokens.push(Token {
                                    tokentype: TokenType::Nil,
                                    src: &self.source[start..=idx],
                                }),
                                "print" => self.tokens.push(Token {
                                    tokentype: TokenType::Print,
                                    src: &self.source[start..=idx],
                                }),
                                "return" => self.tokens.push(Token {
                                    tokentype: TokenType::Return,
                                    src: &self.source[start..=idx],
                                }),
                                "super" => self.tokens.push(Token {
                                    tokentype: TokenType::Super,
                                    src: &self.source[start..=idx],
                                }),
                                "this" => self.tokens.push(Token {
                                    tokentype: TokenType::This,
                                    src: &self.source[start..=idx],
                                }),
                                "var" => self.tokens.push(Token {
                                    tokentype: TokenType::Var,
                                    src: &self.source[start..=idx],
                                }),
                                "while" => self.tokens.push(Token {
                                    tokentype: TokenType::While,
                                    src: &self.source[start..=idx],
                                }),
                                _ => self.tokens.push(Token {
                                    tokentype: TokenType::Identifier(self.source[start..=idx].to_string()),
                                    src: &self.source[start..=idx],
                                }),
                            }
                            break;
                        }
                        column += 1;
                        idx += 1;
                        char_iter.next();
                    }
                }

                c if c.is_numeric() => {
                    let start = idx;

                    while let Some(&c) = char_iter.peek() {
                        if !(c.is_numeric() || c == '_' || c == '.' || c == 'x') {
                            self.tokens.push(Token {
                                tokentype: TokenType::Number(match self.source[start..=idx].parse::<f64>() {
                                    Ok(num) => num,
                                    Err(_) => {
                                        self.errors.push(Error {
                                            line,
                                            column,
                                            message: format!("Failed to parse number `{}`.", &self.source[start..=idx]),
                                            errortype: ErrorType::Scanning,
                                        });
                                        f64::NAN
                                    }
                                }),
                                src: &self.source[start..=idx],
                            });
                            break;
                        }
                        column += 1;
                        idx += 1;
                        char_iter.next();
                    }
                }

                '\"' => {
                    let start = idx;

                    while let c = char_iter.next() {
                        match c {
                            Some(c) => {
                                if c == '\"' {
                                    self.tokens.push(Token {
                                        tokentype: TokenType::String(self.source[start + 1..=idx].to_string()),
                                        src: &self.source[start..=idx + 1],
                                    });
                                    column += 1;
                                    idx += 1;
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
                        }
                    }
                }

                _ => self.errors.push(Error {
                    line,
                    column,
                    message: format!("Unexpected character '{}'", c),
                    errortype: ErrorType::Scanning,
                }),
            }
            column += 1;
            idx += 1;
        }
    }
}
