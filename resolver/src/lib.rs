use std::{collections::HashMap, mem};

use common::{stmt::Stmt, errors::{Result, ReportKind}, expr::Expr, token::Token, error};
use interpreter::Interpreter;

pub struct Resolver {
    scopes: Vec<HashMap<String, bool>>,
    locals: HashMap<usize, usize>,
}

impl Resolver {
    pub fn new() -> Self {
        Self {
            scopes: Vec::new(),
            locals: HashMap::new(),
        }
    }

    pub fn resolve(&mut self,
        stmts: &Vec<Stmt>,
        interpreter: &mut Interpreter,
    ) -> Result<()>{
        self.resolve_stmts(stmts)?;
        interpreter.extend_locals(mem::take(&mut self.locals));
        Ok(())
    }

    pub fn resolve_stmts(&mut self, stmts: &Vec<Stmt>) -> Result<()> {
        for stmt in stmts {
            self.resolve_stmt(stmt)?;
        }

        Ok(())
    }

    fn resolve_stmt(&mut self, stmt: &Stmt) -> Result<()> {
        match stmt {
            Stmt::Expr(expr) => self.resolve_expr(expr),
            Stmt::Fn(name, args, body, ret) => {
                self.declare(&name);
                self.define(&name);

                self.begin_scope();
                for arg in args {
                    self.declare(arg);
                    self.define(arg);
                }

                self.resolve_stmts(body)?;
                self.resolve_expr(ret)?;
                Ok(())
            },
            Stmt::Let(name, expr) => {
                self.declare(&name);
                self.resolve_expr(expr)?;
                self.define(&name);
                Ok(())
            },
            Stmt::Loop(body) => {
                self.begin_scope();
                self.resolve_stmts(body)?;
                self.end_scope();
                Ok(())
            },
            Stmt::Return(expr) => self.resolve_expr(expr),
            Stmt::While(cond, body) => {
                self.resolve_expr(cond)?;
                self.begin_scope();
                self.resolve_stmts(body)?;
                self.end_scope();
                Ok(())
            },
        }
    }

    fn resolve_expr(&mut self, expr: &Expr) -> Result<()> {
        match expr {
            Expr::Assignment(expr_id, name, value) => {
                self.resolve_expr(value)?;
                println!("LOOOOL");
                self.resolve_local(*expr_id, name);
                Ok(())
            },
            Expr::Binary(lhs, _, rhs) => {
                self.resolve_expr(lhs)?;
                self.resolve_expr(rhs)?;
                Ok(())
            },
            Expr::Block(stmts, expr) => {
                self.begin_scope();
                self.resolve_expr(expr)?;
                self.resolve_stmts(stmts)?;
                self.end_scope();
                Ok(())
            },
            Expr::Call(callee, _, args) => {
                self.resolve_expr(callee)?;
                for arg in args {
                    self.resolve_expr(arg)?;
                }

                Ok(())
            },
            Expr::Grouping(expr) => self.resolve_expr(expr),
            Expr::If(cond, then_branch, else_branch) => {
                self.resolve_expr(cond)?;
                self.resolve_expr(then_branch)?;
                self.resolve_expr(else_branch)?;
                Ok(())
            },
            Expr::Literal(_) => Ok(()),
            Expr::Logical(lhs, _, rhs) => {
                self.resolve_expr(lhs)?;
                self.resolve_expr(rhs)?;
                Ok(())
            },
            Expr::Unary(_, expr) => self.resolve_expr(expr),
            Expr::Variable(expr_id, name) => {
                if !self.scopes.is_empty() && self.scopes.last().unwrap().contains_key(name.lexeme()) {
                    return error!(
                        ReportKind::SyntaxError,
                        &format!("Cannot read local variable {} in its own initializer.", name.lexeme()),
                        name.location()
                    );
                }

                println!("LMAO");
                self.resolve_local(*expr_id, name);
                Ok(())
            },
        }
    }

    fn resolve_local(&mut self, expr_id: usize, name: &Token) {
        println!("Resolving local {} id: {}", name.lexeme(), expr_id);
        println!("Scopes: {:#?}", self.scopes);
        for (i, scope) in self.scopes.iter().rev().enumerate() {
            println!("Resolving local {} id: {}", name.lexeme(), expr_id);
            if scope.contains_key(name.lexeme()) {
                println!("Resolving local {} id: {}", name.lexeme(), expr_id);
                self.locals.insert(expr_id, self.scopes.len() - 1 - i);
                return;
            }
        }
    }

    fn declare(&mut self, name: &Token) {
        if self.scopes.is_empty() {
            return;
        }

        let scope = self.scopes.last_mut().unwrap();
        scope.insert(name.lexeme().to_string(), false);
    }

    fn define(&mut self, name: &Token) {
        if self.scopes.is_empty() {
            return;
        }

        let scope = self.scopes.last_mut().unwrap();
        scope.insert(name.lexeme().to_string(), true);
    }

    fn begin_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    fn end_scope(&mut self) {
        self.scopes.pop();
    }
}
