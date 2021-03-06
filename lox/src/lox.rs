use std::cell::RefCell;
use std::io::prelude::*;
use std::rc::Rc;
use std::{fs, io, path, process};

use frontend::parser::Parser;
use frontend::scanner::Scanner;
use frontend::token::Token;
use interpreter::interpreter::Interpreter;
use interpreter::resolver::Resolver;

pub struct Lox {
    error: Option<String>,
}

impl Lox {
    pub fn new() -> Self {
        Lox { error: None }
    }

    pub fn run_file(&mut self, path: path::PathBuf, i: Rc<RefCell<Interpreter>>) {
        let source = fs::read_to_string(path).expect("Unable to read file");
        self.run(source.as_str(), i);

        if self.error.is_some() {
            process::exit(1)
        }
    }

    pub fn run_prompt(&mut self, i: Rc<RefCell<Interpreter>>) {
        let mut input = String::new();
        let stdin = io::stdin();
        loop {
            print!("lox> ");
            io::stdout().flush().expect("[ICE] Unable to flush stdout");
            stdin.lock().read_line(&mut input).unwrap();
            self.run(input.as_str(), Rc::clone(&i));
            input.clear();
            self.error = None;
        }
    }

    pub fn report(&mut self, line: usize, message: String) {
        println!("[line {}] Error: {}", line, message);
        self.error = Some(message);
    }

    fn run(&mut self, source: &str, interpreter: Rc<RefCell<Interpreter>>) {
        // Lexer
        let mut scanner = Scanner::new(source);
        let mut tokens: Vec<Token> = Vec::new();
        match scanner.scan_tokens() {
            Ok(ts) => tokens = ts,
            Err(err) => self.report(err.line(), format!("{}", err)),
        }

        // Parser
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().expect("Parser Error");

        let mut resolver = Resolver::new(Rc::clone(&interpreter));
        let resolver_res = resolver.resolve_stmts(&ast);

        if resolver_res.is_err() {
            panic!("Error from resolver");
        }

        // Interpreter
        interpreter
            .borrow_mut()
            .interpret(ast)
            .expect("Interpreter Error");
    }
}
