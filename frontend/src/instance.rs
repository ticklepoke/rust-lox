use crate::class::Class;
use std::fmt::Display;

#[derive(Debug, Clone)]
pub struct Instance {
    class: Class,
}

impl Instance {
    pub fn new(class: Class) -> Self {
        Instance { class }
    }

    pub fn to_string(&self) -> String {
        self.class.to_string().to_owned() + " instance"
    }
}

impl Display for Instance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}
