use firnas_core::chunk::Chunk;
use firnas_core::chunk::OpCode;
use firnas_core::dbg_exec;
use firnas_core::value::Value;
use firnas_core::vm::Vm;

fn main() {
    let mut chunk = Chunk::new();

    chunk.write_constant(Value::new(1.2), 123);
    chunk.write(OpCode::Negate.into(), 123);
    chunk.write(OpCode::Return.into(), 123);
    dbg_exec! { chunk.disassemble_chunk("test chunk") }

    let mut vm = Vm::new(chunk);
    _ = vm.interpret();
}
