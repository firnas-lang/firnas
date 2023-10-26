use firnas_core::chunk::Chunk;
use firnas_core::dbg_exec;
use firnas_core::op_code::OpCode;
use firnas_core::value::Value;
use firnas_core::vm::Vm;

fn main() {
    let mut chunk = Chunk::new();

    chunk.write_constant(Value::new(1.2), 123);
    chunk.write_constant(Value::new(3.4), 123);
    chunk.write_opcode(OpCode::Add, 123);
    chunk.write_constant(Value::new(5.6), 123);
    chunk.write_opcode(OpCode::Divide, 123);
    chunk.write_opcode(OpCode::Negate, 123);
    chunk.write_opcode(OpCode::Return, 123);
    dbg_exec! { chunk.disassemble_chunk("test chunk") }

    let mut vm = Vm::new(chunk);
    _ = vm.interpret();
}
