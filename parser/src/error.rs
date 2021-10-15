use crate::ast::Expr;

pub type ParserResult = Result<Expr, ParserError>;

#[derive(Debug)]
pub enum ParserError {
    UnexpectedToken(String, usize),
}
