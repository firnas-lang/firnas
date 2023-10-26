use crate::{
    chunk::{Chunk, OpCode},
    dbg_exec,
    value::Value,
};

pub struct Vm {
    chunk: Chunk,
    ip: usize,
}

pub enum InterpretError {
    CompileTime,
    RunTime,
}

impl Vm {
    pub fn new(chunk: Chunk) -> Self {
        Self { chunk, ip: 0 }
    }

    pub fn interpret(&mut self) -> Result<(), InterpretError> {
        Self::run(self)
    }

    pub fn run(&mut self) -> Result<(), InterpretError> {
        loop {
            dbg_exec! { self.chunk.disassemble_instruction(self.ip) }
            if cfg!(feature = "dbg") {}
            let byte = self.read_bytecode();
            match byte.into() {
                OpCode::Return => return Ok(()),
                OpCode::Constant => {
                    let constant = self.read_constant();
                    println!("{constant}");
                }
                _ => todo!(),
            }
        }
    }

    fn read_bytecode(&mut self) -> u8 {
        let chunk = self.chunk.code[self.ip];
        self.ip += 1;
        chunk
    }

    fn read_constant(&mut self) -> &Value {
        let bytecode = self.read_bytecode();
        let constant = &self.chunk.constants[bytecode as usize];
        constant
    }
}
