use crate::{token::{Token, TokenType}, expr::Expr, error::ErrorHandler};

pub struct Parser<'a, 'b> {
    pub tokens: Vec<Token<'a>>,
    pub expressions: Vec<Expr<'a>>,
    pub errorhandler: &'b mut ErrorHandler<String>,
}

impl<'a, 'b> Parser <'a, 'b> {

    pub fn parse_expr(&mut self) {
        use crate::expr::Expr::*;
        use crate::token::TokenType::*;
        self.expressions.push(Parser::rec_parse(&self.tokens[..]))
        
    }

    pub fn rec_parse(string: &'a [Token<'a>]) -> Box<Expr<'a>> {
        use crate::expr::Expr::*;
        use crate::token::TokenType::*;
        let mut hidx = 0;
        let mut high = &string[0].tokentype;
        for (i, a) in string.iter().map(|a| &a.tokentype).enumerate() {
            if precedence(a, &high) {
                hidx = i;
                high = a;
            }
        }

        let e = match *high {
            EqualEqual | Greater | GreaterEqual | Less | LessEqual | Minus | MinusMinus | MinusEqual | Plus | PlusPlus | PlusEqual | Star | StarEqual | Bang | BangEqual | And | AndAnd | Or | OrOr => { Binary ( Parser::rec_parse(&string[0..hidx]), &string[hidx], Parser::rec_parse(&string[hidx+1..]))}
            Identifier(_) | String(_) | Number(_) => Literal (&string[hidx]),

            _ => Null
        };

        return Box::new(e);



        fn precedence(a: &TokenType, high: &TokenType) -> bool {
            fn num(t: &TokenType) -> usize {
                match t {
                    LeftParen | RightParen => 0,
                    Bang | Minus => 1,
                    Star | Slash => 2,
                    Plus | Minus => 3,
                    _ => 4
                }
            }
            match num(high).checked_sub(num(a)) {
                Some(_) => true,
                None => false
            }
        }
    }

    pub fn pretty_print(&self) {
        use crate::expr::Expr::*;
        for expr in &self.expressions {
            println!("{}", print_expr(expr));
        }

        fn print_expr(expr: &crate::expr::Expr) -> String {
            match expr {
                Literal(a) => format!("L{}", a.src),
                Unary(a, ex) => format!("U({}{})", a.src, print_expr(ex)),
                Grouping(ex) => format!("G({})", print_expr(ex)),
                Expression(ex) => format!("E({})", print_expr(ex)),
                Binary(ex1, a, ex2) => format!("B({} {} {})", print_expr(ex1), a.src, print_expr(ex2)),
                Null => "N".to_string(),
            }
        }
    }
}