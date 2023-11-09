use firnas_compiler::compiler;
use firnas_vm::stdlib::io::StdIO;
use firnas_vm::virtual_machine;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    // The `console.log` is quite polymorphic, so we can bind it with multiple
    // signatures. Note that we need to use `js_name` to ensure we always call
    // `log` in JS.
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_u32(a: u32);

    // Multiple arguments too!
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_many(a: &str, b: &str);
}

struct WasmStdIO;

impl StdIO for WasmStdIO {
    fn print(&self, content: &str) {
        log(&format!("{content}"));
    }
}

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
        let mut vm = virtual_machine::VirtualMachine::new(Box::new(WasmStdIO));
        vm.interpret(f).unwrap();
    });

    String::from("Done")
}
