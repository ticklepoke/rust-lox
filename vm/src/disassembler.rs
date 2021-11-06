use crate::chunk::{Chunk, OpCodeLine};

pub fn disassemble_chunk(chunk: &Chunk, name: &str) {
    if cfg!(debug_assertions) {
        println!("== {} ==", name);
        for (idx, op) in chunk.code.borrow().iter().enumerate() {
            disassemble_instruction(op, idx);
        }
    }
}

pub fn disassemble_instruction(op: &OpCodeLine, idx: usize) {
    println!("{} {:?}", idx, op);
}
