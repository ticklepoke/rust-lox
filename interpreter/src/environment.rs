use crate::interpreter::InterpreterResult;
use lexer::literal::Literal;
use std::collections::HashMap;
use utils::errors::InterpreterError;

#[derive(Debug, Clone)]
pub struct Environment {
    values: HashMap<String, Literal>,
    pub enclosing: Option<Box<Environment>>,
}

impl Environment {
    pub fn new(enclosing: Option<Box<Environment>>) -> Self {
        Environment {
            values: HashMap::new(),
            enclosing,
        }
    }

    pub fn define(&mut self, name: String, value: Literal) {
        self.values.insert(name, value);
    }

    pub fn get(&self, name: String) -> Option<Literal> {
        if let Some(res) = self.values.get(&name) {
            return Some(res.clone());
        }

        if let Some(parent) = &self.enclosing {
            return parent.get(name);
        }

        None
    }

    pub fn assign(&mut self, name: String, value: Literal) -> InterpreterResult<()> {
        if self.values.contains_key(&name) {
            self.values.insert(name, value);
            return Ok(());
        }

        if let Some(mut parent) = self.enclosing.clone() {
            println!("outer scope");
            return parent.assign(name, value);
        }

        Err(InterpreterError::UndefinedVariable(name))
    }
}
