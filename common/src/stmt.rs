/*
 * Copyright (C) 2022 Umut İnan Erdoğan <umutinanerdogan@pm.me>
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use crate::{expr::Expr, token::Token};

#[derive(Debug, Clone)]
pub enum Stmt {
    Expr(Expr),
    Fn(Token, Vec<Token>, Vec<Stmt>, Expr),
    Let(Token, Expr),
    Loop(Vec<Stmt>),
    While(Expr, Vec<Stmt>),
}
