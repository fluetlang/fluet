/*
 * Copyright (C) 2022 Umut İnan Erdoğan <umutinanerdogan@pm.me>
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

pub mod callable;

use std::fmt;
use common::errors::Result;
use common::stmt::Stmt;

use crate::Interpreter;

#[derive(Clone)]
pub enum Value {
    Bool(bool),
    Fn(Stmt),
    NativeFn(fn(&mut Interpreter, Vec<Value>) -> Result<Value>, usize),
    Null,
    Number(f64),
    String(String),
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::Bool(bool) => write!(f, "{}", bool),
            Value::Fn(_) => write!(f, "<fn>"),
            Value::NativeFn(_, _) => write!(f, "<native fn>"),
            Value::Null => write!(f, "null"),
            Value::Number(number) => write!(f, "{}", number),
            Value::String(string) => write!(f, "'{}'", string),
        }
    }
}
