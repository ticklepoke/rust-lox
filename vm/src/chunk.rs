use std::{cell::RefCell, rc::Rc};

use crate::opcode::OpCode;

#[derive(Debug)]
pub struct OpCodeLine {
    pub code: OpCode,
    pub line: usize,
}

pub struct Chunk {
    pub code: Rc<RefCell<Vec<OpCodeLine>>>,
}

impl Default for Chunk {
    fn default() -> Self {
        Chunk::new()
    }
}

impl Clone for Chunk {
    fn clone(&self) -> Self {
        Chunk {
            code: Rc::clone(&self.code),
        }
    }
}

impl Chunk {
    pub fn new() -> Self {
        Chunk {
            code: Rc::new(RefCell::new(Vec::new())),
        }
    }

    pub fn write_chunk(&mut self, op_code: OpCode, line: usize) {
        self.code.borrow_mut().push(OpCodeLine {
            code: op_code,
            line,
        });
    }
}
