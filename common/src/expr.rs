/*
 * Copyright (C) 2022 Umut İnan Erdoğan <umutinanerdogan@pm.me>
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use crate::stmt::Stmt;
use crate::token::{Literal, Token};

#[derive(Debug, Clone)]
pub enum Expr {
    Assignment(Token, Box<Expr>),
    Binary(Box<Expr>, Token, Box<Expr>),
    Block(Vec<Stmt>, Option<Box<Expr>>),
    Call(Box<Expr>, Token, Vec<Expr>),
    Grouping(Box<Expr>),
    If(Box<Expr>, Box<Expr>, Option<Box<Expr>>),
    Literal(Literal),
    Logical(Box<Expr>, Token, Box<Expr>),
    Unary(Token, Box<Expr>),
    Variable(Token),
}
