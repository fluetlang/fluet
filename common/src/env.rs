use std::collections::HashMap;

use crate::{value::Value, token::Token, error};
use crate::errors::{ReportKind, Result};

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

    pub fn from_parent(parent: Env) -> Self {
        Self {
            parent: Some(Box::new(parent)),
            values: HashMap::new(),
        }
    }

    pub fn get(&self, name: Token) -> Result<&Value> {
        match self.values.get(name.lexeme()) {
            Some(value) => Ok(value),
            None => {
                if let Some(parent) = &self.parent {
                    return Ok(parent.get(name)?);
                }

                error!(
                    ReportKind::RuntimeError,
                    &format!("{} is not defined", name),
                    name.filename(),
                    name.line(),
                    name.row()
                )
            },
        }
    }
    
    pub fn define(&mut self, name: String, value: Value) {
        self.values.insert(name, value);
    }

    pub fn assign(&mut self, name: Token, value: &Value) -> Result<()> {
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
            name.filename(),
            name.line(),
            name.row()
        )
    }
}
