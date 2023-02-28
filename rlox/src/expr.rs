use crate::token::Token;

pub enum Expr<'a> {
    Grouping(Box<Expr<'a>>),
    Literal(&'a Token<'a>),
    Unary(&'a Token<'a>, Box<Expr<'a>>),
    Binary(Box<Expr<'a>>, &'a Token<'a>, Box<Expr<'a>>),
    Expression(Box<Expr<'a>>),
    Null,
}
