extern crate frontend;
extern crate interpreter;

mod lox;

#[cfg(test)]
mod integration_tests;

use frontend::{environment::Environment, literal::Literal};
use interpreter::{clock::Clock, interpreter::Interpreter};
use std::{env, path};
fn main() {
    let args: Vec<String> = env::args().collect();
    let mut lox = lox::Lox::new();

    let env = Environment::new(None);
    let mut interpreter = Interpreter::new(env);

    // Add clock function to global env
    interpreter
        .environment
        .define("clock".to_string(), Literal::Callable(Box::new(Clock {})));

    if args.len() == 1 {
        lox.run_prompt(&mut interpreter);
    } else if args.len() == 2 {
        lox.run_file(path::PathBuf::from(&args[1]), &mut interpreter);
    } else {
        println!("usage: lox [filename.lox]")
    }
}
