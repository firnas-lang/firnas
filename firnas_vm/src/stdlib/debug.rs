use crate::{value, virtual_machine::VirtualMachine};
use firnas_bytecode::disassemble_chunk;

pub(crate) fn dis_builtin(
    interp: &mut VirtualMachine,
    args: &[value::Value],
) -> Result<value::Value, String> {
    // arity checking is done in the interpreter
    match &args[0] {
        value::Value::Function(closure_handle) => {
            let closure = interp.heap.get_closure(*closure_handle);
            disassemble_chunk(&closure.function.chunk, "");
            Ok(value::Value::Nil)
        }
        _ => Err(format!(
            "Invalid call: expected lox function, got {:?}.",
            value::type_of(&args[0])
        )),
    }
}
