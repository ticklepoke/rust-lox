use crate::interpreter::Interpreter;
use frontend::ast::{Expr, Stmt};
use frontend::token::Token;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

enum ResolverError {
    UndefinedVariable,
}

type ResolverResult<T> = Result<T, ResolverError>;

#[allow(dead_code)]
pub struct Resolver {
    interpreter: Rc<RefCell<Interpreter>>,
    scopes: Vec<HashMap<String, bool>>,
}

#[allow(dead_code)]
impl Resolver {
    pub fn new(interpreter: Rc<RefCell<Interpreter>>) -> Self {
        // TODO check if inrepreter should be a ref
        Resolver {
            interpreter,
            scopes: Vec::new(),
        }
    }

    fn resolve_stmts(&mut self, stmts: Vec<Stmt>) -> ResolverResult<()> {
        for s in stmts {
            self.resolve_stmt(s)?;
        }
        Ok(())
    }

    // TODO check if we can collapse this
    fn resolve_stmt(&mut self, stmt: Stmt) -> ResolverResult<()> {
        match stmt {
            Stmt::Block(stmts) => self.block(stmts),
            Stmt::Var(name, init) => self.var_stmt(name, init),
            Stmt::Function(ref name, args, body) => self.function_stmt(name, args, body),
            Stmt::Expr(ref expr) => self.resolve_expr(expr),
            Stmt::If(ref condition, consequent, alternate) => {
                self.if_stmt(condition, consequent, alternate)
            }
            Stmt::Print(ref expr) => self.resolve_expr(expr),
            Stmt::Return(_name, expr) => expr.map(|e| self.resolve_expr(&e)).unwrap_or(Ok(())),
            Stmt::While(ref condition, body) => {
                self.resolve_expr(condition)?;
                self.resolve_stmt(*body)?;
                Ok(())
            }
        }
    }

    fn resolve_expr(&self, expr: &Expr) -> ResolverResult<()> {
        match expr {
            Expr::Variable(ref name) => self.var_expr(&expr, name),
            Expr::Assign(ref name, ref init) => self.assign_expr(&expr, name, init),
            Expr::Binary(left, _operator, right) => {
                self.resolve_expr(left)?;
                self.resolve_expr(right)?;
                Ok(())
            }
            Expr::Call(callee, _paren, args) => {
                self.resolve_expr(callee)?;
                for a in args {
                    self.resolve_expr(a)?;
                }
                Ok(())
            }
            Expr::Grouping(expr) => self.resolve_expr(expr),
            Expr::Literal(_literal) => Ok(()),
            Expr::Logical(left, _op, right) => {
                self.resolve_expr(left)?;
                self.resolve_expr(right)?;
                Ok(())
            }
            Expr::Unary(_op, right) => self.resolve_expr(right),
        }
    }

    fn resolve_local(&self, _expr: &Expr, name: &Token) -> ResolverResult<()> {
        for i in (0..self.scopes.len() - 1).rev() {
            if let Some(name_str) = name.lexeme.clone() {
                if let Some(curr_scope) = self.scopes.get(i) {
                    if curr_scope.contains_key(name_str.as_str()) {
                        // interpreter.resolve(expr, self.scopes.len() -1 -i);
                    }
                }
            }
        }
        Ok(())
    }

    fn block(&mut self, body: Vec<Stmt>) -> ResolverResult<()> {
        self.begin_scope();
        self.resolve_stmts(body)?;
        self.end_scope();
        Ok(())
    }

    fn var_stmt(&mut self, name: Token, init: Option<Expr>) -> ResolverResult<()> {
        self.declare(&name);
        if let Some(init) = init {
            self.resolve_expr(&init)?;
        }
        self.define(&name);
        Ok(())
    }

    fn var_expr(&self, expr: &Expr, name: &Token) -> ResolverResult<()> {
        if !self.scopes.is_empty() {
            if let Some(last) = self.scopes.last() {
                if let Some(res) = last.get(&name.lexeme.clone().unwrap()) {
                    if !(*res) {
                        return Err(ResolverError::UndefinedVariable);
                    }
                }
            }
        }
        self.resolve_local(expr, name)?;
        Ok(())
    }

    fn assign_expr(&self, expr: &Expr, name: &Token, init: &Expr) -> ResolverResult<()> {
        self.resolve_expr(init)?;
        self.resolve_local(expr, name)?;
        Ok(())
    }

    fn function_stmt(
        &mut self,
        name: &Token,
        params: Vec<Token>,
        body: Vec<Stmt>,
    ) -> ResolverResult<()> {
        self.declare(name);
        self.define(name);
        self.resolve_function(params, body)?;
        Ok(())
    }

    fn resolve_function(&mut self, params: Vec<Token>, body: Vec<Stmt>) -> ResolverResult<()> {
        self.begin_scope();
        for p in params {
            self.declare(&p);
            self.define(&p);
        }
        self.resolve_stmts(body)?;
        self.end_scope();
        Ok(())
    }

    fn if_stmt(
        &mut self,
        condition: &Expr,
        consequent: Box<Stmt>,
        alternate: Option<Box<Stmt>>,
    ) -> ResolverResult<()> {
        self.resolve_expr(condition)?;
        self.resolve_stmt(*consequent)?;
        if let Some(alt) = alternate {
            self.resolve_stmt(*alt)?;
        }
        Ok(())
    }

    // UTILS
    fn declare(&mut self, name: &Token) {
        if self.scopes.is_empty() {
            return;
        }

        let scope = self.scopes.last_mut().unwrap();
        if let Some(lexeme) = &name.lexeme {
            scope.insert(lexeme.to_string(), false);
        }
    }

    fn define(&mut self, name: &Token) {
        if self.scopes.is_empty() {
            return;
        }

        if let Some(top) = self.scopes.last_mut() {
            if let Some(lexeme) = &name.lexeme {
                top.insert(lexeme.to_string(), true);
            }
        }
    }

    fn begin_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    fn end_scope(&mut self) {
        self.scopes.pop();
    }
}
