use crate::token::Token;


pub enum Expression<'a> {
    Literal(Token<'a>),
    Unary(Token<'a>, Box<Expression<'a>>),
    Binary(Box<Expression<'a>>, Token<'a>, Box<Expression<'a>>),
    Grouping(Box<Expression<'a>>),
    Operation(Box<Expression<'a>>, Token<'a>, Box<Expression<'a>>),
}