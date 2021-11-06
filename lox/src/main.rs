extern crate frontend;
extern crate interpreter;
extern crate vm;

mod lox;

#[cfg(test)]
mod integration_tests;

use frontend::{environment::Environment, literal::Literal};
use interpreter::{clock::Clock, interpreter::Interpreter};
use std::cell::RefCell;
use std::rc::Rc;
use std::{env, path};
use vm::{chunk::Chunk, opcode::OpCode};

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
        let mut chunk = Chunk::new();
        chunk.write_chunk(OpCode::ConstantNumber(123.0), 1);
        chunk.write_chunk(OpCode::ConstantNumber(1.2), 3);
        chunk.write_chunk(OpCode::ConstantNumber(3.4), 4);
        chunk.write_chunk(OpCode::Add, 5);
        chunk.write_chunk(OpCode::ConstantNumber(5.6), 6);
        chunk.write_chunk(OpCode::Divide, 7);
        chunk.write_chunk(OpCode::Negate, 2);
        chunk.write_chunk(OpCode::Return, 8);
        let mut virtual_machine = vm::vm::Vm::new(chunk);
        match virtual_machine.interpret() {
            Ok(()) => {}
            Err(e) => panic!("Error: {:?}", e),
        }
    }
}
