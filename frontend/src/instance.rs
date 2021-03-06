use crate::class::Class;
use crate::literal::Literal;
use crate::token::Token;
use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt::Display;
use std::rc::Rc;

#[derive(Debug)]
pub struct Instance {
    class: Class,
    pub fields: Rc<RefCell<HashMap<String, Literal>>>,
}

impl Instance {
    pub fn new(class: Class) -> Self {
        let fields = Rc::new(RefCell::new(HashMap::new()));
        Instance { class, fields }
    }

    pub fn get(&self, name: Token) -> Literal {
        if let Some(name) = name.lexeme {
            if let Some(field) = self.fields.borrow().get(name.as_str()) {
                // TODO HACK check if cloning leads to issues
                return field.clone();
            }
            if let Some(Literal::Callable(method)) = self.class.get_method(name.as_str()) {
                return Literal::Callable(method.bind(self.clone()));
            }
        };
        Literal::Nil
    }

    pub fn set(&mut self, name: Token, value: Literal) {
        if let Some(name) = name.lexeme {
            self.fields.borrow_mut().insert(name, value);
        }
    }
}

impl Display for Instance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} instance", self.class.to_string())
    }
}

impl Clone for Instance {
    fn clone(&self) -> Self {
        Instance {
            class: self.class.clone(),
            fields: Rc::clone(&self.fields),
        }
    }
}
