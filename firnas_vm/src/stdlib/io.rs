use crate::value;
use crate::value::NativeFunction;
use crate::virtual_machine;

use super::StdFunc;

pub fn std_io_print_line() -> StdFunc {
    let name = if cfg!(feature = "ar") {
        String::from("اطبع_سطر")
    } else {
        String::from("printLine")
    };

    StdFunc {
        name: name.clone(),
        func: value::Value::NativeFunction(NativeFunction {
            arity: 1,
            name,
            func: print_line,
        }),
    }
}

pub fn std_io_print() -> StdFunc {
    let name = if cfg!(feature = "ar") {
        String::from("اطبع_س")
    } else {
        String::from("printL")
    };

    StdFunc {
        name: name.clone(),
        func: value::Value::NativeFunction(NativeFunction {
            arity: 1,
            name,
            func: print,
        }),
    }
}

fn print_line(
    vm: &mut virtual_machine::VirtualMachine,
    args: &[value::Value],
) -> Result<value::Value, String> {
    match args[0] {
        value::Value::String(idx) => {
            println!("{}", vm.heap.get_str(idx))
        }
        value::Value::Number(num) => {
            println!("{}", num)
        }
        value::Value::Bool(b) => {
            println!("{}", b)
        }
        value::Value::Nil => {
            println!("nil")
        }
        value::Value::Function(_) => todo!(),
        value::Value::Instance(_) => todo!(),
        value::Value::BoundMethod(_) => todo!(),
        value::Value::Class(_) => todo!(),
        value::Value::NativeFunction(_) => todo!(),
        value::Value::List(_) => todo!(),
    };
    Ok(value::Value::Nil)
}

fn print(
    vm: &mut virtual_machine::VirtualMachine,
    args: &[value::Value],
) -> Result<value::Value, String> {
    match args[0] {
        value::Value::String(idx) => {
            print!("{}", vm.heap.get_str(idx))
        }
        value::Value::Number(num) => {
            print!("{}", num)
        }
        value::Value::Bool(b) => {
            print!("{}", b)
        }
        value::Value::Nil => {
            print!("nil")
        }
        value::Value::Function(_) => todo!(),
        value::Value::Instance(_) => todo!(),
        value::Value::BoundMethod(_) => todo!(),
        value::Value::Class(_) => todo!(),
        value::Value::NativeFunction(_) => todo!(),
        value::Value::List(_) => todo!(),
    };
    Ok(value::Value::Nil)
}
