use lexer::token::{Literal, Token};
use std::fmt;

#[allow(dead_code)] // TODO remove
#[derive(Debug)]
pub enum Expr {
    Binary(Box<Expr>, Token, Box<Expr>),
    Grouping(Box<Expr>),
    Literal(Literal),
    Unary(Token, Box<Expr>),
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Expr::Binary(ref left, ref token, ref right) => {
                write!(
                    f,
                    "({} {} {})",
                    token.lexeme.as_ref().unwrap_or(&String::new()),
                    left,
                    right
                )
            }
            Expr::Grouping(ref expr) => {
                write!(f, "(group {})", expr)
            }
            Expr::Literal(ref literal) => {
                write!(f, "{}", literal)
            }
            Expr::Unary(ref token, ref expr) => {
                write!(
                    f,
                    "({} {})",
                    token.lexeme.as_ref().unwrap_or(&String::new()),
                    expr
                )
            }
        }
    }
}
