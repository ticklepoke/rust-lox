extern crate frontend;
extern crate interpreter;
extern crate vm;

mod clox;
mod lox;

#[cfg(test)]
mod integration_tests;

use frontend::{environment::Environment, literal::Literal};
use interpreter::{clock::Clock, interpreter::Interpreter};
use std::cell::RefCell;
use std::rc::Rc;
use std::{env, path};
use vm::chunk::Chunk;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut lox = lox::Lox::new();

    if args[1] == "jlox" {
        let env = Environment::new(None);
        let interpreter = Rc::new(RefCell::new(Interpreter::new(env)));

        // Add clock function to global env
        interpreter
            .borrow()
            .environment
            .borrow_mut()
            .define("clock".to_string(), Literal::Callable(Box::new(Clock {})));

        if args.len() == 2 {
            lox.run_prompt(interpreter);
        } else if args.len() == 3 {
            lox.run_file(path::PathBuf::from(&args[2]), interpreter);
        } else {
            println!("usage: jlox [filename.lox]")
        }
    } else if args[1] == "clox" {
        let chunk = Chunk::new();
        let mut virtual_machine = vm::vm::Vm::new(chunk);
        if args.len() == 2 {
            clox::repl(&mut virtual_machine);
        } else if args.len() == 3 {
            clox::run_file(args[2].as_str(), &mut virtual_machine);
        } else {
            println!("Usage: clox [path]");
        }
    }
}
