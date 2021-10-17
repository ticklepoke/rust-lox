use crate::literal::Literal;
use std::{cell::RefCell, collections::HashMap, rc::Rc};
use utils::errors::InterpreterError;

type InterpreterResult<T> = Result<T, InterpreterError>;

#[derive(Debug, Clone)]
pub struct Environment {
    values: HashMap<String, Literal>,
    pub enclosing: Option<Rc<RefCell<Environment>>>,
}

impl Environment {
    pub fn new(enclosing: Option<Rc<RefCell<Environment>>>) -> Self {
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
            return parent.borrow_mut().get(name);
        }

        None
    }

    pub fn assign(&mut self, name: String, value: Literal) -> InterpreterResult<()> {
        if self.values.contains_key(&name) {
            self.values.insert(name, value);
            return Ok(());
        }

        if let Some(parent) = &self.enclosing {
            return parent.borrow_mut().assign(name, value);
        }

        Err(InterpreterError::UndefinedVariable(name))
    }

    pub fn into_cell(self) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(self))
    }
}
