use crate::op_code::OpCode;
use crate::value::Value;
use crate::value::ValueVec;

pub struct Chunk {
    pub code: Vec<u8>,
    pub lines: Vec<u32>,
    pub constants: ValueVec,
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

    pub fn write_opcode(&mut self, opcode: OpCode, line: u32) {
        self.write(opcode.into(), line);
    }

    pub fn write_constant(&mut self, value: Value, line: u32) {
        self.write(OpCode::Constant.into(), line);
        let constant = self.add_constant(value);
        self.write(constant as u8, line);
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
        println!("==== {name} ====");
        let mut offset = 0;
        while offset < self.code.len() {
            offset = self.disassemble_instruction(offset);
        }
    }

    pub fn disassemble_instruction(&self, offset: usize) -> usize {
        print!("{offset:04} ");
        if offset > 0 && &self.lines[offset] == &self.lines[offset - 1] {
            print!("   | ");
        } else {
            print!("{:04} ", &self.lines[offset]);
        }
        let instruction = &self.code[offset];
        match (*instruction).into() {
            code @ OpCode::Return => Chunk::simple_instruction(&code.dbg_str(), offset),
            code @ OpCode::Constant => Chunk::constant_instruction(&code.dbg_str(), self, offset),
            code @ OpCode::ConstantLong => {
                Chunk::constant_long_instruction(&code.dbg_str(), self, offset)
            }
            code @ OpCode::Negate => Chunk::simple_instruction(&code.dbg_str(), offset),
            code @ OpCode::Add => Chunk::simple_instruction(&code.dbg_str(), offset),
            code @ OpCode::Subtract => Chunk::simple_instruction(&code.dbg_str(), offset),
            code @ OpCode::Multiply => Chunk::simple_instruction(&code.dbg_str(), offset),
            code @ OpCode::Divide => Chunk::simple_instruction(&code.dbg_str(), offset),
        }
    }

    fn simple_instruction(name: &str, offset: usize) -> usize {
        println!("{name}");
        offset + 1
    }

    fn constant_instruction(name: &str, chunk: &Chunk, offset: usize) -> usize {
        let constant = &chunk.code[offset + 1];
        println!(
            "{name:<16} {constant:04} '{}'",
            &chunk.constants[*constant as usize]
        );
        offset + 2
    }

    fn constant_long_instruction(name: &str, chunk: &Chunk, offset: usize) -> usize {
        let constant_long = &chunk.code[offset + 1..offset + 4];
        print!("{name:<16} ");
        print!("[");
        for constant in constant_long {
            print!(" {constant:04} ");
        }
        print!("]");
        print!(" [");
        for constant in constant_long {
            print!(" '{}' ", &chunk.constants[*constant as usize]);
        }
        println!("]");
        offset + 4
    }
}
