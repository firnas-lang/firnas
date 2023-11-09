use firnas_compiler::compiler::{Compiler, Error};
use firnas_vm::virtual_machine::VirtualMachine;
use firnas_vm::virtual_machine::VmError;

#[macro_export]
macro_rules! vec_of_strings {
    ($($x:expr),*) => (vec![$($x.to_string()),*]);
}

pub fn evaluate(code: &str, extensions: firnas_ext::Extensions) -> Result<Vec<String>, String> {
    let func_or_err = Compiler::compile(String::from(code), extensions);

    match func_or_err {
        Ok(func) => {
            let mut vm = VirtualMachine::default();
            let res = vm.interpret(func);
            match res {
                Ok(()) => Ok(vm.get_output()),
                Err(VmError::Runtime(err)) => Err(err),
            }
        }
        Err(Error::Lexical(err)) => Err(err.what),
        Err(Error::Parse(err)) => Err(err.what),
        Err(Error::Semantic(err)) => Err(err.what),
        Err(Error::Internal(err)) => Err(err),
    }
}

pub fn check_output(code: &str, extensions: firnas_ext::Extensions, expected_output: &[String]) {
    let res = evaluate(code, extensions);

    match res {
        Ok(output) => assert_eq!(output, expected_output),
        Err(err) => panic!("{}", err),
    }
}

pub fn check_output_default(code: &str, expected_output: &[String]) {
    check_output(code, firnas_ext::Extensions::default(), expected_output);
}

pub fn check_output_lists(code: &str, expected_output: &[String]) {
    check_output(
        code,
        firnas_ext::Extensions {
            lists: true,
            ..Default::default()
        },
        expected_output,
    );
}

pub fn check_error(code: &str, extensions: firnas_ext::Extensions, f: &dyn Fn(&str) -> ()) {
    let res = evaluate(code, extensions);

    match res {
        Ok(output) => panic!("{:?}", output),
        Err(err) => f(&err),
    }
}

pub fn check_error_default(code: &str, f: &dyn Fn(&str) -> ()) {
    check_error(code, firnas_ext::Extensions::default(), f);
}
