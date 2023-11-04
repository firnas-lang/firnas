use crate::value;
use crate::virtual_machine;

pub fn len(
    interp: &mut virtual_machine::VirtualMachine,
    args: &[value::Value],
) -> Result<value::Value, String> {
    match &args[0] {
        value::Value::String(id) => Ok(value::Value::Number(interp.heap.get_str(*id).len() as f64)),
        value::Value::List(id) => Ok(value::Value::Number(
            interp.heap.get_list_elements(*id).len() as f64,
        )),
        val => Err(format!(
            "Object of type {:?} has no len.",
            value::type_of(val)
        )),
    }
}

pub fn for_each(
    interp: &mut virtual_machine::VirtualMachine,
    args: &[value::Value],
) -> Result<value::Value, String> {
    match &args[0] {
        value::Value::List(id) => {
            let list_elements = interp.heap.get_list_elements(*id).clone();
            let callable = args[1].clone();
            for element in list_elements.iter() {
                interp.stack.push(callable.clone());
                interp.stack.push(element.clone());

                // stash the current frame number if we're going to call a pure firnas function ...
                let frame_idx = interp.frames.len();

                if let Err(virtual_machine::VmError::Runtime(err)) =
                    interp.call_value(callable.clone(), 1)
                {
                    return Err(err);
                }

                // If we're calling a pure firnas function, `interp.call_value` doesn't actually
                // call the value, it just sets up a call frame. We loop the interpreter
                // until it his an error or returns to the call frame with `frame_idx`.
                // Unfortunately, this doesn't play well with our current debugger
                // implementation, which manually calls `interpreter.step()`
                loop {
                    if interp.frames.len() == frame_idx {
                        break;
                    }

                    if let Err(virtual_machine::VmError::Runtime(err)) = interp.step() {
                        return Err(err);
                    }
                }
            }
            Ok(value::Value::Nil)
        }
        val => Err(format!(
            "Can't call forEach on value of type {:?}.",
            value::type_of(val)
        )),
    }
}

pub fn map(
    interp: &mut virtual_machine::VirtualMachine,
    args: &[value::Value],
) -> Result<value::Value, String> {
    match &args[1] {
        value::Value::List(id) => {
            let list_elements = interp.heap.get_list_elements(*id).clone();
            let callable = args[0].clone();
            let mut res_elements = Vec::new();
            for element in list_elements.iter() {
                interp.stack.push(callable.clone());
                interp.stack.push(element.clone());

                //stash the current frame number if we're going to call a pure firnas function ...
                let frame_idx = interp.frames.len();

                if let Err(virtual_machine::VmError::Runtime(err)) =
                    interp.call_value(callable.clone(), 1)
                {
                    return Err(err);
                }

                // If we're calling a pure firnas function, `interp.call_value` doesn't actually
                // call the value, it just sets up a call frame. We loop the interpreter
                // until it his an error or returns to the call frame with `frame_idx`.
                // Unfortunately, this doesn't play well with our current debugger
                // implementation, which manually calls `interpreter.step()`
                loop {
                    if interp.frames.len() == frame_idx {
                        break;
                    }

                    if let Err(virtual_machine::VmError::Runtime(err)) = interp.step() {
                        return Err(err);
                    }
                }

                res_elements.push(interp.pop_stack());
            }
            Ok(value::Value::List(interp.heap.manage_list(res_elements)))
        }
        val => Err(format!(
            "Can't call forEach on value of type {:?}.",
            value::type_of(val)
        )),
    }
}
