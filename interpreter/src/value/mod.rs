/*
 * Copyright (C) 2022 Umut İnan Erdoğan <umutinanerdogan@pm.me>
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

pub mod callable;

use std::fmt;
use callable::Callable;
use common::{errors::{Result, ReportKind}, location::Location};

#[derive(Debug, Clone)]
pub enum Value {
    Number(f64),
    String(String),
    Bool(bool),
    Null,
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::Number(number) => write!(f, "{}", number),
            Value::String(string) => write!(f, "'{}'", string),
            Value::Bool(bool) => write!(f, "{}", bool),
            Value::Null => write!(f, "null"),
        }
    }
}

impl Callable for Value {
    fn call(&mut self, args: Vec<Value>, paren_loc: &Location) -> Result<Value> {
        error!(ReportKind::TypeError, &format!("{self} is not a function"), paren_loc)
    }
}
