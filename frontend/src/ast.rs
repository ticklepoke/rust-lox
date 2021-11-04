use crate::literal::Literal;
use crate::token::Token;
use std::fmt;

#[derive(Debug, Clone)]
pub enum Stmt {
    Block(Vec<Stmt>),
    Expr(Expr),
    Function(Token, Vec<Token>, Vec<Stmt>),
    If(Expr, Box<Stmt>, Option<Box<Stmt>>),
    Print(Expr),
    Return(Token, Option<Expr>),
    Var(Token, Option<Expr>),
    While(Expr, Box<Stmt>),
    Class(Token, Option<Expr>, Vec<Stmt>),
}

impl fmt::Display for Stmt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Stmt::Expr(ref e) | Stmt::Print(ref e) => write!(f, "{}", e),
            Stmt::Var(ref name, ref init) => match init {
                Some(ref init) => write!(f, "({} {})", name, init),
                None => write!(f, "({})", name),
            },
            Stmt::Block(ref stmts) => {
                let mut output = String::new();
                for s in stmts {
                    output.push_str(format!("({})", s).as_str());
                }
                return write!(f, "({})", output);
            }
            Stmt::If(ref condition, ref consequent, ref alternative) => {
                if let Some(alt) = alternative {
                    write!(f, "({} {} {})", condition, consequent, alt)
                } else {
                    write!(f, "({} {})", condition, consequent)
                }
            }
            Stmt::While(ref condition, ref body) => {
                write!(f, "({} {})", condition, body)
            }
            Stmt::Function(ref name, ..) => write!(f, "function {}", name),
            Stmt::Return(ref _return, ref return_value) => match return_value {
                Some(return_value) => write!(f, "return {}", return_value),
                None => write!(f, "return Nil"),
            },
            Stmt::Class(ref name, ..) => match &name.lexeme {
                Some(n) => write!(f, "class {}", n),
                None => write!(f, "class"),
            },
        }
    }
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub enum Expr {
    Assign(Token, Box<Expr>),
    Binary(Box<Expr>, Token, Box<Expr>),
    Call(Box<Expr>, Token, Vec<Expr>),
    Get(Box<Expr>, Token),
    Grouping(Box<Expr>),
    Literal(Literal),
    Logical(Box<Expr>, Token, Box<Expr>),
    Set(Box<Expr>, Token, Box<Expr>),
    This(Token),
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
            Expr::Logical(ref left, ref operator, ref right) => {
                write!(f, "({} {} {})", left, operator, right)
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
            Expr::Assign(ref name, ref init) => {
                write!(f, "({} = {})", name, init)
            }
            Expr::Call(ref callee, ref _paren, ref args) => {
                write!(f, "(functionCall {} ({:?}))", callee, args)
            }
            Expr::Get(ref object, ref name) => {
                write!(f, "(Instance access {} ({:?}))", object, name)
            }
            Expr::Set(ref object, ref name, ref new_value) => {
                write!(f, "({}.{} = {}", object, name, new_value)
            }
            Expr::This(ref name) => {
                write!(f, "{}", name)
            }
        }
    }
}
