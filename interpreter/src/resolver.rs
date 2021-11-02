use crate::interpreter::Interpreter;
use frontend::ast::{Expr, Stmt};
use frontend::token::Token;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

pub enum ResolverError {
    UndefinedVariable,
    ExistingVariable,
    InvalidReturnStatement,
}

type ResolverResult<T> = Result<T, ResolverError>;

#[derive(Clone)]
enum FunctionType {
    None,
    Function,
}

pub struct Resolver {
    interpreter: Rc<RefCell<Interpreter>>,
    scopes: Vec<HashMap<String, bool>>,
    current_function: FunctionType,
}

#[allow(dead_code)]
impl Resolver {
    pub fn new(interpreter: Rc<RefCell<Interpreter>>) -> Self {
        Resolver {
            interpreter,
            scopes: Vec::new(),
            current_function: FunctionType::None,
        }
    }

    pub fn resolve_stmts(&mut self, stmts: &[Stmt]) -> ResolverResult<()> {
        for s in stmts {
            self.resolve_stmt(s)?;
        }
        Ok(())
    }

    fn resolve_stmt(&mut self, stmt: &Stmt) -> ResolverResult<()> {
        match stmt {
            Stmt::Block(stmts) => self.block(stmts),
            Stmt::Var(name, init) => self.var_stmt(name, init),
            Stmt::Function(ref name, args, body) => self.function_stmt(name, args, body),
            Stmt::Expr(ref expr) => self.resolve_expr(expr),
            Stmt::If(ref condition, consequent, alternate) => {
                self.if_stmt(condition, consequent, alternate)
            }
            Stmt::Print(ref expr) => self.resolve_expr(expr),
            Stmt::Return(_name, expr) => {
                if let FunctionType::None = self.current_function {
                    return Err(ResolverError::InvalidReturnStatement);
                }
                expr.as_ref()
                    .map(|e| self.resolve_expr(e))
                    .unwrap_or(Ok(()))
            }
            Stmt::While(ref condition, body) => {
                self.resolve_expr(condition)?;
                self.resolve_stmt(body)?;
                Ok(())
            }
            Stmt::Class(ref name, _fns) => {
                self.declare(name)?;
                self.define(name);
                Ok(())
            }
        }
    }

    fn resolve_expr(&self, expr: &Expr) -> ResolverResult<()> {
        match expr {
            Expr::Variable(ref name) => self.var_expr(expr, name),
            Expr::Assign(ref name, ref init) => self.assign_expr(expr, name, init),
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
            Expr::Literal(_literal) => Ok(()), // No op, we do not need to resolve literals
            Expr::Logical(left, _op, right) => {
                self.resolve_expr(left)?;
                self.resolve_expr(right)?;
                Ok(())
            }
            Expr::Unary(_op, right) => self.resolve_expr(right),
            Expr::Get(object, _name) => self.resolve_expr(object),
        }
    }

    fn resolve_local(&self, expr: &Expr, name: &Token) -> ResolverResult<()> {
        for i in (0..self.scopes.len()).rev() {
            if let Some(name_str) = name.lexeme.clone() {
                if let Some(curr_scope) = self.scopes.get(i) {
                    if curr_scope.contains_key(name_str.as_str()) {
                        self.interpreter
                            .borrow_mut()
                            .resolve(expr.clone(), self.scopes.len() - 1 - i);
                    }
                }
            }
        }
        Ok(())
    }

    fn block(&mut self, body: &[Stmt]) -> ResolverResult<()> {
        self.begin_scope();
        self.resolve_stmts(body)?;
        self.end_scope();
        Ok(())
    }

    fn var_stmt(&mut self, name: &Token, init: &Option<Expr>) -> ResolverResult<()> {
        self.declare(name)?;
        if let Some(init) = init {
            self.resolve_expr(init)?;
        }
        self.define(name);
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
        params: &[Token],
        body: &[Stmt],
    ) -> ResolverResult<()> {
        self.declare(name)?;
        self.define(name);
        self.resolve_function(params, body, FunctionType::Function)?;
        Ok(())
    }

    fn resolve_function(
        &mut self,
        params: &[Token],
        body: &[Stmt],
        f_type: FunctionType,
    ) -> ResolverResult<()> {
        let enclosing_function = self.current_function.clone();
        self.current_function = f_type;
        self.begin_scope();
        for p in params {
            self.declare(p)?;
            self.define(p);
        }
        self.resolve_stmts(body)?;
        self.end_scope();
        self.current_function = enclosing_function;
        Ok(())
    }

    fn if_stmt(
        &mut self,
        condition: &Expr,
        consequent: &Stmt,
        alternate: &Option<Box<Stmt>>,
    ) -> ResolverResult<()> {
        self.resolve_expr(condition)?;
        self.resolve_stmt(consequent)?;
        if let Some(alt) = alternate {
            self.resolve_stmt(alt)?;
        }
        Ok(())
    }

    // UTILS
    fn declare(&mut self, name: &Token) -> ResolverResult<()> {
        if self.scopes.is_empty() {
            return Ok(());
        }

        let scope = self.scopes.last_mut().unwrap();
        if let Some(lexeme) = &name.lexeme {
            if scope.contains_key(lexeme) {
                return Err(ResolverError::ExistingVariable);
            }
            scope.insert(lexeme.to_string(), false);
        }
        Ok(())
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
