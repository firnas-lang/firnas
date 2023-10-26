use firnas_core::chunk::Chunk;
use firnas_core::chunk::OpCode;
use firnas_core::value::Value;

fn main() {
    let mut chunk = Chunk::new();
    chunk.write_constant(Value::new(1.2), 123);
    chunk.write(OpCode::Return.into(), 123);
    chunk.disassemble_chunk("test chunk");
}
