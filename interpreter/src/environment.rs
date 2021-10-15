use lexer::literal::Literal;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Environment {
    values: HashMap<String, Literal>,
}

impl Environment {
    pub fn new() -> Self {
        Environment {
            values: HashMap::new(),
        }
    }

    pub fn define(&mut self, name: String, value: Literal) {
        self.values.insert(name, value);
    }

    pub fn get(&self, name: String) -> Option<Literal> {
        if let Some(res) = self.values.get(&name) {
            Some(res.clone())
        } else {
            None
        }
    }
}
