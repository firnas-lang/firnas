pub enum OpCode {
    OpReturn,
}

pub struct Chunk(Vec<OpCode>);

impl Chunk {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn write(&mut self, byte: OpCode) {
        self.0.push(byte);
    }
}

#[cfg(feature = "dbg")]
impl Chunk {
    pub fn disassemble_chunk(&self, name: &str) {
        println!("== {name} ==");
        let mut offset = 0;
        while offset < self.0.len() {
            offset = self.disassemble_instruction(offset);
        }
    }

    fn disassemble_instruction(&self, offset: usize) -> usize {
        print!("{offset:04} ");
        let instruction = self.0.get(offset).unwrap();
        match instruction {
            OpCode::OpReturn => Chunk::simple_instruction("OP_RETURN", offset),
        }
    }

    fn simple_instruction(name: &str, offset: usize) -> usize {
        println!("{name}");
        offset + 1
    }
}
