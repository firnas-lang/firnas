use crate::value;
use crate::value::NativeFunction;
use crate::virtual_machine;
use std::time::SystemTime;
use std::time::UNIX_EPOCH;

pub fn std_time_clock() -> (String, value::Value) {
    let name = if cfg!(feature = "ar") {
        String::from("ساعة")
    } else {
        String::from("clock")
    };

    (
        name.clone(),
        value::Value::NativeFunction(NativeFunction {
            arity: 0,
            name,
            func: clock,
        }),
    )
}

fn clock(
    _interp: &mut virtual_machine::VirtualMachine,
    _args: &[value::Value],
) -> Result<value::Value, String> {
    let start = SystemTime::now();
    let since_the_epoch = start.duration_since(UNIX_EPOCH).unwrap();

    Ok(value::Value::Number(since_the_epoch.as_millis() as f64))
}
