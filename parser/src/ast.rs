use lexer::literal::Literal;
use lexer::token::Token;
use std::fmt;

#[derive(Debug)]
pub enum Stmt {
    Expr(Expr),
    Print(Expr),
    Var(Token, Option<Expr>),
}

impl fmt::Display for Stmt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Stmt::Expr(ref e) | Stmt::Print(ref e) => write!(f, "{}", e),
            Stmt::Var(ref name, ref init) => match init {
                Some(ref init) => write!(f, "({} {})", name, init),
                None => write!(f, "({})", name),
            },
        }
    }
}

#[derive(Debug)]
pub enum Expr {
    Binary(Box<Expr>, Token, Box<Expr>),
    Grouping(Box<Expr>),
    Literal(Literal),
    Unary(Token, Box<Expr>),
    Variable(Token),
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Expr::Binary(ref left, ref token, ref right) => {
                write!(f, "({:?} {} {})", token.token_type, left, right)
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
            Expr::Variable(ref token) => {
                write!(f, "{}", token)
            }
        }
    }
}
