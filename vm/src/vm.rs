use crate::scanner::Scanner;
use crate::token::TokenType;
use crate::{chunk::Chunk, disassembler, opcode::OpCode, value::Value};

#[derive(Debug)]
pub enum CompileError {
    InvalidOperand(String),
}

#[derive(Debug)]
pub enum RuntimeError {}

#[derive(Debug)]
pub enum InterpreterError {
    Compile(CompileError),
    Runtime(RuntimeError),
}

type InterpreterResult<T> = Result<T, InterpreterError>;

pub struct Vm {
    chunk: Chunk,
    stack: Vec<Value>,
}

impl Vm {
    pub fn new(chunk: Chunk) -> Self {
        Vm {
            chunk,
            stack: Vec::new(),
        }
    }

    pub fn interpret(&mut self, source: &str) -> InterpreterResult<()> {
        let mut scanner = Scanner::new(source);
        loop {
            let tok = scanner.scan_token();
            println!("{:?}", tok);
            if let TokenType::Eof = tok.token_type {
                break;
            }
        }
        Ok(())
    }

    fn run(&mut self) -> InterpreterResult<()> {
        for (idx, op_code_line) in self.chunk.clone().code.borrow_mut().iter().enumerate() {
            // Debug utils
            disassembler::disassemble_instruction(op_code_line, idx);
            self.print_stack();

            match op_code_line.code {
                OpCode::ConstantNumber(val) => self.stack.push(Value::Number(val)),
                OpCode::Negate => {
                    let val = self.pop();
                    if let Value::Number(n) = val {
                        self.push(Value::Number(-n));
                    } else {
                        return Err(InterpreterError::Compile(CompileError::InvalidOperand(
                            format!("Invalid right hand side of '-': {:?}", val),
                        )));
                    }
                }
                OpCode::Add | OpCode::Subtract | OpCode::Multiply | OpCode::Divide => {
                    let res = self.binary_op(&op_code_line.code);
                    self.push(res);
                }
                OpCode::Return => {
                    let val = self.stack.pop().unwrap();
                    println!("{:?}", val);
                    return Ok(());
                }
            }
        }
        self.print_stack();
        Ok(())
    }

    fn binary_op(&mut self, op: &OpCode) -> Value {
        let b = self.pop();
        let a = self.pop();

        match op {
            OpCode::Add => a.add(b),
            OpCode::Subtract => a.subtract(b),
            OpCode::Multiply => a.multiply(b),
            OpCode::Divide => a.divide(b),
            _ => panic!("Opcode cannot be used for binary operations"),
        }
    }

    fn print_stack(&self) {
        if cfg!(debug_assertions) {
            println!("{:?}", self.stack);
        }
    }

    fn push(&mut self, val: Value) {
        self.stack.push(val);
    }

    fn pop(&mut self) -> Value {
        if let Some(val) = self.stack.pop() {
            val
        } else {
            panic!("Cannot call pop() on empty stack");
        }
    }
}
