/*
 * Copyright (C) 2022 Umut İnan Erdoğan <umutinanerdogan@pm.me>
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use common::{location::Location, stmt::Stmt};
use common::errors::{Result, ReportKind};

use crate::{value::Value, Interpreter, env::Env};

pub trait Callable {
    fn arity(&self, paren_loc: &Location) -> Result<usize>;
    fn call(&mut self, interpreter: &mut Interpreter, args: Vec<Value>, paren_loc: &Location) -> Result<Value>;
}

impl Callable for Value {
    fn call(&mut self, interpreter: &mut Interpreter, args: Vec<Value>, paren_loc: &Location) -> Result<Value> {
        let arity = self.arity(paren_loc)?;
        if args.len() != arity {
            return error!(
                ReportKind::RuntimeError,
                &format!(
                    "Expected {arity} arguments but got {}.",
                    args.len()
                ),
                paren_loc
            );
        }

        match self {
            Value::Fn(Stmt::Fn(_, fn_args, body)) => {
                let mut env = Env::from_parent(Box::new(interpreter.globals()));
                for (i, arg) in fn_args.iter().enumerate() {
                    if let Some(value) = args.get(i) {
                        env.define(arg.lexeme().to_string(), value.clone());
                    }
                }

                Ok(interpreter.evaluate_with_env(body, env)?)
            },
            Value::NativeFn(fn_ptr, _) => (fn_ptr)(args),
            _ => unreachable!()
        }
    }

    fn arity(&self, paren_loc: &Location) -> Result<usize> {
        match self {
            Value::Fn(Stmt::Fn(_, fn_args, _)) => Ok(fn_args.len()),
            Value::NativeFn(_, arity) => Ok(*arity),
            _ => error!(ReportKind::TypeError, &format!("{self} is not a function"), paren_loc)
        }
    }
}
