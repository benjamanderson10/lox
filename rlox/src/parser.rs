use std::cell::RefCell;

use crate::{token::{Token, TokenType}, expr::Expr, error::ErrorHandler};

pub struct Parser<'a, 'b> {
    pub tokens: Vec<Token<'a>>,
    pub expressions: RefCell<Vec<Box<Expr<'a>>>>,
    pub errorhandler: &'b mut ErrorHandler<String>,
}

impl<'a, 'b> Parser <'a, 'b> {

    pub fn parse_expr(&'a self) {
        use crate::expr::Expr::*;
        use crate::token::TokenType::*;
        self.expressions.borrow_mut().push(Parser::rec_parse(&self.tokens));
    }

    pub fn rec_parse(string: &'a [Token<'a>]) -> Box<Expr<'a>> {
        use crate::expr::Expr::*;
        use crate::token::TokenType::*;
        let mut hidx = 0;
        let mut high = &string[0].tokentype;
        if string.len() >= 3 {
            for (i, a) in string.iter().map(|a| &a.tokentype).enumerate() {
                if precedence(a, &high) {
                    hidx = i;
                    high = a;
                }
            }
        }

        let e = match *high {
            EqualEqual | Greater | GreaterEqual | Less | LessEqual | Minus | MinusMinus | MinusEqual | Plus | PlusPlus | PlusEqual | Star | StarEqual | Bang | BangEqual | And | AndAnd | Or | OrOr => {println!("{}", &string[hidx]); Binary ( Parser::rec_parse(&string[0..hidx]), &string[hidx], Parser::rec_parse(&string[hidx+1..]))}
            Identifier(_) | String(_) | Number(_) => Literal (&string[hidx]),
            _ => Null
        };

        return Box::new(e);


        
        fn precedence(a: &TokenType, high: &TokenType) -> bool {
            fn num(t: &TokenType) -> usize {
                match t {
                    Identifier(_) | String(_) | Number(_) => 6,
                    LeftParen | RightParen => 5,
                    Bang => 4,
                    Star | Slash => 3,
                    Plus | Minus => 2,
                    _ => 1
                }
            }
            println!("{a:?}{}, {high:?}{}", num(a), num(high));
            num(high) >= num(a)
        }
    }

    pub fn pretty_print(&self) {
        
        for expr in self.expressions.take() {
            println!("{}", Self::print_expr(&expr));
        }
    }
    pub fn print_expr(expr: &crate::expr::Expr) -> String {
        use crate::expr::Expr::*;
        match expr {
            Literal(a) => format!("L{}", a.src),
            Unary(a, ex) => format!("U({}{})", a.src, Self::print_expr(ex)),
            Grouping(ex) => format!("G({})", Self::print_expr(ex)),
            Expression(ex) => format!("E({})", Self::print_expr(ex)),
            Binary(ex1, a, ex2) => format!("B({} {} {})", Self::print_expr(ex1), a.src, Self::print_expr(ex2)),
            Null => "N".to_string(),
        }
    }
}