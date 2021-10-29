extern crate frontend;
extern crate interpreter;

mod lox;

#[cfg(test)]
mod integration_tests;

use frontend::{environment::Environment, literal::Literal};
use interpreter::{clock::Clock, interpreter::Interpreter};
use std::cell::RefCell;
use std::rc::Rc;
use std::{env, path};

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut lox = lox::Lox::new();

    let env = Environment::new(None);
    let interpreter = Rc::new(RefCell::new(Interpreter::new(env)));

    // Add clock function to global env
    interpreter
        .borrow()
        .environment
        .borrow_mut()
        .define("clock".to_string(), Literal::Callable(Box::new(Clock {})));

    if args.len() == 1 {
        lox.run_prompt(interpreter);
    } else if args.len() == 2 {
        lox.run_file(path::PathBuf::from(&args[1]), interpreter);
    } else {
        println!("usage: lox [filename.lox]")
    }
}
