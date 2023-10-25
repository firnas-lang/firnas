use firnas_core::chunk::{Chunk, OpCode};

fn main() {
    let mut chunk = Chunk::new();
    chunk.write(OpCode::OpReturn);
    chunk.disassemble_chunk("test chunk");
}
