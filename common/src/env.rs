/*
 * Copyright (C) 2022 Umut İnan Erdoğan <umutinanerdogan@pm.me>
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use std::collections::HashMap;

use crate::errors::{ReportKind, Result};
use crate::{error, token::Token, value::Value};

#[derive(Debug, Clone)]
pub struct Env {
    parent: Option<Box<Env>>,
    values: HashMap<String, Value>,
}

impl Env {
    pub fn new() -> Self {
        Self {
            parent: None,
            values: HashMap::new(),
        }
    }

    pub fn from_parent(parent: Box<Env>) -> Self {
        Self {
            parent: Some(parent),
            values: HashMap::new(),
        }
    }

    pub fn get(&self, name: &Token) -> Result<&Value> {
        match self.values.get(name.lexeme()) {
            Some(value) => Ok(value),
            None => {
                if let Some(parent) = &self.parent {
                    return Ok(parent.get(name)?);
                }

                error!(
                    ReportKind::RuntimeError,
                    &format!("{} is not defined", name),
                    name.location()
                )
            }
        }
    }

    pub fn define(&mut self, name: String, value: Value) {
        self.values.insert(name, value);
    }

    pub fn assign(&mut self, name: &Token, value: &Value) -> Result<()> {
        let lexeme = name.lexeme();
        if self.values.contains_key(lexeme) {
            self.values.insert(lexeme.to_string(), value.clone());
            return Ok(());
        }

        if let Some(parent) = &mut self.parent {
            parent.assign(name, value)?;
            return Ok(());
        }

        error!(
            ReportKind::RuntimeError,
            &format!("{} is not defined", name),
            name.location()
        )
    }

    pub fn parent(&self) -> Option<&Box<Env>> {
        self.parent.as_ref()
    }
}
