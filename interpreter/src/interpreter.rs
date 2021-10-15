use crate::environment::Environment;
use lexer::literal::Literal;
use lexer::token::Token;
use parser::ast::{Expr, Stmt};
use std::convert::TryFrom;
use utils::errors::InterpreterError;

pub type InterpreterResult<T> = Result<T, InterpreterError>;

pub struct Interpreter {
    environment: Environment,
}

impl Default for Interpreter {
    fn default() -> Self {
        Interpreter {
            environment: Environment::new(),
        }
    }
}

impl Interpreter {
    pub fn new(e: Environment) -> Self {
        Interpreter { environment: e }
    }

    pub fn interpret(&mut self, stmts: Vec<Stmt>) -> InterpreterResult<()> {
        for stmt in stmts {
            match stmt {
                Stmt::Expr(e) => {
                    self.evaluate(e)?;
                }
                Stmt::Print(e) => {
                    self.print_statement(e)?;
                }
                Stmt::Var(name, init) => self.var_statement(name, init)?,
            }
        }
        Ok(())
    }

    fn evaluate(&self, expr: Expr) -> InterpreterResult<Literal> {
        match expr {
            Expr::Literal(l) => Ok(l),
            Expr::Grouping(e) => self.evaluate(*e),
            Expr::Unary(operator, right) => self.unary_expr(operator, *right),
            Expr::Binary(left, operator, right) => self.binary_expr(*left, operator, *right),
            Expr::Variable(name) => self.var_expression(name),
        }
    }

    fn print_statement(&self, expr: Expr) -> InterpreterResult<()> {
        let value = self.evaluate(expr)?;
        println!("{}", value);
        Ok(())
    }

    fn var_statement(&mut self, name: Token, init: Option<Expr>) -> InterpreterResult<()> {
        let mut value = None;
        if init.is_some() {
            value = Some(self.evaluate(init.unwrap())?);
        }

        if let Some(name) = name.lexeme {
            match value {
                Some(v) => self.environment.define(name, v),
                None => self.environment.define(name, Literal::Nil),
            }
        }
        Ok(())
    }

    fn var_expression(&self, name: Token) -> InterpreterResult<Literal> {
        let name = name.lexeme.expect("Expected lexeme for variable lookup");
        let value = self.environment.get(name);
        if let Some(value) = value {
            return Ok(value);
        }
        Ok(Literal::Nil)
    }

    fn unary_expr(&self, operator: Token, right: Expr) -> InterpreterResult<Literal> {
        let right = self.evaluate(right)?;
        use lexer::token::TokenType::*;
        match operator.token_type {
            Minus => Ok(Literal::Number(-(f64::try_from(right)?))),
            Bang => Ok(Literal::Boolean(!(bool::try_from(right)?))),
            _ => Err(InterpreterError::InvalidAstType),
        }
    }

    fn binary_expr(&self, left: Expr, operator: Token, right: Expr) -> InterpreterResult<Literal> {
        let left = self.evaluate(left)?;
        let right = self.evaluate(right)?;

        use lexer::token::TokenType::*;
        match operator.token_type {
            Minus => {
                let left = f64::try_from(left)?;
                let right = f64::try_from(right)?;
                Ok(Literal::Number(left - right))
            }
            Slash => {
                let left = f64::try_from(left)?;
                let right = f64::try_from(right)?;
                Ok(Literal::Number(left / right))
            }
            Star => {
                let left = f64::try_from(left)?;
                let right = f64::try_from(right)?;
                Ok(Literal::Number(left * right))
            }
            Plus => match (left, right) {
                (Literal::String(l), Literal::String(r)) => {
                    Ok(Literal::String(format!("{}{}", l, r)))
                }
                (Literal::Number(l), Literal::Number(r)) => Ok(Literal::Number(l + r)),
                _ => Err(InterpreterError::InvalidAstType),
            },
            Greater => Ok(Literal::Boolean(left > right)),
            GreaterEqual => Ok(Literal::Boolean(left >= right)),
            Less => Ok(Literal::Boolean(left < right)),
            LessEqual => Ok(Literal::Boolean(left <= right)),
            EqualEqual => Ok(Literal::Boolean(left == right)),
            BangEqual => Ok(Literal::Boolean(left != right)),
            _ => Err(InterpreterError::InvalidAstType),
        }
    }
}
