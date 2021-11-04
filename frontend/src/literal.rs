use crate::callable::Callable;
use crate::class::Class;
use crate::instance::Instance;
use crate::runnable::EarlyReturn;
use std::cmp::{Ordering, PartialEq, PartialOrd};
use std::convert::TryFrom;
use std::fmt::Display;
use std::hash::Hash;
use utils::errors::InterpreterError;

#[derive(Debug, Clone)]
pub enum Literal {
    String(String),
    Number(f64),
    Boolean(bool),
    Nil,
    Callable(Box<dyn Callable>),
    Class(Class),
    Instance(Instance),
}

impl Hash for Literal {
    fn hash<H: std::hash::Hasher>(&self, _state: &mut H) {
        // Noop, since literals do not need to get resolved
    }
}

impl Eq for Literal {
    fn assert_receiver_is_total_eq(&self) {
        // Noop
    }
}

impl Display for Literal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::String(str) => write!(f, "{}", str),
            Self::Number(fl) => write!(f, "{}", fl),
            Self::Boolean(b) => write!(f, "{}", b),
            Self::Nil => write!(f, "Nil"),
            Self::Callable(_c) => write!(f, "Callable"),
            Self::Class(c) => write!(f, "class {}", c),
            Self::Instance(i) => write!(f, "{}", i),
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
            (&Literal::Instance(ref i), &Literal::Instance(ref j)) => {
                // check for referential equality
                std::ptr::eq(i, j)
            }
            (&Literal::Class(ref i), &Literal::Class(ref j)) => {
                // check for referential equality
                std::ptr::eq(i, j)
            }
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
            (&Literal::Instance(ref _i), &Literal::Instance(ref _j)) => None,
            (&Literal::Class(ref _i), &Literal::Class(ref _j)) => None,
            _ => None,
        }
    }
}

impl TryFrom<Literal> for f64 {
    type Error = EarlyReturn;

    fn try_from(value: Literal) -> Result<Self, Self::Error> {
        if let Literal::Number(n) = value {
            Ok(n)
        } else {
            Err(EarlyReturn::Error(InterpreterError::InvalidCoercion(
                "Unable to coerce into number".to_string(),
            )))
        }
    }
}

impl TryFrom<Literal> for String {
    type Error = EarlyReturn;

    fn try_from(value: Literal) -> Result<Self, Self::Error> {
        if let Literal::String(s) = value {
            Ok(s)
        } else {
            Err(EarlyReturn::Error(InterpreterError::InvalidCoercion(
                "Unable to coerce into string".to_string(),
            )))
        }
    }
}

// Hack: Conflicting implementation for trait, circumvents E0119
pub struct TryFromWrapper<T>(pub T);
impl TryFrom<TryFromWrapper<Literal>> for bool {
    type Error = EarlyReturn;

    fn try_from(value: TryFromWrapper<Literal>) -> Result<Self, Self::Error> {
        if let Literal::Boolean(b) = value.0 {
            Ok(b)
        } else {
            Err(EarlyReturn::Error(InterpreterError::InvalidCoercion(
                "Unable to coerce into boolean".to_string(),
            )))
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
    type Error = EarlyReturn;

    fn try_from(value: Literal) -> Result<Self, Self::Error> {
        if let Literal::Nil = value {
            Ok(())
        } else {
            Err(EarlyReturn::Error(InterpreterError::InvalidCoercion(
                "Unable to coerce into Nil".to_string(),
            )))
        }
    }
}
