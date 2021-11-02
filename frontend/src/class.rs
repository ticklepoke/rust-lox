use crate::callable::Callable;
use crate::instance::Instance;
use crate::literal::Literal;
use std::fmt::Display;

#[derive(Debug, Clone)]
pub struct Class {
    name: String,
}

impl Class {
    pub fn new(name: String) -> Self {
        Class { name }
    }
}

impl Display for Class {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl Callable for Class {
    fn arity(&self) -> usize {
        0
    }

    fn box_clone(&self) -> Box<dyn Callable> {
        Box::new(self.clone())
    }

    fn call(
        &self,
        _interpreter: &mut dyn crate::runnable::Runnable,
        _args: Vec<crate::literal::Literal>,
    ) -> crate::scanner::ScannerResult<crate::literal::Literal> {
        // HACK: cloning class to create instance for now to avoid messy lifetimes
        let instance = Instance::new(self.clone());
        Ok(Literal::Instance(instance))
    }
}
