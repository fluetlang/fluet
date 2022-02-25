/*
 * Copyright (C) 2022 Umut İnan Erdoğan <umutinanerdogan@pm.me>
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

#[macro_use]
extern crate common;

use common::errors::{report_error, FluetError, ReportKind, Result};
use common::expr::Expr;
use common::stmt::Stmt;
use common::token::{Literal, Token, TokenType};

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> Result<Vec<Stmt>> {
        let mut statements = vec![];
        while !self.is_at_end() {
            let statement = self.declaration();
            match statement {
                Ok(stmt) => statements.push(stmt),
                Err(err) => {
                    match self.synchronize() {
                        Ok(_) => continue,
                        Err(_) => return Err(err),
                    };
                }
            }
        }

        Ok(statements)
    }

    pub fn parse_repl(&mut self) -> Result<Expr> {
        let mut statements = vec![];
        let mut expr = Expr::Literal(Literal::Null);

        while !self.is_at_end() {
            let last = self.current;
            match self.declaration() {
                Ok(statement) => {
                    statements.push(statement);
                }
                Err(err) => {
                    let current = self.current;
                    self.set_current(last); // rewind to before error
                    if let Ok(last_expr) = self.expression() {
                        expr = last_expr;
                    } else {
                        self.set_current(current);
                        return Err(err);
                    }
                }
            }
        }

        return Ok(Expr::Block(statements, Box::new(expr)));
    }

    fn synchronize(&mut self) -> Result<()> {
        self.advance();

        while !self.is_at_end() {
            if self.previous().token_type() == TokenType::Semicolon {
                return Ok(());
            }

            // TODO: add more cases
            match self.peek().token_type() {
                TokenType::Class
                | TokenType::Fn
                | TokenType::For
                | TokenType::If
                | TokenType::Let
                | TokenType::Return
                | TokenType::While => return Ok(()),
                _ => {}
            }

            self.advance();
        }

        Err(FluetError("".to_string()))
    }

    fn expression(&mut self) -> Result<Expr> {
        self.assignment()
    }

    fn declaration(&mut self) -> Result<Stmt> {
        if self.match_token(TokenType::Let) {
            return self.let_declaration();
        }

        self.statement()
    }

    fn let_declaration(&mut self) -> Result<Stmt> {
        let name = self.consume(TokenType::Identifier, "Expected variable name")?;

        let mut initializer = Expr::Literal(Literal::Null);
        if self.match_token(TokenType::Equal) {
            initializer = self.expression()?;
        }

        self.consume(
            TokenType::Semicolon,
            "Expected ';' after variable declaration",
        )?;
        Ok(Stmt::Let(name, initializer))
    }

    fn statement(&mut self) -> Result<Stmt> {
        if self.match_token(TokenType::Loop) {
            return self.loop_statement();
        }
        if self.match_token(TokenType::While) {
            return self.while_statement();
        }

        self.expression_statement()
    }

    fn expression_statement(&mut self) -> Result<Stmt> {
        let expr = self.expression()?;
        self.consume(TokenType::Semicolon, "Expected ';' after expression.")?;
        Ok(Stmt::Expr(expr))
    }

    fn loop_statement(&mut self) -> Result<Stmt> {
        self.consume(TokenType::LeftBrace, "Expected '{' after 'loop'.")?;

        let mut body = vec![];
        while !self.check(TokenType::RightBrace) && !self.is_at_end() {
            body.push(self.declaration()?);
        }

        self.consume(TokenType::RightBrace, "Expected '}' after loop body.")?;
        Ok(Stmt::Loop(body))
    }

    fn while_statement(&mut self) -> Result<Stmt> {
        let condition = self.expression()?;
        self.consume(TokenType::LeftBrace, "Expected '{' after 'while'.")?;

        let mut body = vec![];
        while !self.check(TokenType::RightBrace) && !self.is_at_end() {
            body.push(self.declaration()?);
        }

        self.consume(TokenType::RightBrace, "Expected '}' after loop body.")?;
        Ok(Stmt::While(condition, body))
    }

    fn assignment(&mut self) -> Result<Expr> {
        let lhs = self.conditional()?;

        if self.match_token(TokenType::Equal) {
            let equals = self.previous();
            let value = self.assignment()?;

            if let Expr::Variable(name) = lhs {
                return Ok(Expr::Assignment(name, Box::new(value)));
            }

            // Report error but don't return error
            eprintln!(
                "{}",
                report_error(
                    ReportKind::SyntaxError,
                    None,
                    "Invalid left-hand side in assignment",
                    equals.location()
                )
            );
        }

        Ok(lhs)
    }

    fn conditional(&mut self) -> Result<Expr> {
        if self.match_token(TokenType::If) {
            let condition = self.conditional()?;
            self.consume(TokenType::Then, "Expected 'then' after 'if' condition.")?;

            let then_branch = self.expression()?;
            let else_branch = if self.match_token(TokenType::Else) {
                self.expression()?
            } else {
                Expr::Literal(Literal::Null)
            };

            return Ok(Expr::If(
                Box::new(condition),
                Box::new(then_branch),
                Box::new(else_branch),
            ));
        }

        self.logic()
    }

    fn logic(&mut self) -> Result<Expr> {
        let mut expr = self.equality()?;

        while self.match_any_token(vec![TokenType::LogicalAnd, TokenType::LogicalOr]) {
            let operator = self.previous();
            let right = self.equality()?;
            expr = Expr::Logical(Box::new(expr), operator, Box::new(right));
        }

        Ok(expr)
    }

    fn equality(&mut self) -> Result<Expr> {
        let mut expr = self.comparison()?;

        while self.match_any_token(vec![TokenType::BangEqual, TokenType::EqualEqual]) {
            let operator = self.previous();
            let right = self.comparison()?;
            expr = Expr::Binary(Box::new(expr), operator, Box::new(right));
        }

        Ok(expr)
    }

    fn comparison(&mut self) -> Result<Expr> {
        let mut expr = self.term()?;

        while self.match_any_token(vec![
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ]) {
            let operator = self.previous();
            let right = self.term()?;
            expr = Expr::Binary(Box::new(expr), operator, Box::new(right));
        }

        Ok(expr)
    }

    fn term(&mut self) -> Result<Expr> {
        let mut expr = self.factor()?;

        while self.match_any_token(vec![TokenType::Minus, TokenType::Plus]) {
            let operator = self.previous();
            let right = self.factor()?;
            expr = Expr::Binary(Box::new(expr), operator, Box::new(right));
        }

        Ok(expr)
    }

    fn factor(&mut self) -> Result<Expr> {
        let mut expr = self.unary()?;

        while self.match_any_token(vec![TokenType::Percent, TokenType::Slash, TokenType::Star]) {
            let operator = self.previous();
            let right = self.unary()?;
            expr = Expr::Binary(Box::new(expr), operator, Box::new(right));
        }

        Ok(expr)
    }

    fn unary(&mut self) -> Result<Expr> {
        if self.match_any_token(vec![TokenType::Bang, TokenType::Minus]) {
            let operator = self.previous();
            let right = self.unary()?;
            return Ok(Expr::Unary(operator, Box::new(right)));
        }

        self.block()
    }

    fn block(&mut self) -> Result<Expr> {
        if self.match_token(TokenType::LeftBrace) {
            let mut statements = vec![];
            let mut expr = Expr::Literal(Literal::Null);

            while !self.check(TokenType::RightBrace) && !self.is_at_end() {
                let last = self.current;
                match self.declaration() {
                    Ok(statement) => {
                        statements.push(statement);
                    }
                    Err(err) => {
                        let current = self.current;
                        self.set_current(last); // rewind to before error
                        if let Ok(last_expr) = self.expression() {
                            expr = last_expr;
                        } else {
                            self.set_current(current);
                            return Err(err);
                        }
                    }
                }
            }

            self.consume(TokenType::RightBrace, "Expected '}' after block")?;
            return Ok(Expr::Block(statements, Box::new(expr)));
        }

        self.call()
    }

    fn call(&mut self) -> Result<Expr> {
        let mut expr = self.primary()?;

        loop {
            if self.match_token(TokenType::LeftParen) {
                expr = self.finish_call(expr)?;
            } else {
                break;
            }
        }

        Ok(expr)
    }

    fn finish_call(&mut self, callee: Expr) -> Result<Expr> {
        let mut args = vec![];

        if !self.check(TokenType::RightParen) {
            loop {
                args.push(self.expression()?);
                if !self.match_token(TokenType::Comma) { break; }
            }
        }

        let paren = self.consume(TokenType::RightParen, "Expected ')' after arguments.")?;

        Ok(Expr::Call(Box::new(callee), paren, args))
    }

    fn primary(&mut self) -> Result<Expr> {
        if self.match_token(TokenType::False) {
            return Ok(Expr::Literal(Literal::Bool(false)));
        }
        if self.match_token(TokenType::True) {
            return Ok(Expr::Literal(Literal::Bool(true)));
        }
        if self.match_token(TokenType::Null) {
            return Ok(Expr::Literal(Literal::Null));
        }

        if self.match_any_token(vec![TokenType::Number, TokenType::String]) {
            // safe to unwrap because all number and string tokens have literal values
            return Ok(Expr::Literal(self.previous().literal().unwrap().clone()));
        }

        if self.match_token(TokenType::Identifier) {
            return Ok(Expr::Variable(self.previous()));
        }

        if self.match_token(TokenType::LeftParen) {
            let expr = self.expression()?;
            self.consume(TokenType::RightParen, "Expected ')' after expression")?;
            return Ok(Expr::Grouping(Box::new(expr)));
        }

        error!(
            ReportKind::SyntaxError,
            "Expected expression",
            self.peek().location()
        )
    }

    fn match_any_token(&mut self, token_types: Vec<TokenType>) -> bool {
        for token_type in token_types {
            if self.match_token(token_type) {
                return true;
            }
        }

        false
    }

    fn match_token(&mut self, token_type: TokenType) -> bool {
        if self.check(token_type) {
            self.advance();
            return true;
        }

        false
    }

    fn consume(&mut self, token_type: TokenType, message: &str) -> Result<Token> {
        if self.check(token_type) {
            return Ok(self.advance());
        }

        error!(
            ReportKind::SyntaxError,
            &format!("{}", message),
            self.peek().location()
        )
    }

    fn check(&self, token_type: TokenType) -> bool {
        self.peek().token_type() == token_type
    }

    fn advance(&mut self) -> Token {
        if !self.is_at_end() {
            self.current += 1;
        }

        self.previous()
    }

    fn is_at_end(&self) -> bool {
        self.peek().token_type() == TokenType::EOF
    }

    fn peek(&self) -> Token {
        self.tokens[self.current].clone()
    }

    fn previous(&self) -> Token {
        self.tokens[self.current - 1].clone()
    }

    fn set_current(&mut self, current: usize) {
        self.current = current;
    }
}
