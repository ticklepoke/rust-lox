use crate::callable::Callable;
use std::cmp::{Ordering, PartialEq, PartialOrd};
use std::convert::TryFrom;
use std::fmt::Display;
use utils::errors::InterpreterError;

#[derive(Debug, Clone)]
pub enum Literal {
    String(String),
    Number(f64),
    Boolean(bool),
    Nil,
    Callable(Box<dyn Callable>),
}

impl Display for Literal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::String(str) => write!(f, "{}", str),
            Self::Number(fl) => write!(f, "{}", fl),
            Self::Boolean(b) => write!(f, "{}", b),
            Self::Nil => write!(f, "Nil"),
            Self::Callable(_c) => write!(f, "Callable"),
        }
    }
}

// Allows equality checks on Expr
impl PartialEq for Literal {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (&Literal::String(ref s), &Literal::String(ref o)) => (s == o),
            (&Literal::Number(ref s), &Literal::Number(ref o)) => (s == o),
            (&Literal::Boolean(ref s), &Literal::Boolean(ref o)) => (s == o),
            (&Literal::Nil, &Literal::Nil) => true,
            _ => false,
        }
    }
}

// Allows gt lt eq checks on Expr
impl PartialOrd for Literal {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (&Literal::String(ref s), &Literal::String(ref o)) => s.partial_cmp(o),
            (&Literal::Number(ref s), &Literal::Number(ref o)) => s.partial_cmp(o),
            (&Literal::Boolean(ref s), &Literal::Boolean(ref o)) => s.partial_cmp(o),
            (&Literal::Nil, &Literal::Nil) => Some(Ordering::Equal),
            _ => None,
        }
    }
}

impl TryFrom<Literal> for f64 {
    type Error = InterpreterError;

    fn try_from(value: Literal) -> Result<Self, Self::Error> {
        if let Literal::Number(n) = value {
            Ok(n)
        } else {
            Err(InterpreterError::InvalidCoercion(
                "Unable to coerce into number".to_string(),
            ))
        }
    }
}

impl TryFrom<Literal> for String {
    type Error = InterpreterError;

    fn try_from(value: Literal) -> Result<Self, Self::Error> {
        if let Literal::String(s) = value {
            Ok(s)
        } else {
            Err(InterpreterError::InvalidCoercion(
                "Unable to coerce into string".to_string(),
            ))
        }
    }
}

// Hack: Conflicting implementation for trait, circumvents E0119
pub struct TryFromWrapper<T>(pub T);
impl TryFrom<TryFromWrapper<Literal>> for bool {
    type Error = InterpreterError;

    fn try_from(value: TryFromWrapper<Literal>) -> Result<Self, Self::Error> {
        if let Literal::Boolean(b) = value.0 {
            Ok(b)
        } else {
            Err(InterpreterError::InvalidCoercion(
                "Unable to coerce into boolean".to_string(),
            ))
        }
    }
}

impl From<Literal> for bool {
    fn from(value: Literal) -> Self {
        match value {
            Literal::Nil => false,
            Literal::Boolean(b) => b,
            _ => true,
        }
    }
}

impl TryFrom<Literal> for () {
    type Error = InterpreterError;

    fn try_from(value: Literal) -> Result<Self, Self::Error> {
        if let Literal::Nil = value {
            Ok(())
        } else {
            Err(InterpreterError::InvalidCoercion(
                "Unable to coerce into Nil".to_string(),
            ))
        }
    }
}
