use firnas_compiler::compiler;
use firnas_vm::stdlib::io::StdIO;
use firnas_vm::virtual_machine;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

struct WasmStdIO;

impl StdIO for WasmStdIO {
    fn print(&self, content: &str) {
        log(&format!("{content}"));
    }
}

#[wasm_bindgen]
pub fn compile(content: &str) {
    let func_or_err = compiler::Compiler::compile(
        content.to_string(),
        firnas_ext::Extensions {
            lists: false,
            lambdas: false,
        },
    );

    let _ = func_or_err.map(|f| {
        let mut vm = virtual_machine::VirtualMachine::new(Box::new(WasmStdIO));
        vm.interpret(f).unwrap();
    });
}
