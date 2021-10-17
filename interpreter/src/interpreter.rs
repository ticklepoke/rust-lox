use frontend::ast::{Expr, Stmt};
use frontend::environment::Environment;
use frontend::literal::{Literal, TryFromWrapper};
use frontend::runnable::Runnable;
use frontend::token::{Token, TokenType};
use std::convert::TryFrom;
use utils::errors::InterpreterError;

pub type InterpreterResult<T> = Result<T, InterpreterError>;

pub struct Interpreter {
    environment: Environment,
}

impl Default for Interpreter {
    fn default() -> Self {
        Interpreter {
            environment: Environment::new(None),
        }
    }
}

impl Runnable for Interpreter {
    fn block(&mut self, body: Vec<Stmt>, e: Environment) -> InterpreterResult<()> {
        self.environment = e;
        self.interpret(body)?;
        self.environment = *self.environment.clone().enclosing.unwrap();
        Ok(())
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
                Stmt::Block(stmts) => self.block(
                    stmts,
                    Environment::new(Some(Box::new(self.environment.clone()))),
                )?,
                Stmt::If(condition, consequent, alternative) => {
                    self.if_statement(condition, *consequent, alternative.map(|alt| *alt))?
                }
                Stmt::While(condition, body) => self.while_statement(condition, *body)?,
            }
        }
        Ok(())
    }

    fn evaluate(&mut self, expr: Expr) -> InterpreterResult<Literal> {
        match expr {
            Expr::Literal(l) => Ok(l),
            Expr::Grouping(e) => self.evaluate(*e),
            Expr::Unary(operator, right) => self.unary_expr(operator, *right),
            Expr::Binary(left, operator, right) => self.binary_expr(*left, operator, *right),
            Expr::Variable(name) => self.var_expression(name),
            Expr::Assign(name, init) => self.assignment_expression(name, *init),
            Expr::Logical(left, operator, right) => {
                self.logical_expression(*left, operator, *right)
            }
            Expr::Call(callee, _paren, args) => self.call_expression(*callee, args),
        }
    }

    fn call_expression(&mut self, callee: Expr, args: Vec<Expr>) -> InterpreterResult<Literal> {
        let function = match self.evaluate(callee)? {
            Literal::Callable(c) => c,
            _ => return Err(InterpreterError::InvalidAstType),
        };
        let mut arg_literals = Vec::new();
        for arg in args {
            arg_literals.push(self.evaluate(arg)?);
        }

        if arg_literals.len() != function.arity() {
            return Err(InterpreterError::MismatchFunctionArity);
        }
        function
            .call(self, arg_literals)
            .map_err(|_| InterpreterError::InvalidAstType)
    }

    fn while_statement(&mut self, condition: Expr, body: Stmt) -> InterpreterResult<()> {
        while bool::from(self.evaluate(condition.clone())?) {
            self.interpret(vec![body.clone()])?;
        }
        Ok(())
    }

    fn if_statement(
        &mut self,
        condition: Expr,
        consequent: Stmt,
        alternative: Option<Stmt>,
    ) -> InterpreterResult<()> {
        if bool::from(self.evaluate(condition)?) {
            self.interpret(vec![consequent])?;
        } else if let Some(alt) = alternative {
            self.interpret(vec![alt])?;
        }
        Ok(())
    }

    fn logical_expression(
        &mut self,
        left: Expr,
        operator: Token,
        right: Expr,
    ) -> InterpreterResult<Literal> {
        let left = self.evaluate(left)?;
        if operator.token_type == TokenType::Or {
            if bool::from(left.clone()) {
                return Ok(left);
            }
        } else if !bool::from(left.clone()) {
            return Ok(left);
        }
        self.evaluate(right)
    }

    fn assignment_expression(&mut self, name: Token, init: Expr) -> InterpreterResult<Literal> {
        let value = self.evaluate(init)?;
        if let Some(name) = name.lexeme {
            let assign_result = self.environment.assign(name, value.clone());
            return assign_result.map(|()| value);
        }
        Ok(value)
    }

    fn print_statement(&mut self, expr: Expr) -> InterpreterResult<()> {
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

    fn unary_expr(&mut self, operator: Token, right: Expr) -> InterpreterResult<Literal> {
        let right = self.evaluate(right)?;
        use frontend::token::TokenType::*;
        match operator.token_type {
            Minus => Ok(Literal::Number(-(f64::try_from(right)?))),
            Bang => Ok(Literal::Boolean(!(bool::try_from(TryFromWrapper(right))?))),
            _ => Err(InterpreterError::InvalidAstType),
        }
    }

    fn binary_expr(
        &mut self,
        left: Expr,
        operator: Token,
        right: Expr,
    ) -> InterpreterResult<Literal> {
        let left = self.evaluate(left)?;
        let right = self.evaluate(right)?;

        use frontend::token::TokenType::*;
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
