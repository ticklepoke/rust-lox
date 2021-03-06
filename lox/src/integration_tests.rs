use crate::lox::Lox;
use frontend::environment::Environment;
use interpreter::interpreter::Interpreter;
use std::cell::RefCell;
use std::env;
use std::path;
use std::rc::Rc;

#[test]
fn run_file() {
    let mut lox = Lox::new();
    let env = Environment::new(None);
    let interpreter = Rc::new(RefCell::new(Interpreter::new(env)));

    let curr_dir = env::current_dir().expect("path");
    let mut file_path = path::PathBuf::new();
    file_path.push(curr_dir);
    file_path.push("../__fixtures__/scope.lox");
    lox.run_file(file_path, interpreter);
}
