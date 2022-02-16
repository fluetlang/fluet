/*
 * Copyright (C) 2022 Umut İnan Erdoğan <umutinanerdogan@pm.me>
 * 
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

#[macro_use]
extern crate common;

use common::env::Env;
use common::expr::Expr;
use common::errors::{ReportKind, Result};
use common::location::Location;
use common::stmt::Stmt;
use common::token::{Token, TokenType, Literal};
use common::value::Value;

pub struct Interpreter {
    env: Env,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            env: Env::new(),
        }
    }

    pub fn interpret(&mut self, statements: Vec<Stmt>) -> Result<Value> {
        let mut value = Value::Null;
        for statement in statements {
            value = self.execute(statement)?;
        }

        Ok(value)
    }

    pub fn execute(&mut self, statement: Stmt) -> Result<Value> {
        match statement {
            Stmt::Expr(expr) => self.evaluate(expr),
            Stmt::Let(name, expr) => {
                let value = self.evaluate(expr)?;
                self.env.define(name.lexeme().to_string(), value.clone());
                Ok(value)
            },
        }
    }

    fn evaluate(&mut self, expr: Expr) -> Result<Value> {
        match expr {
            Expr::Assignment(name, value) => {
                let value = self.evaluate(*value)?;
                self.env.assign(name, &value)?;
                Ok(value)
            },
            Expr::Binary(lhs, op, rhs) => self.evaluate_binary(*lhs, op, *rhs),
            Expr::Block(statements, expr)
                => self.evaluate_block(statements, expr),
            Expr::Grouping(expr) => self.evaluate(*expr),
            Expr::Literal(literal) => Ok(self.evaluate_literal(literal)),
            Expr::Unary(op, expr) => self.evaluate_unary(op, *expr),
            Expr::Variable(name) => self.env.get(name).map(|value| value.clone()),
        }
    }

    fn evaluate_binary(&mut self, lhs: Expr, op: Token, rhs: Expr) -> Result<Value> {
        let lhs = self.evaluate(lhs)?;
        let rhs = self.evaluate(rhs)?;

        match (lhs, op.token_type(), rhs) {
            (lhs, TokenType::LogicalAnd, rhs)
                => Ok(Value::Bool(self.is_truthy(&lhs, op.location())? && self.is_truthy(&rhs, op.location())?)),
            (lhs, TokenType::LogicalOr, rhs)
                => Ok(Value::Bool(self.is_truthy(&lhs, op.location())? || self.is_truthy(&rhs, op.location())?)),

            (lhs, TokenType::BangEqual, rhs)
                => Ok(Value::Bool(!self.is_equal(lhs, rhs))),
            (lhs, TokenType::EqualEqual, rhs)
                => Ok(Value::Bool(self.is_equal(lhs, rhs))),

            (Value::Number(lhs), TokenType::Greater, Value::Number(rhs))
                => Ok(Value::Bool(lhs > rhs)),
            (Value::Number(lhs), TokenType::GreaterEqual, Value::Number(rhs))
                => Ok(Value::Bool(lhs >= rhs)),
            (Value::Number(lhs), TokenType::Less, Value::Number(rhs))
                => Ok(Value::Bool(lhs < rhs)),
            (Value::Number(lhs), TokenType::LessEqual, Value::Number(rhs))
                => Ok(Value::Bool(lhs <= rhs)),
            
            (Value::Number(lhs), TokenType::Minus, Value::Number(rhs))
                => Ok(Value::Number(lhs - rhs)),
            (Value::Number(lhs), TokenType::Plus, Value::Number(rhs))
                => Ok(Value::Number(lhs + rhs)),
            (Value::Number(lhs), TokenType::Slash, Value::Number(rhs))
                => Ok(Value::Number(lhs / rhs)),
            (Value::Number(lhs), TokenType::Star, Value::Number(rhs))
                => Ok(Value::Number(lhs * rhs)),

            (Value::String(lhs), TokenType::Plus, Value::String(rhs))
                => Ok(Value::String(format!("{}{}", lhs, rhs))),
            (Value::String(lhs), TokenType::Plus, rhs)
                => Ok(Value::String(format!("{}{}", lhs, rhs))),
            (lhs, TokenType::Plus, Value::String(rhs))
                => Ok(Value::String(format!("{}{}", lhs, rhs))),
            
            (_, token_type, _) => error!(
                ReportKind::TypeError,
                &format!("invalid binary operation '{}'", token_type),
                op.location()
            ),
        }
    }

    fn evaluate_block(&mut self, statements: Vec<Stmt>, expr: Option<Box<Expr>>) -> Result<Value> {
        let previous = self.env.clone();
        let env = Env::from_parent(previous.clone());
        self.env = env;

        for statement in statements {
            self.execute(statement)?;
        }

        let expr = match expr {
            Some(expr) => self.evaluate(*expr)?,
            None => Value::Null,
        };

        self.env = previous;
        Ok(expr)
    }

    fn evaluate_literal(&self, literal: Literal) -> Value {
        match literal {
            Literal::Number(num) => Value::Number(num),
            Literal::String(str) => Value::String(str),
            Literal::Bool(bool) => Value::Bool(bool),
            Literal::Null => Value::Null
        }
    }

    fn evaluate_unary(&mut self, op: Token, expr: Expr) -> Result<Value> {
        let rhs = self.evaluate(expr)?;

        match op.token_type() {
            TokenType::Minus => match rhs {
                Value::Number(num) => Ok(Value::Number(-num)),
                _ => error!(
                    ReportKind::TypeError,
                    "Unary minus operator can only be applied to numbers",
                    op.location()
                )
            },
            TokenType::Bang => Ok(Value::Bool(!self.is_truthy(&rhs, op.location())?)),
            _ => error!(
                ReportKind::TypeError,
                "Unary operator not implemented",
                op.location()
            )
        }
    }

    // FIXME: accept a location value instead of a token
    fn is_truthy(&self, value: &Value, location: &Location) -> Result<bool> {
        match value {
            Value::Null | Value::Bool(false) => Ok(false),
            Value::Bool(true) => Ok(true),
            _ => error!(
                ReportKind::TypeError,
                "Expected a boolean or null",
                location
            )
        }
    }

    fn is_equal(&self, lhs: Value, rhs: Value) -> bool {
        match (lhs, rhs) {
            (Value::Number(lhs), Value::Number(rhs)) => lhs == rhs,
            (Value::String(lhs), Value::String(rhs)) => lhs == rhs,
            (Value::Bool(lhs), Value::Bool(rhs)) => lhs == rhs,
            (Value::Null, Value::Null) => true,
            _ => false
        }
    }
}
