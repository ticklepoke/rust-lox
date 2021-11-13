#[derive(Debug)]
pub enum Literal {
    String(String),
    Number(f64),
    Nil,
}
