use std::u32;

use crate::value::Value;
use crate::value::ValueVec;

pub enum OpCode {
    Return,
    Constant,
}

impl From<OpCode> for u8 {
    fn from(val: OpCode) -> Self {
        match val {
            OpCode::Return => 0,
            OpCode::Constant => 1,
        }
    }
}

impl From<u8> for OpCode {
    fn from(val: u8) -> Self {
        match val {
            0 => OpCode::Return,
            1 => OpCode::Constant,
            _ => panic!("Undefined state"),
        }
    }
}

pub struct Chunk {
    code: Vec<u8>,
    lines: Vec<u32>,
    constants: ValueVec,
}

impl Chunk {
    pub fn new() -> Self {
        Self {
            code: Vec::new(),
            lines: Vec::new(),
            constants: ValueVec::new(),
        }
    }

    pub fn write(&mut self, byte: u8, line: u32) {
        self.code.push(byte);
        self.lines.push(line);
    }

    pub fn add_constant(&mut self, value: Value) -> usize {
        self.constants.write(value);
        self.constants.len() - 1
    }
}

impl Default for Chunk {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(feature = "dbg")]
impl Chunk {
    pub fn disassemble_chunk(&self, name: &str) {
        println!("== {name} ==");
        let mut offset = 0;
        while offset < self.code.len() {
            offset = self.disassemble_instruction(offset);
        }
    }

    fn disassemble_instruction(&self, offset: usize) -> usize {
        print!("{offset:04} ");
        if offset > 0 && self.lines.get(offset).unwrap() == self.lines.get(offset - 1).unwrap() {
            print!("   | ");
        } else {
            print!("{:04} ", self.lines.get(offset).unwrap());
        }
        let instruction = self.code.get(offset).unwrap();
        match (*instruction).into() {
            OpCode::Return => Chunk::simple_instruction("OP_RETURN", offset),
            OpCode::Constant => Chunk::constant_instruction("OP_CONSTANT", self, offset),
        }
    }

    fn simple_instruction(name: &str, offset: usize) -> usize {
        println!("{name}");
        offset + 1
    }

    fn constant_instruction(name: &str, chunk: &Chunk, offset: usize) -> usize {
        let constant = chunk.code.get(offset + 1).unwrap();
        println!(
            "{name:<16} {constant:04} '{}'",
            chunk.constants.get(*constant as usize).unwrap()
        );
        offset + 2
    }
}
