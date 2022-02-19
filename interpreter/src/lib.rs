/*
 * Copyright (C) 2022 Umut İnan Erdoğan <umutinanerdogan@pm.me>
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

#[macro_use]
extern crate common;

pub mod value;
pub mod env;

use common::errors::{ReportKind, Result};
use common::expr::Expr;
use common::location::Location;
use common::stmt::Stmt;
use common::token::{Literal, Token, TokenType};

use env::Env;
use value::Value;
use value::callable::Callable;

pub struct Interpreter {
    env: Env,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            env: Env::from_parent({
                let mut globals = Env::new();

                globals.define("print".to_string(), Value::NativeFn(|args| {
                    match &args[0] {
                        Value::String(s) => println!("{}", s),
                        value => println!("{}", value),
                    }

                    Ok(Value::Null)
                }, 1));

                globals.define("eprint".to_string(), Value::NativeFn(|args| {
                    match &args[0] {
                        Value::String(s) => eprintln!("{}", s),
                        value => eprintln!("{}", value),
                    }

                    Ok(Value::Null)
                }, 1));

                Box::new(globals)
            })
        }
    }

    pub fn interpret(&mut self, statements: Vec<Stmt>) -> Result<Value> {
        let mut value = Value::Null;
        for statement in statements {
            value = self.execute(&statement)?;
        }

        Ok(value)
    }

    pub fn execute(&mut self, statement: &Stmt) -> Result<Value> {
        match statement {
            Stmt::Expr(expr) => self.evaluate(expr),
            Stmt::Let(name, expr) => {
                let value = self.evaluate(expr)?;
                self.env.define(name.lexeme().to_string(), value.clone());
                Ok(value)
            }
            Stmt::Loop(body) => self.execute_loop(body),
            Stmt::While(condition, body) => self.execute_while(condition, body),
        }
    }

    fn execute_loop(&mut self, body: &Vec<Stmt>) -> Result<Value> {
        loop {
            self.env = Env::from_parent(Box::new(self.env.clone()));

            for statement in body {
                self.execute(statement)?;
            }

            self.env = *self.env.parent().unwrap().clone();
        }

        Ok(Value::Null)
    }

    fn execute_while(&mut self, condition: &Expr, body: &Vec<Stmt>) -> Result<Value> {
        let mut condition_value = self.evaluate(condition)?;

        // FIXME: implement reading expression locations somehow
        while self.is_truthy_restrictive(
            &condition_value,
            &Location {
                filename: "".to_string(),
                line: "".to_string(),
                row: 0,
                column: 0,
            },
        )? {
            self.env = Env::from_parent(Box::new(self.env.clone()));

            for statement in body {
                self.execute(statement)?;
            }

            self.env = *self.env.parent().unwrap().clone();
            condition_value = self.evaluate(condition)?;
        }

        Ok(Value::Null)
    }

    fn evaluate(&mut self, expr: &Expr) -> Result<Value> {
        match expr {
            Expr::Assignment(name, value) => {
                let value = self.evaluate(value)?;
                self.env.assign(name, &value)?;
                Ok(value)
            }
            Expr::Binary(lhs, op, rhs) => self.evaluate_binary(lhs, op, rhs),
            Expr::Block(statements, expr) => self.evaluate_block(statements, expr),
            Expr::Call(callee, paren, args)
                => self.evaluate_call(callee, paren, args),
            Expr::Grouping(expr) => self.evaluate(expr),
            Expr::If(condition, then_branch, else_branch) => {
                self.evaluate_conditional(condition, then_branch, else_branch)
            }
            Expr::Literal(literal) => Ok(self.evaluate_literal(literal)),
            Expr::Logical(lhs, op, rhs) => self.evaluate_logical(lhs, op, rhs),
            Expr::Unary(op, expr) => self.evaluate_unary(op, expr),
            Expr::Variable(name) => self.env.get(name).map(|value| value.clone()),
        }
    }

    fn evaluate_binary(&mut self, lhs: &Expr, op: &Token, rhs: &Expr) -> Result<Value> {
        let lhs = self.evaluate(lhs)?;
        let rhs = self.evaluate(rhs)?;

        match (lhs, op.token_type(), rhs) {
            (lhs, TokenType::BangEqual, rhs) => Ok(Value::Bool(!self.is_equal(lhs, rhs))),
            (lhs, TokenType::EqualEqual, rhs) => Ok(Value::Bool(self.is_equal(lhs, rhs))),

            (Value::Number(lhs), TokenType::Greater, Value::Number(rhs)) => {
                Ok(Value::Bool(lhs > rhs))
            }
            (Value::Number(lhs), TokenType::GreaterEqual, Value::Number(rhs)) => {
                Ok(Value::Bool(lhs >= rhs))
            }
            (Value::Number(lhs), TokenType::Less, Value::Number(rhs)) => Ok(Value::Bool(lhs < rhs)),
            (Value::Number(lhs), TokenType::LessEqual, Value::Number(rhs)) => {
                Ok(Value::Bool(lhs <= rhs))
            }

            (Value::Number(lhs), TokenType::Minus, Value::Number(rhs)) => {
                Ok(Value::Number(lhs - rhs))
            }
            (Value::Number(lhs), TokenType::Percent, Value::Number(rhs)) => {
                Ok(Value::Number(lhs % rhs))
            }
            (Value::Number(lhs), TokenType::Plus, Value::Number(rhs)) => {
                Ok(Value::Number(lhs + rhs))
            }
            (Value::Number(lhs), TokenType::Slash, Value::Number(rhs)) => {
                Ok(Value::Number(lhs / rhs))
            }
            (Value::Number(lhs), TokenType::Star, Value::Number(rhs)) => {
                Ok(Value::Number(lhs * rhs))
            }

            (Value::String(lhs), TokenType::Plus, Value::String(rhs)) => {
                Ok(Value::String(format!("{}{}", lhs, rhs)))
            }
            (Value::String(lhs), TokenType::Plus, rhs) => {
                Ok(Value::String(format!("{}{}", lhs, rhs)))
            }
            (lhs, TokenType::Plus, Value::String(rhs)) => {
                Ok(Value::String(format!("{}{}", lhs, rhs)))
            }

            (_, token_type, _) => error!(
                ReportKind::TypeError,
                &format!("invalid binary operation '{}'", token_type),
                op.location()
            ),
        }
    }

    fn evaluate_block(
        &mut self,
        statements: &Vec<Stmt>,
        expr: &Option<Box<Expr>>,
    ) -> Result<Value> {
        self.env = Env::from_parent(Box::new(self.env.clone()));

        for statement in statements {
            self.execute(statement)?;
        }

        let expr = match expr {
            Some(expr) => self.evaluate(expr)?,
            None => Value::Null,
        };

        self.env = *self.env.parent().unwrap().clone();
        Ok(expr)
    }

    fn evaluate_call(&mut self, callee: &Expr, paren: &Token, args: &Vec<Expr>) -> Result<Value> {
        let mut callee = self.evaluate(callee)?;

        let args = args
            .iter()
            .map(|expr| self.evaluate(expr))
            .collect::<Result<Vec<Value>>>()?;

        callee.call(args, paren.location())
    }

    fn evaluate_conditional(
        &mut self,
        condition: &Expr,
        then_branch: &Expr,
        else_branch: &Option<Box<Expr>>,
    ) -> Result<Value> {
        let condition = self.evaluate(condition)?;

        // FIXME: implement reading expression locations somehow
        if self.is_truthy_restrictive(
            &condition,
            &Location {
                filename: "".to_string(),
                line: "".to_string(),
                row: 0,
                column: 0,
            },
        )? {
            self.evaluate(then_branch)
        } else if let Some(else_branch) = else_branch {
            self.evaluate(else_branch)
        } else {
            Ok(Value::Null)
        }
    }

    fn evaluate_literal(&self, literal: &Literal) -> Value {
        match literal {
            Literal::Number(num) => Value::Number(*num),
            Literal::String(str) => Value::String(str.clone()),
            Literal::Bool(bool) => Value::Bool(*bool),
            Literal::Null => Value::Null,
        }
    }

    fn evaluate_logical(&mut self, lhs: &Expr, op: &Token, rhs: &Expr) -> Result<Value> {
        let lhs = self.evaluate(lhs)?;
        match op.token_type() {
            TokenType::LogicalAnd => {
                if self.is_truthy(&lhs) {
                    self.evaluate(rhs)
                } else {
                    Ok(lhs)
                }
            }
            TokenType::LogicalOr => {
                if self.is_truthy(&lhs) {
                    Ok(lhs)
                } else {
                    self.evaluate(rhs)
                }
            }

            token_type => error!(
                ReportKind::TypeError,
                &format!("invalid logical operation '{}'", token_type),
                op.location()
            ),
        }
    }

    fn evaluate_unary(&mut self, op: &Token, expr: &Expr) -> Result<Value> {
        let rhs = self.evaluate(expr)?;

        match op.token_type() {
            TokenType::Minus => match rhs {
                Value::Number(num) => Ok(Value::Number(-num)),
                _ => error!(
                    ReportKind::TypeError,
                    "Unary minus operator can only be applied to numbers",
                    op.location()
                ),
            },
            TokenType::Bang => Ok(Value::Bool(!self.is_truthy(&rhs))),
            _ => error!(
                ReportKind::TypeError,
                "Unary operator not implemented",
                op.location()
            ),
        }
    }

    fn is_truthy(&self, value: &Value) -> bool {
        match value {
            Value::Bool(false) | Value::Null => false,
            _ => true,
        }
    }

    fn is_truthy_restrictive(&self, value: &Value, location: &Location) -> Result<bool> {
        match value {
            Value::Bool(false) | Value::Null => Ok(false),
            Value::Bool(true) => Ok(true),
            _ => error!(
                ReportKind::TypeError,
                "Expected a boolean or null", location
            ),
        }
    }

    fn is_equal(&self, lhs: Value, rhs: Value) -> bool {
        match (lhs, rhs) {
            (Value::Number(lhs), Value::Number(rhs)) => lhs == rhs,
            (Value::String(lhs), Value::String(rhs)) => lhs == rhs,
            (Value::Bool(lhs), Value::Bool(rhs)) => lhs == rhs,
            (Value::Null, Value::Null) => true,
            _ => false,
        }
    }
}
