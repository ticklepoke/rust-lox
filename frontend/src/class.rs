use crate::callable::Callable;
use crate::instance::Instance;
use crate::literal::Literal;
use std::collections::HashMap;
use std::fmt::Display;

#[derive(Debug, Clone)]
pub struct Class {
    name: String,
    methods: HashMap<String, Literal>,
    super_class: Option<Box<Class>>,
}

impl Class {
    pub fn new(
        name: String,
        super_class: Option<Box<Class>>,
        methods: HashMap<String, Literal>,
    ) -> Self {
        Class {
            name,
            methods,
            super_class,
        }
    }

    pub fn get_method(&self, name: &str) -> Option<Literal> {
        // TODO HACK to_owned might clone
        let own_method = self.methods.get(name).map(|m| m.to_owned());
        if own_method.is_some() {
            return own_method;
        }
        if let Some(super_class) = self.super_class.as_ref() {
            return super_class.get_method(name);
        }
        None
    }
}

impl Display for Class {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl Callable for Class {
    fn arity(&self) -> usize {
        let init = self.get_method("init");
        if let Some(Literal::Callable(init)) = init {
            return init.arity();
        }
        0
    }

    fn box_clone(&self) -> Box<dyn Callable> {
        Box::new(self.clone())
    }

    fn call(
        &self,
        interpreter: &mut dyn crate::runnable::Runnable,
        args: Vec<crate::literal::Literal>,
    ) -> crate::scanner::ScannerResult<crate::literal::Literal> {
        // HACK: cloning class to create instance for now to avoid messy lifetimes
        let instance = Instance::new(self.clone());
        let init = self.get_method("init");
        if let Some(Literal::Callable(init)) = init {
            init.bind(instance.clone()).call(interpreter, args)?;
        }
        Ok(Literal::Instance(instance))
    }

    fn bind(&self, _instance: Instance) -> Box<dyn Callable> {
        panic!("Unable to bind on class constructor");
    }
}
