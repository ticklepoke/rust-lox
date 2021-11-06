#[derive(Debug)]
pub enum Value {
    Number(f64),
    Nil,
}

impl Value {
    pub fn add(&self, other: Value) -> Value {
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => Value::Number(a + b),
            _ => panic!("Invalid use of '+'"),
        }
    }

    pub fn subtract(&self, other: Value) -> Value {
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => Value::Number(a - b),
            _ => panic!("Invalid use of '+'"),
        }
    }

    pub fn multiply(&self, other: Value) -> Value {
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => Value::Number(a * b),
            _ => panic!("Invalid use of '+'"),
        }
    }

    pub fn divide(&self, other: Value) -> Value {
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => Value::Number(a / b),
            _ => panic!("Invalid use of '+'"),
        }
    }
}
