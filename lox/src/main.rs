extern crate interpreter;
extern crate lexer;
extern crate parser;

mod lox;

use interpreter::environment::Environment;
use interpreter::interpreter::Interpreter;
use std::{env, path};

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut lox = lox::Lox::new();

    let env = Environment::new();
    let mut interpreter = Interpreter::new(env);

    if args.len() == 1 {
        lox.run_prompt(&mut interpreter);
    } else if args.len() == 2 {
        lox.run_file(path::PathBuf::from(&args[1]), &mut interpreter);
    } else {
        println!("usage: lox [filename.lox]")
    }
}
