use firnas_core::chunk::Chunk;
use firnas_core::chunk::OpCode;
use firnas_core::value::Value;

fn main() {
    let mut chunk = Chunk::new();
    chunk.write(OpCode::Return.into());

    let constant = chunk.add_constant(Value::new(1.2));
    chunk.write(OpCode::Constant.into());
    chunk.write(constant as u8);

    chunk.disassemble_chunk("test chunk");
}
