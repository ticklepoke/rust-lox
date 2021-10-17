use crate::ast::Stmt;
use crate::environment::Environment;
use utils::errors::InterpreterError;

type InterpreterResult<T> = Result<T, InterpreterError>;

// Abstract behaviour that interpreters and compilers should implement
pub trait Runnable {
    fn block(&mut self, body: Vec<Stmt>, environment: Environment) -> InterpreterResult<()>;
} // TODO: shift to utils
