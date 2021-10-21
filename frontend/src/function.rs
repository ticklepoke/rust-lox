use crate::runnable::EarlyReturn;
use crate::scanner::ScannerResult;
use std::cell::RefCell;
use std::rc::Rc;
use utils::errors::ScannerError;

use crate::{
    ast::Stmt, callable::Callable, environment::Environment, literal::Literal, token::Token,
};

#[derive(Debug, Clone)]
pub struct Function {
    params: Vec<Token>,
    body: Vec<Stmt>,
    closure: Rc<RefCell<Environment>>,
}

impl Callable for Function {
    fn arity(&self) -> usize {
        self.params.len()
    }

    fn call(
        &self,
        interpreter: &mut dyn crate::runnable::Runnable,
        args: Vec<crate::literal::Literal>,
    ) -> ScannerResult<crate::literal::Literal> {
        let mut curr_env = Environment::new(Some(Rc::clone(&self.closure)));

        for (n, p) in args.into_iter().enumerate() {
            if let Some(name) = &self.params[n].lexeme {
                curr_env.define(name.to_string(), p);
            }
        }

        let res = interpreter.block(self.body.clone(), curr_env.into_cell());

        match res {
            Ok(_) => Ok(Literal::Nil),
            Err(e) => match e {
                EarlyReturn::Error(_) => Err(ScannerError::UnknownError),
                EarlyReturn::Return(val) => Ok(val),
            },
        }
    }

    fn box_clone(&self) -> Box<dyn Callable> {
        Box::new(self.clone())
    }
}

impl Function {
    pub fn new(params: Vec<Token>, body: Vec<Stmt>, closure: Rc<RefCell<Environment>>) -> Self {
        Function {
            params,
            body,
            closure,
        }
    }
}
