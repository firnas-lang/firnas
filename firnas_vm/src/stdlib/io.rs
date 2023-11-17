use super::StdFunc;
use crate::value;
use crate::value::NativeFunction;
use crate::virtual_machine;

pub trait StdIO {
    fn print(&self, content: &str);
    fn println(&self, content: &str);
}

pub struct DefaultStdIO;

impl StdIO for DefaultStdIO {
    fn print(&self, content: &str) {
        print!("{content}");
    }

    fn println(&self, content: &str) {
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
    match &args[0] {
        value::Value::String(id) => {
            let output = vm.heap.get_str(*id).clone();
            vm.push_output(output.clone());
            vm.std_io.println(&output);
        }
        value::Value::Number(num) => {
            let output: String = make_number(*num);
            vm.push_output(output.clone());
            vm.std_io.println(&output);
        }
        value::Value::Bool(b) => {
            let output = make_bool(*b);
            vm.push_output(output.clone());
            vm.std_io.println(&output);
        }
        value::Value::Nil => {
            let output = make_nil();
            vm.push_output(output.clone());
            vm.std_io.println(&output);
        }
        value::Value::Function(id) => {
            let output = format!("<fn '{}'>", vm.heap.get_closure(*id).clone().function.name);
            vm.push_output(output.clone());
            vm.std_io.println(&output);
        }
        value::Value::Instance(id) => {
            let instance = vm.heap.get_instance(*id);
            let class_name = &vm.heap.get_class(instance.class_id).name;
            let output = format!("<{} instance>", class_name);
            vm.push_output(output.clone());
            vm.std_io.println(&output);
        }
        value::Value::BoundMethod(id) => {
            let bound_method = vm.heap.get_bound_method(*id);
            let instance = vm.heap.get_instance(bound_method.instance_id);
            let class_name = &vm.heap.get_class(instance.class_id).name;
            let output = format!("<bound method of {} instance>", class_name);
            vm.push_output(output.clone());
            vm.std_io.println(&output);
        }
        value::Value::Class(id) => {
            let output = format!("<class '{}'>", vm.heap.get_class(*id).name);
            vm.push_output(output.clone());
            vm.std_io.println(&output);
        }
        value::Value::NativeFunction(func) => {
            let output = format!("<native fn {}>", func.name);
            vm.push_output(output.clone());
            vm.std_io.println(&output);
        }
        value::Value::List(_) => todo!(),
    };
    Ok(value::Value::Nil)
}

fn print(
    vm: &mut virtual_machine::VirtualMachine,
    args: &[value::Value],
) -> Result<value::Value, String> {
    match &args[0] {
        value::Value::String(id) => {
            let output = vm.heap.get_str(*id).clone();
            vm.push_output(output.clone());
            vm.std_io.print(&output);
        }
        value::Value::Number(num) => {
            let output: String = make_number(*num);
            vm.push_output(output.clone());
            vm.std_io.print(&output);
        }
        value::Value::Bool(b) => {
            let output = make_bool(*b);
            vm.push_output(output.clone());
            vm.std_io.print(&output);
        }
        value::Value::Nil => {
            let output = make_nil();
            vm.push_output(output.clone());
            vm.std_io.print(&output);
        }
        value::Value::Function(id) => {
            let output = format!("<fn '{}'>", vm.heap.get_closure(*id).clone().function.name);
            vm.push_output(output.clone());
            vm.std_io.print(&output);
        }
        value::Value::Instance(id) => {
            let instance = vm.heap.get_instance(*id);
            let class_name = &vm.heap.get_class(instance.class_id).name;
            let output = format!("<{} instance>", class_name);
            vm.push_output(output.clone());
            vm.std_io.print(&output);
        }
        value::Value::BoundMethod(id) => {
            let bound_method = vm.heap.get_bound_method(*id);
            let instance = vm.heap.get_instance(bound_method.instance_id);
            let class_name = &vm.heap.get_class(instance.class_id).name;
            let output = format!("<bound method of {} instance>", class_name);
            vm.push_output(output.clone());
            vm.std_io.print(&output);
        }
        value::Value::Class(id) => {
            let output = format!("<class '{}'>", vm.heap.get_class(*id).name);
            vm.push_output(output.clone());
            vm.std_io.print(&output);
        }
        value::Value::NativeFunction(func) => {
            let output = format!("<native fn {}>", func.name);
            vm.push_output(output.clone());
            vm.std_io.print(&output);
        }
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
