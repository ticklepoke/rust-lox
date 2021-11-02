use crate::callable::Callable;
use crate::instance::Instance;
use crate::literal::Literal;

#[derive(Debug, Clone)]
pub struct Class {
    name: String,
}

impl Class {
    pub fn new(name: String) -> Self {
        Class { name }
    }

    pub fn to_string(&self) -> String {
        self.name.clone()
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
