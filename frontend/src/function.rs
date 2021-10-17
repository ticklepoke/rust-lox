use utils::errors::ScannerError;

use crate::{
    ast::Stmt, callable::Callable, environment::Environment, literal::Literal, token::Token,
};

#[derive(Debug, Clone)]
pub struct Function {
    params: Vec<Token>,
    body: Vec<Stmt>,
}

impl Callable for Function {
    fn arity(&self) -> usize {
        self.params.len()
    }

    fn call(
        &self,
        interpreter: &mut dyn crate::runnable::Runnable,
        args: Vec<crate::literal::Literal>,
    ) -> crate::scanner::ScannerResult<crate::literal::Literal> {
        let mut curr_env = Environment::new(Some(Box::new(interpreter.get_env().clone())));

        for (n, p) in args.into_iter().enumerate() {
            if let Some(name) = &self.params[n].lexeme {
                curr_env.define(name.to_string(), p);
            }
        }

        interpreter
            .block(self.body.clone(), curr_env)
            .map_err(|_| ScannerError::UnknownError)?;

        Ok(Literal::Nil)
    }

    fn box_clone(&self) -> Box<dyn Callable> {
        Box::new(self.clone())
    }
}

impl Function {
    pub fn new(params: Vec<Token>, body: Vec<Stmt>) -> Self {
        Function { params, body }
    }
}
