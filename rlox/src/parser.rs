use std::cell::RefCell;

use crate::{
    error::ErrorHandler,
    expr::Expr,
    token::{Token, TokenType},
};

pub struct Parser<'a, 'b> {
    pub tokens: Vec<Token<'a>>,
    pub expressions: RefCell<Vec<Box<Expr<'a>>>>,
    pub errorhandler: &'b mut ErrorHandler<String>,
}

impl<'a, 'b> Parser<'a, 'b> {
    pub fn parse_expr(&'a self) {
        use crate::expr::Expr::*;
        use crate::token::TokenType::*;
        self.expressions.borrow_mut().push(Parser::rec_parse(&self.tokens));
    }

    pub fn rec_parse(string: &'a [Token<'a>]) -> Box<Expr<'a>> {
        use crate::expr::Expr::*;
        use crate::token::TokenType::*;

        let mut lidx = 0;
        let mut low = usize::max_value();
        let mut par = 0usize;
        let mut par_max = 0usize;

        if string.len() >= 1 {
            for (i, a) in string.iter().map(|a| &a.tokentype).enumerate() {
                if *a == LeftParen {
                    println!("up");
                    par += 1;
                } else if *a == RightParen {
                    println!("down");
                    par -= 1;
                } else {
                    let p = Self::precedence(a, par);
                    if p <= low {
                        lidx = i;
                        low = p;
                        par_max = par;
                        println!("{}: pr: {:b}, par: {}, par_max: {}, low: {low}", string[i], p, par, par_max);
                    }
                }
            }
        }

        for t in string {
            print!("{} ", t);
        }
        println!();
        println!("{}, {}", par, par_max);
        let e = if par_max > 0 && par == 0 {
            Grouping(Self::rec_parse(&string[1..(string.len() - 1)]))
        } else {
            match string[lidx].tokentype {
                EqualEqual | Greater | GreaterEqual | Less | LessEqual | Minus | MinusEqual | Plus | PlusEqual | Star | StarEqual | BangEqual | And | AndAnd | Or | OrOr => {
                    println!("{:?}", string[lidx].tokentype);
                    Binary(Parser::rec_parse(&string[0..lidx]), &string[lidx], Parser::rec_parse(&string[lidx + 1..]))
                }
                Identifier(_) | String(_) | Number(_) => Literal(&string[lidx]),
                //LeftParen => Grouping(Self::rec_parse(&string[..])),
                _ => {
                    println!("{:?}", string[lidx].tokentype);
                    Null
                }
            }
        };

        return Box::new(e);
    }
    fn precedence(a: &TokenType, numpar: usize) -> usize {
        use crate::expr::Expr::*;
        use crate::token::TokenType::*;
        (numpar << 32)
            | match a {
                Identifier(_) | String(_) | Number(_) | LeftParen | RightParen => 11,
                Bang => 9,
                Star | Slash => 7,
                Plus | Minus => 5,
                LessEqual | GreaterEqual | Less | Greater => 3,
                EqualEqual | BangEqual => 1,
                _ => 0,
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
