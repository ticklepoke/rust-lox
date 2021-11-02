use crate::callable::Callable;
use crate::instance::Instance;
use crate::literal::Literal;
use std::collections::HashMap;
use std::fmt::Display;

#[derive(Debug, Clone)]
pub struct Class {
    name: String,
    methods: HashMap<String, Literal>,
}

impl Class {
    pub fn new(name: String, methods: HashMap<String, Literal>) -> Self {
        Class { name, methods }
    }

    pub fn get_method(&self, name: &str) -> Option<Literal> {
        // TODO HACK to_owned might clone
        self.methods.get(name).map(|m| m.to_owned())
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

    fn bind(&self, _instance: Instance) -> Box<dyn Callable> {
        // HACK noop
        self.box_clone()
    }
}
