use std::cell::RefCell;
use std::rc::Rc;

use crate::ast::Stmt;
use crate::environment::Environment;
use crate::literal::Literal;
use utils::errors::InterpreterError;

type InterpreterResult<T> = Result<T, EarlyReturn>;

#[derive(Debug)]
pub enum EarlyReturn {
    Error(InterpreterError),
    Return(Literal),
}

// Abstract behaviour that interpreters and compilers should implement
pub trait Runnable {
    fn block(
        &mut self,
        body: Vec<Stmt>,
        environment: Rc<RefCell<Environment>>,
    ) -> InterpreterResult<()>;

    fn get_global(&self) -> Rc<RefCell<Environment>>;
}
