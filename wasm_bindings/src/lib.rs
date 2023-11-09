use firnas_compiler::compiler;
use firnas_vm::virtual_machine;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub fn compile(content: &str) -> String {
    let func_or_err = compiler::Compiler::compile(
        content.to_string(),
        firnas_ext::Extensions {
            lists: false,
            lambdas: false,
        },
    );

    let _ = func_or_err.map(|f| {
        let mut interpreter = virtual_machine::VirtualMachine::default();
        interpreter.interpret(f).unwrap();
    });

    String::from("Done")
}
