/*
 * Copyright (C) 2022 Umut İnan Erdoğan <umutinanerdogan@pm.me>
 * 
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use std::fmt;

#[derive(Debug, Clone)]
pub enum Literal {
    Number(f64),
    String(String),
    Bool(bool),
    Null,
}

#[derive(Debug, Clone)]
pub struct Token {
    token_type: TokenType,
    lexeme: String,
    filename: String,
    line: String,
    row: usize,
    literal: Option<Literal>,
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: String, filename: String, line: String, row: usize, literal: Option<Literal>) -> Self {
        Self {
            token_type,
            lexeme,
            filename,
            line,
            row,
            literal,
        }
    }

    pub fn token_type(&self) -> TokenType {
        self.token_type
    }

    pub fn lexeme(&self) -> &str {
        &self.lexeme
    }

    pub fn filename(&self) -> &str {
        &self.filename
    }

    pub fn line(&self) -> &str {
        &self.line
    }

    pub fn row(&self) -> usize {
        self.row
    }

    pub fn literal(&self) -> Option<&Literal> {
        self.literal.as_ref()
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.token_type)?;
        write!(f, " {}", self.lexeme)?;
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TokenType {
    // Single-character tokens
    LeftParen, RightParen, LeftBrace, RightBrace,
    Comma, Dot, Minus, Plus, Semicolon, Slash, Star,

    // One or two character tokens
    Bang, BangEqual,
    Equal, EqualEqual,
    Greater, GreaterEqual,
    Less, LessEqual,
    Colon, ColonColon,

    // Literals
    Identifier, String, Number,

    // Keywords
    BitwiseAnd, BitwiseOr, Class, Const, Else, Enum, False, For, Function, If,
    Let, Loop, LogicalAnd, LogicalOr, Match, Module, Null, Return, Super, This,
    True, While,

    EOF
}

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TokenType::LeftParen => write!(f, "("),
            TokenType::RightParen => write!(f, ")"),
            TokenType::LeftBrace => write!(f, "{{"),
            TokenType::RightBrace => write!(f, "}}"),
            TokenType::Comma => write!(f, ","),
            TokenType::Dot => write!(f, "."),
            TokenType::Minus => write!(f, "-"),
            TokenType::Plus => write!(f, "+"),
            TokenType::Semicolon => write!(f, ";"),
            TokenType::Slash => write!(f, "/"),
            TokenType::Star => write!(f, "*"),
            TokenType::Bang => write!(f, "!"),
            TokenType::BangEqual => write!(f, "!="),
            TokenType::Equal => write!(f, "="),
            TokenType::EqualEqual => write!(f, "=="),
            TokenType::Greater => write!(f, ">"),
            TokenType::GreaterEqual => write!(f, ">="),
            TokenType::Less => write!(f, "<"),
            TokenType::LessEqual => write!(f, "<="),
            TokenType::Colon => write!(f, ":"),
            TokenType::ColonColon => write!(f, "::"),
            TokenType::BitwiseAnd => write!(f, "&"),
            TokenType::BitwiseOr => write!(f, "|"),
            TokenType::Class => write!(f, "class"),
            TokenType::Else => write!(f, "else"),
            TokenType::Enum => write!(f, "enum"),
            TokenType::False => write!(f, "false"),
            TokenType::For => write!(f, "for"),
            TokenType::Function => write!(f, "function"),
            TokenType::If => write!(f, "if"),
            TokenType::Let => write!(f, "let"),
            TokenType::Loop => write!(f, "loop"),
            TokenType::LogicalAnd => write!(f, "&&"),
            TokenType::LogicalOr => write!(f, "||"),
            TokenType::Match => write!(f, "match"),
            TokenType::Module => write!(f, "module"),
            TokenType::Null => write!(f, "null"),
            TokenType::Return => write!(f, "return"),
            TokenType::Super => write!(f, "super"),
            TokenType::This => write!(f, "this"),
            TokenType::True => write!(f, "true"),
            TokenType::While => write!(f, "while"),
            _ => write!(f, "{:?}", self)
        }
    }
}
