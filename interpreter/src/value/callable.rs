/*
 * Copyright (C) 2022 Umut İnan Erdoğan <umutinanerdogan@pm.me>
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use std::cell::RefCell;
use std::rc::Rc;

use common::{location::Location, stmt::Stmt};
use common::errors::{Result, ReportKind};

use crate::env::Env;
use crate::{value::Value, Interpreter};

pub trait Callable {
    fn arity(&self, paren_loc: &Location) -> Result<usize>;
    fn call(&self, interpreter: &mut Interpreter, args: Vec<Value>, paren_loc: &Location) -> Result<Value>;
}

impl Callable for Value {
    fn call(&self, interpreter: &mut Interpreter, args: Vec<Value>, paren_loc: &Location) -> Result<Value> {
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
            Value::Fn(Stmt::Fn(_, fn_args, body, return_expr), env) => {
                let env = Rc::new(RefCell::new(Env::from_parent(env.clone())));

                {
                    let mut env_borrow = env.borrow_mut();

                    for (i, arg) in fn_args.iter().enumerate() {
                        env_borrow.define(arg.lexeme().to_string(), args[i].clone());
                    }
                }

                let saved_return_value = interpreter.return_value.clone();
                let mut return_value = Value::Null;
                interpreter.with_env(env, |interpreter| {
                    interpreter.interpret(body.clone())?;

                    if let Some(ret) = &interpreter.return_value {
                        return_value = ret.clone();
                        interpreter.return_value = saved_return_value;
                        return Ok(());
                    }

                    return_value = interpreter.evaluate(return_expr)?;
                    Ok(())
                })?;

                Ok(return_value)
            },
            Value::NativeFn(fn_ptr, _) => (fn_ptr)(interpreter, args),
            _ => unreachable!()
        }
    }

    fn arity(&self, paren_loc: &Location) -> Result<usize> {
        match self {
            Value::Fn(Stmt::Fn(_, fn_args, _, _), _) => Ok(fn_args.len()),
            Value::NativeFn(_, arity) => Ok(*arity),
            _ => error!(ReportKind::TypeError, &format!("{self} is not a function"), paren_loc)
        }
    }
}
