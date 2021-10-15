use lexer::literal::Literal;
use lexer::token::Token;
use parser::ast::Expr;
use std::convert::TryFrom;
use utils::errors::InterpreterError;

pub type InterpreterResult<T> = Result<T, InterpreterError>;

pub struct Interpreter {}

impl Default for Interpreter {
    fn default() -> Self {
        Interpreter {}
    }
}

impl Interpreter {
    pub fn evaluate(&self, expr: Expr) -> InterpreterResult<Literal> {
        match expr {
            Expr::Literal(l) => Ok(l),
            Expr::Grouping(e) => self.evaluate(*e),
            Expr::Unary(operator, right) => self.unary_expr(operator, *right),
            Expr::Binary(left, operator, right) => self.binary_expr(*left, operator, *right),
        }
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
