use super::StdFunc;
use crate::value;
use crate::value::NativeFunction;
use crate::virtual_machine;

pub trait StdIO {
    fn print(&self, content: &str);
}

pub struct DefaultStdIO;

impl StdIO for DefaultStdIO {
    fn print(&self, content: &str) {
        println!("{content}");
    }
}

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
    // Todo: rename it to normal print when removing the print statement
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
            let output = vm.heap.get_str(idx).clone();
            vm.push_output(output.clone());
            vm.std_io.print(&output);
        }
        value::Value::Number(num) => {
            let output: String = make_number(num);
            vm.push_output(output.clone());
            vm.std_io.print(&output);
        }
        value::Value::Bool(b) => {
            let output = make_bool(b);
            vm.push_output(output.clone());
            vm.std_io.print(&output);
        }
        value::Value::Nil => {
            let output = make_nil();
            vm.push_output(output.clone());
            vm.std_io.print(&output);
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
            print!("{}", make_number(num))
        }
        value::Value::Bool(b) => {
            print!("{}", make_bool(b))
        }
        value::Value::Nil => {
            print!("{}", make_nil())
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

fn make_number(num: f64) -> String {
    #[cfg(feature = "ar")]
    {
        use arabic_utils::arabic_number::ArabicNumber;
        num.to_arabic_decimal().unwrap()
    }
    #[cfg(not(feature = "ar"))]
    {
        format!("{num}")
    }
}

fn make_bool(b: bool) -> String {
    let (t, f) = if cfg!(feature = "ar") {
        (String::from("صحيح"), String::from("خطا"))
    } else {
        (String::from("true"), String::from("false"))
    };
    match b {
        true => t,
        false => f,
    }
}

fn make_nil() -> String {
    if cfg!(feature = "ar") {
        String::from("عدم")
    } else {
        String::from("nil")
    }
}
