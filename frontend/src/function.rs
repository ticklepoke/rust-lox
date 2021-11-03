use crate::scanner::ScannerResult;
use crate::{instance::Instance, runnable::EarlyReturn};
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
    is_init: bool,
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
            Ok(_) => {
                if self.is_init {
                    let this_method = self.closure.borrow_mut().get_at(0, "init");
                    if let Some(this_method) = this_method {
                        return Ok(this_method);
                    }
                }
                Ok(Literal::Nil)
            }
            Err(e) => match e {
                EarlyReturn::Error(_) => Err(ScannerError::UnknownError),
                EarlyReturn::Return(val) => {
                    if self.is_init {
                        let this = self.closure.borrow_mut().get_at(0, "this");
                        if let Some(this) = this {
                            return Ok(this);
                        }
                    }
                    Ok(val)
                }
            },
        }
    }

    fn box_clone(&self) -> Box<dyn Callable> {
        Box::new(self.clone())
    }

    fn bind(&self, instance: Instance) -> Box<dyn Callable> {
        let mut env = Environment::new(Some(Rc::clone(&self.closure)));
        env.define("this".to_string(), Literal::Instance(instance));
        Box::new(Function::new(
            self.params.clone(),
            self.body.clone(),
            Rc::new(RefCell::new(env)),
            self.is_init,
        ))
    }
}

impl Function {
    pub fn new(
        params: Vec<Token>,
        body: Vec<Stmt>,
        closure: Rc<RefCell<Environment>>,
        is_init: bool,
    ) -> Self {
        Function {
            params,
            body,
            closure,
            is_init,
        }
    }
}
