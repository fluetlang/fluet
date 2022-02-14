use crate::token::{Token, Literal};

#[derive(Debug, Clone)]
pub enum Expr {
    Assignment(Token, Box<Expr>),
    Binary(Box<Expr>, Token, Box<Expr>),
    Grouping(Box<Expr>),
    Literal(Literal),
    Unary(Token, Box<Expr>),
    Variable(Token),
}
