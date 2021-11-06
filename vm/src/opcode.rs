#[derive(Debug)]
pub enum OpCode {
    // Values
    ConstantNumber(f64),

    // Unary Operators
    Negate,

    // Binary Operators
    Add,
    Subtract,
    Multiply,
    Divide,

    Return,
}
