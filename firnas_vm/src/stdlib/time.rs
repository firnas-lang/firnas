use crate::value;
use crate::virtual_machine;
use std::time::SystemTime;
use std::time::UNIX_EPOCH;

pub fn clock(
    _interp: &mut virtual_machine::VirtualMachine,
    _args: &[value::Value],
) -> Result<value::Value, String> {
    let start = SystemTime::now();
    let since_the_epoch = start.duration_since(UNIX_EPOCH).unwrap();

    Ok(value::Value::Number(since_the_epoch.as_millis() as f64))
}
