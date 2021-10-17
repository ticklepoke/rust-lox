use crate::literal::Literal;
use crate::runnable::Runnable;
use crate::scanner::ScannerResult;
use std::cmp::{Ordering, PartialEq, PartialOrd};
use std::fmt::Debug;

pub trait Callable: Debug {
    fn arity(&self) -> usize;

    fn call(&self, interpreter: &mut dyn Runnable, args: Vec<Literal>) -> ScannerResult<Literal>;

    fn box_clone(&self) -> Box<dyn Callable>;
}

impl Clone for Box<dyn Callable> {
    fn clone(&self) -> Self {
        self.box_clone()
    }
}

impl PartialEq for Box<dyn Callable> {
    fn eq(&self, _other: &Self) -> bool {
        false // Functions are never equal
    }
}

impl PartialOrd for Box<dyn Callable> {
    fn partial_cmp(&self, _other: &Self) -> Option<Ordering> {
        None // Functions cant be ordered?
    }
}
