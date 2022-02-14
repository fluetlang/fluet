/*
 * Copyright (C) 2022 Umut İnan Erdoğan <umutinanerdogan@pm.me>
 * 
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use common::{token::{Token, TokenType, Literal}, util, keywords::KEYWORDS, errors::{report_error, ReportKind}};

pub struct Lexer {
    source: String,
    filename: String,
    lines: Vec<String>,
    start: usize,
    current: usize,
    row: usize,
    tokens: Vec<Token>,
}

impl Lexer {
    pub fn new(source: String, filename: String) -> Self {
        Self {
            source: source.clone(),
            filename,
            lines: source.lines().map(|s| s.to_string()).collect(),
            start: 0,
            current: 0,
            row: 1,
            tokens: vec![],
        }
    }

    pub fn scan_tokens(&mut self) -> &Vec<Token> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }

        self.tokens.push(Token::new(
            TokenType::EOF,
            "".to_string(),
            self.filename.clone(),
            self.source.clone(),
            self.row,
            None
        ));
        &self.tokens
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn scan_token(&mut self) {
        match self.advance() {
            // Single-character tokens
            Some('(') => self.add_token(TokenType::LeftParen, None),
            Some(')') => self.add_token(TokenType::RightParen, None),
            Some('{') => self.add_token(TokenType::LeftBrace, None),
            Some('}') => self.add_token(TokenType::RightBrace, None),
            Some(',') => self.add_token(TokenType::Comma, None),
            Some('-') => self.add_token(TokenType::Minus, None),
            Some('+') => self.add_token(TokenType::Plus, None),
            Some(';') => self.add_token(TokenType::Semicolon, None),
            Some('*') => self.add_token(TokenType::Star, None),

            // One or two character tokens
            Some('!') => {
                let token = if self.match_char('=') {
                    TokenType::BangEqual
                } else {
                    TokenType::Bang
                };

                self.add_token(token, None);
            },
            Some('=') => {
                let token = if self.match_char('=') {
                    TokenType::EqualEqual
                } else {
                    TokenType::Equal
                };

                self.add_token(token, None);
            },
            Some('<') => {
                let token = if self.match_char('=') {
                    TokenType::LessEqual
                } else {
                    TokenType::Less
                };

                self.add_token(token, None);
            },
            Some('>') => {
                let token = if self.match_char('=') {
                    TokenType::GreaterEqual
                } else {
                    TokenType::Greater
                };

                self.add_token(token, None);
            },
            Some(':') => {
                let token = if self.match_char(':') {
                    TokenType::ColonColon
                } else {
                    TokenType::Colon
                };

                self.add_token(token, None);
            },
            Some('&') => {
                let token = if self.match_char('&') {
                    TokenType::LogicalAnd
                } else {
                    TokenType::BitwiseAnd
                };

                self.add_token(token, None);
            },
            Some('|') => {
                let token = if self.match_char('|') {
                    TokenType::LogicalOr
                } else {
                    TokenType::BitwiseOr
                };

                self.add_token(token, None);
            },
            Some('/') => {
                if self.match_char('/') {
                    // A comment goes until the end of the line.
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else if self.match_char('*') {
                    let mut nesting_counter = 0;
                    loop {
                        if self.peek() == '/' && self.peek_nth(2) == '*' {
                            self.advance();
                            self.advance();
                            nesting_counter += 1;
                        } else if self.match_char('*') && self.match_char('/') {
                            if nesting_counter == 0 {
                                break;
                            }

                            nesting_counter -= 1;
                        } else {
                            self.advance();
                        }
                    }
                } else {
                    self.add_token(TokenType::Slash, None);
                }
            },

            Some('.') => self.dot(),
            Some('"') => self.string('"'),
            Some('\'') => self.string('\''),

            // Ignore whitespace
            Some(' ') | Some('\r') | Some('\t') => (),
            Some('\n') => self.row += 1,

            Some('0'..='9') => self.number(),
            Some('a'..='z' | 'A'..='Z' | '_') => self.identifier(),
            Some(c) => eprintln!("{}", report_error(
                ReportKind::SyntaxError,
                None,
                &format!("Unexpected character {} at line {}", c, self.row),
                &self.filename,
                "",
                0
            )),
            None => eprintln!("{}", report_error(
                ReportKind::SyntaxError,
                None,
                &format!("Unexpected character at line {}", self.row),
                &self.filename,
                "",
                0
            )),
        }
    }

    fn string(&mut self, quote: char) {
        while self.peek() != quote && !self.is_at_end() {
            if self.peek() == '\n' {
                self.row += 1;
            }

            self.advance();
        }

        if self.is_at_end() {
            eprintln!("Unterminated string at line {}", self.row);
            return;
        }

        // The closing quote
        self.advance();

        // Trim the surrounding quotes
        let value = self.source[self.start + 1..self.current - 1].to_string();
        self.add_token(TokenType::String, Some(Literal::String(value)));
    }

    fn dot(&mut self) {
        if util::is_digit(self.peek()) {
            while util::is_digit(self.peek()) {
                self.advance();
            }

            // Should always be a valid f64
            let number = self.source[self.start..self.current].parse().unwrap();
            self.add_token(TokenType::Number, Some(Literal::Number(number)));
        } else {
            self.add_token(TokenType::Dot, None);
        }
    }

    fn number(&mut self) {
        while util::is_digit(self.peek()) {
            self.advance();
        }

        // Look for the fractional part
        if self.peek() == '.' && util::is_digit(self.peek_nth(2)) {
            // Consume the "."
            self.advance();

            while util::is_digit(self.peek()) {
                self.advance();
            }
        }

        // Should always be a valid f64
        let number = self.source[self.start..self.current].parse().unwrap();
        self.add_token(TokenType::Number, Some(Literal::Number(number)));
    }

    fn identifier(&mut self) {
        while util::is_valid_identifier(self.peek()) {
            self.advance();
        }

        let text = &self.source[self.start..self.current];
        let token_type = KEYWORDS.get(text)
            .map_or(TokenType::Identifier, |&token_type| token_type);
        
        let literal = match token_type {
            TokenType::True => Some(Literal::Bool(true)),
            TokenType::False => Some(Literal::Bool(false)),
            TokenType::Number => {
                let number = text.parse().unwrap();
                Some(Literal::Number(number))
            },
            TokenType::Null => Some(Literal::Null),
            _ => None
        };

        self.add_token(token_type, literal);
    }

    fn advance(&mut self) -> Option<char> {
        let current = self.source.chars().nth(self.current);
        self.current += 1;
        current
    }

    fn match_char(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }

        if self.source.chars().nth(self.current) != Some(expected) {
            return false;
        }

        self.current += 1;
        true
    }

    fn peek(&mut self) -> char {
        self.peek_nth(1)
    }

    fn peek_nth(&mut self, chars: usize) -> char {
        if self.current + chars - 1 >= self.source.len() {
            return '\0';
        }

        // Should always be Some
        self.source.chars().nth(self.current + chars - 1).unwrap()
    }

    fn add_token(&mut self, token_type: TokenType, literal: Option<Literal>) {
        let text = self.source[self.start..self.current].to_string();
        self.tokens.push(
            Token::new(
                token_type,
                text,
                self.filename.clone(),
                self.lines[self.row - 1].to_string(),
                self.row,
                literal
            )
        );
    }
}
