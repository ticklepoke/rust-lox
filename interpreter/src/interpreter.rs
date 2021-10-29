use frontend::ast::{Expr, Stmt};
use frontend::environment::Environment;
use frontend::function::Function;
use frontend::literal::{Literal, TryFromWrapper};
use frontend::runnable::{EarlyReturn, Runnable};
use frontend::token::{Token, TokenType};
use std::cell::RefCell;
use std::collections::HashMap;
use std::convert::TryFrom;
use std::mem;
use std::rc::Rc;
use utils::errors::InterpreterError;

pub type InterpreterResult<T> = Result<T, EarlyReturn>;

pub struct Interpreter {
    pub environment: Rc<RefCell<Environment>>,
    pub globals: Rc<RefCell<Environment>>,
    locals: HashMap<Expr, usize>,
}

impl Default for Interpreter {
    fn default() -> Self {
        Interpreter::new(Environment::new(None))
    }
}

impl Runnable for Interpreter {
    fn block(&mut self, body: Vec<Stmt>, e: Rc<RefCell<Environment>>) -> InterpreterResult<()> {
        let previous = mem::replace(&mut self.environment, e);
        self.interpret(body)?;
        self.environment = previous;
        Ok(())
    }
}

impl Interpreter {
    pub fn new(e: Environment) -> Self {
        let globals = e.into_cell();
        let environment = Rc::clone(&globals);
        let locals = HashMap::new();
        Interpreter {
            globals,
            environment,
            locals,
        }
    }

    pub fn interpret(&mut self, stmts: Vec<Stmt>) -> InterpreterResult<()> {
        for stmt in stmts {
            match stmt {
                Stmt::Expr(e) => {
                    self.evaluate(&e)?;
                }
                Stmt::Print(e) => {
                    self.print_statement(e)?;
                }
                Stmt::Var(name, init) => self.var_statement(name, init)?,
                Stmt::Block(stmts) => self.block(
                    stmts,
                    Environment::new(Some(Rc::clone(&self.environment))).into_cell(),
                )?,
                Stmt::If(condition, consequent, alternative) => {
                    self.if_statement(condition, *consequent, alternative.map(|alt| *alt))?
                }
                Stmt::While(condition, body) => self.while_statement(condition, *body)?,
                Stmt::Function(name, params, body) => self.function(name, params, body)?,
                Stmt::Return(_return_keyword, return_value) => {
                    self.return_statement(return_value)?
                }
            }
        }
        Ok(())
    }

    fn evaluate(&mut self, expr: &Expr) -> InterpreterResult<Literal> {
        match *expr {
            Expr::Literal(ref l) => Ok(l.clone()),
            Expr::Grouping(ref e) => self.evaluate(e),
            Expr::Unary(ref operator, ref right) => self.unary_expr(operator, right),
            Expr::Binary(ref left, ref operator, ref right) => {
                self.binary_expr(left, operator, right)
            }
            Expr::Variable(ref name) => self.var_expression(expr, name),
            Expr::Assign(ref name, ref init) => self.assignment_expression(expr, name, init),
            Expr::Logical(ref left, ref operator, ref right) => {
                self.logical_expression(left, operator, right)
            }
            Expr::Call(ref callee, ref _paren, ref args) => self.call_expression(callee, args),
        }
    }

    pub fn resolve(&mut self, expr: Expr, depth: usize) {
        self.locals.insert(expr, depth);
    }

    fn call_expression(&mut self, callee: &Expr, args: &[Expr]) -> InterpreterResult<Literal> {
        let function = match self.evaluate(callee)? {
            Literal::Callable(c) => c,
            _ => return Err(EarlyReturn::Error(InterpreterError::InvalidAstType)),
        };
        let mut arg_literals = Vec::new();
        for arg in args {
            arg_literals.push(self.evaluate(arg)?);
        }

        if arg_literals.len() != function.arity() {
            return Err(EarlyReturn::Error(InterpreterError::MismatchFunctionArity));
        }
        function
            .call(self, arg_literals)
            .map_err(|_| EarlyReturn::Error(InterpreterError::InvalidAstType))
    }

    fn function(&self, name: Token, params: Vec<Token>, body: Vec<Stmt>) -> InterpreterResult<()> {
        let function = Function::new(params, body, Rc::clone(&self.environment));
        if let Some(name) = name.lexeme {
            self.environment
                .borrow_mut()
                .define(name, Literal::Callable(Box::new(function)));
        }
        Ok(())
    }

    fn return_statement(&mut self, return_value: Option<Expr>) -> InterpreterResult<()> {
        if let Some(return_value) = return_value {
            let value = self.evaluate(&return_value)?;
            return Err(EarlyReturn::Return(value));
        }
        Err(EarlyReturn::Return(Literal::Nil))
    }

    fn while_statement(&mut self, condition: Expr, body: Stmt) -> InterpreterResult<()> {
        while bool::from(self.evaluate(&condition)?) {
            self.interpret(vec![body.clone()])?; // TODO expensive clone, how to use ref for this?
        }
        Ok(())
    }

    fn if_statement(
        &mut self,
        condition: Expr,
        consequent: Stmt,
        alternative: Option<Stmt>,
    ) -> InterpreterResult<()> {
        if bool::from(self.evaluate(&condition)?) {
            self.interpret(vec![consequent])?;
        } else if let Some(alt) = alternative {
            self.interpret(vec![alt])?;
        }
        Ok(())
    }

    fn logical_expression(
        &mut self,
        left: &Expr,
        operator: &Token,
        right: &Expr,
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

    fn assignment_expression(
        &mut self,
        expr: &Expr,
        name: &Token,
        init: &Expr,
    ) -> InterpreterResult<Literal> {
        let value = self.evaluate(init)?;

        let distance = self.locals.get(expr);

        if let Some(name) = &name.lexeme {
            let name = name.to_string();
            let assign_result;
            if let Some(distance) = distance {
                assign_result =
                    self.environment
                        .borrow_mut()
                        .assign_at(*distance, name, value.clone());
            } else {
                assign_result = self.globals.borrow_mut().assign(name, value.clone());
            }

            return assign_result.map(|()| value).map_err(EarlyReturn::Error);
        }
        Ok(value)
    }

    fn print_statement(&mut self, expr: Expr) -> InterpreterResult<()> {
        let value = self.evaluate(&expr)?;
        println!("{}", value);
        Ok(())
    }

    fn var_statement(&mut self, name: Token, init: Option<Expr>) -> InterpreterResult<()> {
        let mut value = None;
        if init.is_some() {
            value = Some(self.evaluate(&init.unwrap())?);
        }

        if let Some(name) = name.lexeme {
            match value {
                Some(v) => self.environment.borrow_mut().define(name, v),
                None => self.environment.borrow_mut().define(name, Literal::Nil),
            }
        }
        Ok(())
    }

    fn var_expression(&self, expr: &Expr, name: &Token) -> InterpreterResult<Literal> {
        let name = name
            .lexeme
            .as_ref()
            .expect("Expected lexeme for variable lookup");
        self.lookup_variable(expr, name)
    }

    fn lookup_variable(&self, expr: &Expr, name: &str) -> InterpreterResult<Literal> {
        let distance = self.locals.get(expr);
        let res;
        if let Some(distance) = distance {
            res = self.environment.borrow().get_at(*distance, name);
        } else {
            res = self.globals.borrow().get(name);
        }
        res.ok_or_else(|| EarlyReturn::Error(InterpreterError::UndefinedVariable(name.to_string())))
    }

    fn unary_expr(&mut self, operator: &Token, right: &Expr) -> InterpreterResult<Literal> {
        let right = self.evaluate(right)?;
        use frontend::token::TokenType::*;
        match operator.token_type {
            Minus => Ok(Literal::Number(-(f64::try_from(right)?))),
            Bang => Ok(Literal::Boolean(!(bool::try_from(TryFromWrapper(right))?))),
            _ => Err(EarlyReturn::Error(InterpreterError::InvalidAstType)),
        }
    }

    fn binary_expr(
        &mut self,
        left: &Expr,
        operator: &Token,
        right: &Expr,
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
                _ => Err(EarlyReturn::Error(InterpreterError::InvalidAstType)),
            },
            Greater => Ok(Literal::Boolean(left > right)),
            GreaterEqual => Ok(Literal::Boolean(left >= right)),
            Less => Ok(Literal::Boolean(left < right)),
            LessEqual => Ok(Literal::Boolean(left <= right)),
            EqualEqual => Ok(Literal::Boolean(left == right)),
            BangEqual => Ok(Literal::Boolean(left != right)),
            _ => Err(EarlyReturn::Error(InterpreterError::InvalidAstType)),
        }
    }
}
