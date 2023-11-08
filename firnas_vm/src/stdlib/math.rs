use super::StdFunc;
use crate::value;
use crate::value::NativeFunction;
use crate::virtual_machine;

pub fn std_math_exp() -> StdFunc {
    let name = if cfg!(feature = "ar") {
        String::from("التوزيع_الأسي")
    } else {
        String::from("exp")
    };

    StdFunc {
        name: name.clone(),
        func: value::Value::NativeFunction(NativeFunction {
            arity: 1,
            name,
            func: exp,
        }),
    }
}

pub fn std_math_sqrt() -> StdFunc {
    let name = if cfg!(feature = "ar") {
        String::from("الجذر_التربيعي")
    } else {
        String::from("sqrt")
    };

    StdFunc {
        name: name.clone(),
        func: value::Value::NativeFunction(NativeFunction {
            arity: 1,
            name,
            func: sqrt,
        }),
    }
}

fn exp(
    _interp: &mut virtual_machine::VirtualMachine,
    args: &[value::Value],
) -> Result<value::Value, String> {
    match args[0] {
        value::Value::Number(num) => Ok(value::Value::Number(num.exp())),
        _ => Err(format!(
            "Invalid call: expected number, got {:?}.",
            value::type_of(&args[0])
        )),
    }
}

fn sqrt(
    _interp: &mut virtual_machine::VirtualMachine,
    args: &[value::Value],
) -> Result<value::Value, String> {
    match args[0] {
        value::Value::Number(num) => Ok(value::Value::Number(num.sqrt())),
        _ => Err(format!(
            "Invalid call: expected number, got {:?}.",
            value::type_of(&args[0])
        )),
    }
}
