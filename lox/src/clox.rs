use std::fs;
use std::io;
use std::io::prelude::*;

use vm::vm::Vm;

pub fn repl(virtual_machine: &mut Vm) {
    let mut input = String::new();
    let stdin = io::stdin();

    loop {
        print!("clox>");
        io::stdout().flush().expect("[ICE] Unable to flush stdout");
        stdin.lock().read_line(&mut input).unwrap();
        match virtual_machine.interpret(input.as_str()) {
            Ok(()) => {}
            Err(e) => panic!("Error: {:?}", e),
        }
        input.clear();
    }
}

pub fn run_file(source_file: &str, virtual_machine: &mut Vm) {
    let source = fs::read_to_string(source_file).expect("[ICE] Unable to read file");
    // run
    match virtual_machine.interpret(source.as_str()) {
        Ok(()) => {}
        Err(e) => panic!("Error: {:?}", e),
    }
}
