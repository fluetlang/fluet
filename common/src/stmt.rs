use crate::{expr::Expr, token::Token};

pub enum Stmt {
    Block(Vec<Stmt>),
    Expr(Expr),
    Let(Token, Expr),
}
