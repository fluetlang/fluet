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
    Bool(bool),
    NativeFn(fn(Vec<Value>) -> Result<Value>, usize),
    Null,
    Number(f64),
    String(String),
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::Bool(bool) => write!(f, "{}", bool),
            Value::NativeFn(_, _) => write!(f, "<native fn>"),
            Value::Null => write!(f, "null"),
            Value::Number(number) => write!(f, "{}", number),
            Value::String(string) => write!(f, "'{}'", string),
        }
    }
}

impl Callable for Value {
    fn call(&mut self, args: Vec<Value>, paren_loc: &Location) -> Result<Value> {
        match self {
            Value::NativeFn(fn_ptr, arity) => {
                if args.len() != *arity {
                    return error!(
                        ReportKind::RuntimeError,
                        &format!(
                            "Expected {arity} arguments but got {}.",
                            args.len()
                        ),
                        paren_loc
                    );
                }

                (fn_ptr)(args)
            }
            _ => error!(ReportKind::TypeError, &format!("{self} is not a function"), paren_loc)
        }
    }

    fn arity(&self) -> usize {
        match self {
            _ => 0
        }
    }
}
