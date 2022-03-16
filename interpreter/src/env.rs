/*
 * Copyright (C) 2022 Umut İnan Erdoğan <umutinanerdogan@pm.me>
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use common::errors::{ReportKind, Result};
use common::{error, token::Token};

use crate::value::Value;

#[derive(Clone, Debug)]
pub struct Env {
    parent: Option<Rc<RefCell<Env>>>,
    pub values: HashMap<String, Value>,
}

impl Env {
    pub fn new() -> Self {
        Self {
            parent: None,
            values: HashMap::new(),
        }
    }

    pub fn from_parent(parent: Rc<RefCell<Env>>) -> Self {
        Self {
            parent: Some(parent),
            values: HashMap::new(),
        }
    }

    pub fn get_at(&self, distance: usize, name: &Token) -> Option<Value> {
        self.ancestor(distance).borrow().values.get(name.lexeme()).map(|val| val.clone())
    }

    fn ancestor(&self, distance: usize) -> Rc<RefCell<Env>> {
        if distance == 0 {
            return Rc::new(RefCell::new(self.clone()));
        }

        match &self.parent {
            Some(parent) => parent.borrow().ancestor(distance - 1),
            None => unreachable!("ancestor called with non-zero distance but no parent"),
        }
    }

    pub fn define(&mut self, name: String, value: Value) {
        self.values.insert(name, value);
    }

    pub fn assign_at(&mut self, distance: usize, name: &Token, value: Value) {
        self.ancestor(distance).borrow_mut().values.insert(
            name.lexeme().to_string(),
            value
        );
    }

    pub fn assign(&mut self, name: &Token, value: &Value) -> Result<()> {
        let lexeme = name.lexeme();
        if self.values.contains_key(lexeme) {
            self.values.insert(lexeme.to_string(), value.clone());
            return Ok(());
        }

        error!(
            ReportKind::RuntimeError,
            &format!("{} is not defined.", name),
            name.location()
        )
    }

    pub fn parent(&self) -> Option<Rc<RefCell<Env>>> {
        self.parent.clone()
    }
}
