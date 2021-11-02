use crate::class::Class;
use crate::literal::Literal;
use crate::token::Token;
use std::collections::HashMap;
use std::fmt::Display;

#[derive(Debug, Clone)]
pub struct Instance {
    class: Class,
    pub fields: HashMap<String, Literal>,
}

impl Instance {
    pub fn new(class: Class) -> Self {
        let fields = HashMap::new();
        Instance { class, fields }
    }

    pub fn get(&self, name: Token) -> Literal {
        if let Some(name) = name.lexeme {
            return self
                .fields
                .get(name.as_str())
                .unwrap_or(&Literal::Nil)
                .clone();
        };
        Literal::Nil
    }
}

impl Display for Instance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} instance", self.class.to_string())
    }
}
