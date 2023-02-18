use crate::{token::Token, expr::Expression, error::ErrorHandler};



pub struct Parser<'a, 'b> {
    pub tokens: Vec<Token<'a>>,
    pub expressions: Vec<Expression<'a>>,
    pub errorhandler: &'b mut ErrorHandler<String>,
}

impl<'a, 'b> Parser <'a, 'b> {

    pub fn parse(&mut self) {
        let mut index = 1;
        
        while let (Some(a), Some(b), Some(c)) = (self.tokens.get(index), self.tokens.get(index + 1), self.tokens.get(index + 2)) {
            println!("{}, {}, {}", a.src, b.src, c.src);
            index += 1;
        }
        fn eval() {
            
        }
        
    }

    pub fn pretty_print(&self) {
        use crate::expr::Expression::*;
        for expr in &self.expressions {
            println!("{}", print_expr(expr));
        }

        fn print_expr(expr: &Expression) -> String {
            format!("( {} )", match expr {
                Literal(a) => a.src.to_string(),
                Unary(a, ex) => format!("{} {}", a.src, print_expr(ex)),
                Binary(ex1, a, ex2) => format!("{} {} {}", print_expr(ex1), a.src, print_expr(ex2)),
                Grouping(ex) => print_expr(ex),
                Operation(ex1, a, ex2) => format!("{} {} {}", print_expr(ex1), a.src, print_expr(ex2)),
            })
        }
    }
}