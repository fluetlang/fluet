/*
 * Copyright (C) 2022 Umut İnan Erdoğan <umutinanerdogan@pm.me>
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use std::collections::HashMap;

use crate::token::TokenType;

lazy_static! {
    pub static ref KEYWORDS: HashMap<&'static str, TokenType> = {
        let mut keywords = HashMap::new();
        keywords.insert("and",      TokenType::LogicalAnd);
        keywords.insert("class",    TokenType::Class);
        keywords.insert("const",    TokenType::Const);
        keywords.insert("else",     TokenType::Else);
        keywords.insert("enum",     TokenType::Enum);
        keywords.insert("false",    TokenType::False);
        keywords.insert("for",      TokenType::For);
        keywords.insert("function", TokenType::Function);
        keywords.insert("if",       TokenType::If);
        keywords.insert("inf",      TokenType::Number);
        keywords.insert("let",      TokenType::Let);
        keywords.insert("loop",     TokenType::Loop);
        keywords.insert("match",    TokenType::Match);
        keywords.insert("module",   TokenType::Module);
        keywords.insert("NaN",      TokenType::Number);
        keywords.insert("null",     TokenType::Null);
        keywords.insert("or",       TokenType::LogicalOr);
        keywords.insert("return",   TokenType::Return);
        keywords.insert("super",    TokenType::Super);
        keywords.insert("then",     TokenType::Then);
        keywords.insert("this",     TokenType::This);
        keywords.insert("true",     TokenType::True);
        keywords.insert("while",    TokenType::While);
        keywords
    };
}
