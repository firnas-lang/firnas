use crate::gc;
use crate::stdlib;
use crate::stdlib::io::std_io_print;
use crate::stdlib::io::std_io_print_line;
use crate::stdlib::io::DefaultStdIO;
use crate::stdlib::io::StdIO;
use crate::stdlib::math::std_math_exp;
use crate::stdlib::math::std_math_sqrt;
use crate::stdlib::time::std_time_clock;
use crate::stdlib::StdFunc;
use crate::value;
use firnas_bytecode;
use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt;
use std::rc::Rc;

#[derive(Debug)]
enum Binop {
    Add,
    Sub,
    Mul,
    Div,
}

pub struct VirtualMachine {
    pub frames: Vec<CallFrame>,
    pub stack: Vec<value::Value>,
    output: Vec<String>,
    pub globals: HashMap<String, value::Value>,
    pub upvalues: Vec<Rc<RefCell<value::Upvalue>>>,
    pub heap: gc::Heap,
    gray_stack: Vec<gc::HeapId>,
    pub std_io: Box<dyn StdIO>,
}

impl VirtualMachine {
    pub fn new(std_io: Box<dyn StdIO>) -> Self {
        let mut res = VirtualMachine {
            frames: Default::default(),
            stack: Default::default(),
            output: Default::default(),
            globals: Default::default(),
            upvalues: Default::default(),
            heap: Default::default(),
            gray_stack: Default::default(),
            std_io,
        };
        res.stack.reserve(256);
        res.frames.reserve(64);

        res.add_std_func(std_io_print());
        res.add_std_func(std_io_print_line());

        res.add_std_func(std_time_clock());

        res.add_std_func(std_math_exp());
        res.add_std_func(std_math_sqrt());
        res
    }

    pub fn get_output(&self) -> Vec<String> {
        self.output.clone()
    }

    pub fn push_output(&mut self, output: String) {
        self.output.push(output);
    }

    fn add_std_func(&mut self, std_func: StdFunc) {
        self.globals.insert(std_func.name, std_func.func);
    }
}

impl Default for VirtualMachine {
    fn default() -> VirtualMachine {
        let mut res = VirtualMachine {
            frames: Default::default(),
            stack: Default::default(),
            output: Default::default(),
            globals: Default::default(),
            upvalues: Default::default(),
            heap: Default::default(),
            gray_stack: Default::default(),
            std_io: Box::new(DefaultStdIO),
        };
        res.stack.reserve(256);
        res.frames.reserve(64);

        res.add_std_func(std_io_print());
        res.add_std_func(std_io_print_line());

        res.add_std_func(std_time_clock());

        res.add_std_func(std_math_exp());
        res.add_std_func(std_math_sqrt());

        res.globals.insert(
            String::from("dis"),
            value::Value::NativeFunction(value::NativeFunction {
                arity: 1,
                name: String::from("dis"),
                func: stdlib::debug::dis_builtin,
            }),
        );
        res.globals.insert(
            String::from("len"),
            value::Value::NativeFunction(value::NativeFunction {
                arity: 1,
                name: String::from("len"),
                func: stdlib::collection::len,
            }),
        );
        res.globals.insert(
            String::from("forEach"),
            value::Value::NativeFunction(value::NativeFunction {
                arity: 2,
                name: String::from("forEach"),
                func: stdlib::collection::for_each,
            }),
        );
        res.globals.insert(
            String::from("map"),
            value::Value::NativeFunction(value::NativeFunction {
                arity: 2,
                name: String::from("map"),
                func: stdlib::collection::map,
            }),
        );

        res
    }
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub enum VmError {
    Runtime(String),
}

impl fmt::Display for VmError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            VmError::Runtime(err) => write!(f, "Lox runtime error: {}", err),
        }
    }
}

#[derive(Default)]
pub struct CallFrame {
    pub closure: value::Closure,
    pub ip: usize,
    pub slots_offset: usize,
}

impl CallFrame {
    fn next_op(&self) -> (firnas_bytecode::Op, firnas_bytecode::Lineno) {
        self.closure.function.chunk.code[self.ip].clone()
    }

    fn next_op_and_advance(&mut self) -> (firnas_bytecode::Op, firnas_bytecode::Lineno) {
        let res = self.next_op();
        self.ip += 1;
        res
    }

    fn read_constant(&self, idx: usize) -> firnas_bytecode::Constant {
        self.closure.function.chunk.constants[idx].clone()
    }
}

impl VirtualMachine {
    pub fn prepare_interpret(&mut self, func: firnas_bytecode::Function) {
        self.stack
            .push(value::Value::Function(self.heap.manage_closure(
                value::Closure {
                    function: func.clone(),
                    upvalues: Vec::new(),
                },
            )));
        self.frames.push(CallFrame {
            closure: value::Closure {
                function: func,
                upvalues: Vec::new(),
            },
            ip: 0,
            slots_offset: 1,
        });
    }

    pub fn interpret(&mut self, func: firnas_bytecode::Function) -> Result<(), VmError> {
        self.prepare_interpret(func);
        self.run()
    }

    pub fn format_backtrace(&self) -> String {
        let lines: Vec<_> = self
            .frames
            .iter()
            .map(|frame| {
                let frame_name = &frame.closure.function.name;
                let (_, lineno) = frame.closure.function.chunk.code[frame.ip];
                if frame_name.is_empty() {
                    format!("[line {}] in script", lineno.value)
                } else {
                    format!("[line {}] in {}()", lineno.value, frame_name)
                }
            })
            .collect();
        format!("Backtrace (most recent call last):\n\n{}", lines.join("\n"))
    }

    pub fn format_upval(&self, val: &value::Upvalue) -> String {
        match val {
            value::Upvalue::Open(idx) => format!("Open({})", idx),
            value::Upvalue::Closed(val) => format!("Closed({})", self.format_val(val)),
        }
    }

    pub fn format_val(&self, val: &value::Value) -> String {
        match val {
            value::Value::Number(num) => num.to_string(),
            value::Value::Bool(b) => b.to_string(),
            value::Value::String(str_handle) => self.get_str(*str_handle).clone(),
            value::Value::Function(closure_handle) => {
                format!("<fn '{}'>", self.get_closure(*closure_handle).function.name)
            }
            value::Value::Class(class_handle) => {
                format!("<class '{}'>", self.get_class(*class_handle).name)
            }
            value::Value::Instance(instance_handle) => {
                let instance = self.get_instance(*instance_handle);
                let class_name = &self.get_class(instance.class_id).name;
                format!("<{} instance>", class_name)
            }
            value::Value::NativeFunction(func) => format!("<native fn {}>", func.name),
            value::Value::BoundMethod(bound_method_id) => {
                let bound_method = self.get_bound_method(*bound_method_id);
                let instance = self.get_instance(bound_method.instance_id);
                let class_name = &self.get_class(instance.class_id).name;
                format!("<bound method of {} instance>", class_name)
            }
            value::Value::Nil => "nil".to_string(),
            value::Value::List(list_id) => {
                let elements = self.get_list_elements(*list_id);
                format!(
                    "[{}]",
                    elements
                        .iter()
                        .map(|element| self.format_val(element))
                        .collect::<Vec<String>>()
                        .join(", ")
                )
            }
        }
    }

    fn run(&mut self) -> Result<(), VmError> {
        loop {
            if self.is_done() {
                return Ok(());
            }

            self.step()?;
        }
    }

    pub fn is_done(&self) -> bool {
        self.frames.is_empty() || self.frame().ip >= self.frame().closure.function.chunk.code.len()
    }

    pub fn step(&mut self) -> Result<(), VmError> {
        let op = self.next_op_and_advance();

        if self.heap.should_collect() {
            self.collect_garbage();
        }

        match op {
            (firnas_bytecode::Op::Return, _) => {
                let result = self.pop_stack();

                for idx in self.frame().slots_offset..self.stack.len() {
                    self.close_upvalues(idx);
                }

                if self.frames.len() <= 1 {
                    self.frames.pop();
                    return Ok(());
                }

                let num_to_pop = self.stack.len() - self.frame().slots_offset
                    + usize::from(self.frame().closure.function.arity);
                self.frames.pop();

                self.pop_stack_n_times(num_to_pop);

                self.stack.push(result);
            }
            (firnas_bytecode::Op::Closure(idx, upvals), _) => {
                let constant = self.read_constant(idx);

                if let value::Value::Function(closure_handle) = constant {
                    let closure = self.get_closure(closure_handle).clone();
                    let upvalues = upvals
                        .iter()
                        .map(|upval| match upval {
                            firnas_bytecode::UpvalueLoc::Upvalue(idx) => {
                                self.frame().closure.upvalues[*idx].clone()
                            }
                            firnas_bytecode::UpvalueLoc::Local(idx) => {
                                if let Some(upval) = self.find_open_uval(*idx) {
                                    upval
                                } else {
                                    let index = self.frame().slots_offset + *idx - 1;
                                    let upval = Rc::new(RefCell::new(value::Upvalue::Open(index)));
                                    self.upvalues.push(upval.clone());
                                    upval
                                }
                            }
                        })
                        .collect();

                    self.stack
                        .push(value::Value::Function(self.heap.manage_closure(
                            value::Closure {
                                function: closure.function,
                                upvalues,
                            },
                        )));
                } else {
                    panic!(
                        "When interpreting firnas_bytecode::Op::Closure, expected function, found {:?}",
                        value::type_of(&constant)
                    );
                }
            }
            (firnas_bytecode::Op::Constant(idx), _) => {
                let constant = self.read_constant(idx);
                self.stack.push(constant);
            }
            (firnas_bytecode::Op::Nil, _) => {
                self.stack.push(value::Value::Nil);
            }
            (firnas_bytecode::Op::True, _) => {
                self.stack.push(value::Value::Bool(true));
            }
            (firnas_bytecode::Op::False, _) => {
                self.stack.push(value::Value::Bool(false));
            }
            (firnas_bytecode::Op::Negate, lineno) => {
                let top_stack = self.peek();
                let maybe_number = VirtualMachine::extract_number(top_stack);

                match maybe_number {
                        Some(to_negate) => {
                            self.pop_stack();
                            self.stack.push(value::Value::Number(-to_negate));
                        }
                        None => {
                            return Err(VmError::Runtime(format!(
                                "invalid operand to unary op negate. Expected number, found {:?} at line {}",
                                value::type_of(top_stack), lineno.value
                            )))
                        }
                    }
            }
            (firnas_bytecode::Op::Add, lineno) => {
                let val1 = self.peek_by(0).clone();
                let val2 = self.peek_by(1).clone();

                match (&val1, &val2) {
                    (value::Value::Number(_), value::Value::Number(_)) => {
                        self.numeric_binop(Binop::Add, lineno)?
                    }
                    (value::Value::String(s1), value::Value::String(s2)) => {
                        self.pop_stack();
                        self.pop_stack();
                        self.stack
                            .push(value::Value::String(self.heap.manage_str(format!(
                                "{}{}",
                                self.get_str(*s2),
                                self.get_str(*s1)
                            ))));
                    }
                    (value::Value::List(id1), value::Value::List(id2)) => {
                        self.pop_stack();
                        self.pop_stack();
                        let mut res = self.get_list_elements(*id2).clone();
                        res.extend(self.get_list_elements(*id1).clone());
                        self.stack
                            .push(value::Value::List(self.heap.manage_list(res)));
                    }
                    _ => {
                        return Err(VmError::Runtime(format!(
                            "invalid operands of type {:?} and {:?} in add expression: \
                                 both operands must be number or string (line={})",
                            value::type_of(&val1),
                            value::type_of(&val2),
                            lineno.value
                        )))
                    }
                }
            }
            (firnas_bytecode::Op::Subtract, lineno) => match self.numeric_binop(Binop::Sub, lineno)
            {
                Ok(()) => {}
                Err(err) => return Err(err),
            },
            (firnas_bytecode::Op::Multiply, lineno) => match self.numeric_binop(Binop::Mul, lineno)
            {
                Ok(()) => {}
                Err(err) => return Err(err),
            },
            (firnas_bytecode::Op::Divide, lineno) => match self.numeric_binop(Binop::Div, lineno) {
                Ok(()) => {}
                Err(err) => return Err(err),
            },
            (firnas_bytecode::Op::Not, lineno) => {
                let top_stack = self.peek();
                let maybe_bool = VirtualMachine::extract_bool(top_stack);

                match maybe_bool {
                        Some(b) => {
                            self.pop_stack();
                            self.stack.push(value::Value::Bool(!b));
                        }
                        None => {
                            return Err(VmError::Runtime(format!(
                                "invalid operand in not expression. Expected boolean, found {:?} at line {}",
                                value::type_of(top_stack), lineno.value)))
                        }
                    }
            }
            (firnas_bytecode::Op::Equal, _) => {
                let val1 = self.pop_stack();
                let val2 = self.pop_stack();
                self.stack
                    .push(value::Value::Bool(self.values_equal(&val1, &val2)));
            }
            (firnas_bytecode::Op::Greater, lineno) => {
                let val1 = self.peek_by(0).clone();
                let val2 = self.peek_by(1).clone();

                match (&val1, &val2) {
                        (value::Value::Number(n1), value::Value::Number(n2)) => {
                            self.pop_stack();
                            self.pop_stack();

                            self.stack.push(value::Value::Bool(n2 > n1));
                        }
                        _ => return Err(VmError::Runtime(format!(
                            "invalid operands in Greater expression. Expected numbers, found {:?} and {:?} at line {}",
                            value::type_of(&val1), value::type_of(&val2), lineno.value)))

                    }
            }
            (firnas_bytecode::Op::Less, lineno) => {
                let val1 = self.peek_by(0).clone();
                let val2 = self.peek_by(1).clone();

                match (&val1, &val2) {
                        (value::Value::Number(n1), value::Value::Number(n2)) => {
                            self.pop_stack();
                            self.pop_stack();
                            self.stack.push(value::Value::Bool(n2 < n1));
                        }
                        _ => return Err(VmError::Runtime(format!(
                            "invalid operands in Less expression. Expected numbers, found {:?} and {:?} at line {}",
                            value::type_of(&val1), value::type_of(&val2), lineno.value)))

                    }
            }
            (firnas_bytecode::Op::Print, _) => {
                let to_print = self.peek().clone();
                self.print_val(&to_print);
            }
            (firnas_bytecode::Op::Pop, _) => {
                self.pop_stack();
            }
            (firnas_bytecode::Op::DefineGlobal(idx), _) => {
                if let value::Value::String(name_id) = self.read_constant(idx) {
                    let val = self.pop_stack();
                    self.globals.insert(self.get_str(name_id).clone(), val);
                } else {
                    panic!(
                        "expected string when defining global, found {:?}",
                        value::type_of(&self.read_constant(idx))
                    );
                }
            }
            (firnas_bytecode::Op::GetGlobal(idx), lineno) => {
                if let value::Value::String(name_id) = self.read_constant(idx) {
                    match self.globals.get(self.get_str(name_id)) {
                        Some(val) => {
                            self.stack.push(val.clone());
                        }
                        None => {
                            return Err(VmError::Runtime(format!(
                                "Undefined variable '{}' at line {}.",
                                self.get_str(name_id),
                                lineno.value
                            )));
                        }
                    }
                } else {
                    panic!(
                        "expected string when defining global, found {:?}",
                        value::type_of(&self.read_constant(idx))
                    );
                }
            }
            (firnas_bytecode::Op::SetGlobal(idx), lineno) => {
                if let value::Value::String(name_id) = self.read_constant(idx) {
                    let name_str = self.get_str(name_id).clone();
                    let val = self.peek().clone();
                    if let std::collections::hash_map::Entry::Occupied(mut e) =
                        self.globals.entry(name_str.clone())
                    {
                        e.insert(val);
                    } else {
                        return Err(VmError::Runtime(format!(
                            "Use of undefined variable {} in setitem expression at line {}.",
                            name_str, lineno.value
                        )));
                    }
                } else {
                    panic!(
                        "expected string when setting global, found {:?}",
                        value::type_of(&self.read_constant(idx))
                    );
                }
            }
            (firnas_bytecode::Op::GetLocal(idx), _) => {
                let slots_offset = self.frame().slots_offset;
                let val = self.stack[slots_offset + idx - 1].clone();
                self.stack.push(val);
            }
            (firnas_bytecode::Op::SetLocal(idx), _) => {
                let val = self.peek();
                let slots_offset = self.frame().slots_offset;
                self.stack[slots_offset + idx - 1] = val.clone();
            }
            (firnas_bytecode::Op::GetUpval(idx), _) => {
                let upvalue = self.frame().closure.upvalues[idx].clone();
                let val = match &*upvalue.borrow() {
                    value::Upvalue::Closed(value) => value.clone(),
                    value::Upvalue::Open(stack_index) => self.stack[*stack_index].clone(),
                };
                self.stack.push(val);
            }
            (firnas_bytecode::Op::SetUpval(idx), _) => {
                let new_value = self.peek().clone();
                let upvalue = self.frame().closure.upvalues[idx].clone();
                match &mut *upvalue.borrow_mut() {
                    value::Upvalue::Closed(value) => *value = new_value,
                    value::Upvalue::Open(stack_index) => self.stack[*stack_index] = new_value,
                };
            }
            (firnas_bytecode::Op::JumpIfFalse(offset), _) => {
                if self.is_falsey(self.peek()) {
                    self.frame_mut().ip += offset;
                }
            }
            (firnas_bytecode::Op::Jump(offset), _) => {
                self.frame_mut().ip += offset;
            }
            (firnas_bytecode::Op::Loop(offset), _) => {
                self.frame_mut().ip -= offset;
            }
            (firnas_bytecode::Op::Call(arg_count), _) => {
                self.call_value(self.peek_by(arg_count.into()).clone(), arg_count)?;
            }
            (firnas_bytecode::Op::CloseUpvalue, _) => {
                let idx = self.stack.len() - 1;
                self.close_upvalues(idx);
                self.stack.pop();
            }
            (firnas_bytecode::Op::Class(idx), _) => {
                if let value::Value::String(name_id) = self.read_constant(idx) {
                    let name = self.get_str(name_id).clone();
                    self.stack
                        .push(value::Value::Class(self.heap.manage_class(value::Class {
                            name,
                            methods: HashMap::new(),
                        })));
                } else {
                    panic!(
                        "expected string when defining class, found {:?}",
                        value::type_of(&self.read_constant(idx))
                    );
                }
            }
            (firnas_bytecode::Op::SetProperty(idx), _) => {
                if let value::Value::String(attr_id) = self.read_constant(idx) {
                    let val = self.pop_stack();
                    let instance = self.pop_stack();
                    self.setattr(instance, val.clone(), attr_id)?;
                    self.stack.push(val);
                } else {
                    panic!(
                        "expected string when setting property, found {:?}",
                        value::type_of(&self.read_constant(idx))
                    )
                }
            }
            (firnas_bytecode::Op::GetProperty(idx), _) => {
                if let value::Value::String(attr_id) = self.read_constant(idx) {
                    let maybe_instance = self.peek().clone();

                    let (class_id, instance_id) = match maybe_instance {
                        value::Value::Instance(instance_id) => {
                            let instance = self.heap.get_instance(instance_id).clone();
                            (instance.class_id, instance_id)
                        }
                        _ => panic!(),
                    };

                    let class = self.heap.get_class(class_id).clone();
                    if let Some(attr) = self.getattr(maybe_instance.clone(), attr_id)? {
                        self.pop_stack();
                        self.stack.push(attr);
                    } else if !self.bind_method(instance_id, class, attr_id)? {
                        return Err(VmError::Runtime(format!(
                            "value {} has no attribute {}.",
                            self.format_val(&maybe_instance),
                            self.get_str(attr_id)
                        )));
                    }
                } else {
                    panic!(
                        "expected string when setting property, found {:?}",
                        value::type_of(&self.read_constant(idx))
                    )
                }
            }
            (firnas_bytecode::Op::Method(idx), _) => {
                if let value::Value::String(method_name_id) = self.read_constant(idx) {
                    let method_name = self.heap.get_str(method_name_id).clone();
                    let maybe_method = self.peek_by(0).clone();
                    let maybe_method_id = gc::Heap::extract_id(&maybe_method).unwrap();
                    let maybe_class = self.peek_by(1).clone();
                    match maybe_class {
                        value::Value::Class(class_id) => {
                            let class = self.heap.get_class_mut(class_id);
                            class.methods.insert(method_name, maybe_method_id);
                            self.pop_stack();
                        }
                        _ => {
                            panic!(
                                "should only define methods on a class! tried on {:?}",
                                self.format_val(&maybe_class)
                            );
                        }
                    }
                } else {
                    panic!("expected string when defining a method.");
                }
            }
            (firnas_bytecode::Op::Invoke(method_name, arg_count), _) => {
                self.invoke(&method_name, arg_count)?;
            }
            (firnas_bytecode::Op::Inherit, lineno) => {
                {
                    let (superclass_id, subclass_id) = match (self.peek_by(1), self.peek()) {
                        (value::Value::Class(superclass_id), value::Value::Class(subclass_id)) => {
                            (*superclass_id, *subclass_id)
                        }
                        (not_a_class, value::Value::Class(_)) => {
                            return Err(VmError::Runtime(format!(
                                "Superclass must be a class, found {:?} at lineno={:?}",
                                value::type_of(not_a_class),
                                lineno
                            )));
                        }
                        _ => panic!("expected classes when interpreting Inherit!"),
                    };

                    let superclass_methods = self.get_class(superclass_id).methods.clone();
                    let subclass = self.get_class_mut(subclass_id);

                    subclass.methods.extend(superclass_methods);
                }
                self.pop_stack(); //subclass
            }
            (firnas_bytecode::Op::GetSuper(idx), _) => {
                let method_id = if let value::Value::String(method_id) = self.read_constant(idx) {
                    method_id
                } else {
                    panic!();
                };

                let maybe_superclass = self.pop_stack();
                let superclass = match maybe_superclass {
                    value::Value::Class(class_id) => self.get_class(class_id).clone(),
                    _ => panic!(),
                };

                let maybe_instance = self.peek();
                let instance_id = match maybe_instance {
                    value::Value::Instance(instance_id) => *instance_id,
                    _ => panic!(),
                };

                if !self.bind_method(instance_id, superclass, method_id)? {
                    return Err(VmError::Runtime(format!(
                        "superclass {} has no attribute {}.",
                        self.format_val(&maybe_superclass),
                        self.get_str(method_id)
                    )));
                }
            }
            (firnas_bytecode::Op::SuperInvoke(method_name, arg_count), _) => {
                let maybe_superclass = self.pop_stack();
                let superclass_id = match maybe_superclass {
                    value::Value::Class(class_id) => class_id,
                    _ => panic!("{}", self.format_val(&maybe_superclass)),
                };
                self.invoke_from_class(superclass_id, &method_name, arg_count)?;
            }
            (firnas_bytecode::Op::BuildList(size), _) => {
                let mut list_elements = Vec::new();
                for _ in 0..size {
                    list_elements.push(self.pop_stack())
                }
                list_elements.reverse();
                self.stack
                    .push(value::Value::List(self.heap.manage_list(list_elements)));
            }
            (firnas_bytecode::Op::Subscr, lineno) => {
                let subscript = self.pop_stack();
                let value_to_subscript = self.pop_stack();
                let res = self.subscript(value_to_subscript, subscript, lineno)?;
                self.stack.push(res);
            }
            (firnas_bytecode::Op::SetItem, lineno) => {
                let rhs = self.pop_stack();
                let subscript = self.pop_stack();
                let lhs = self.pop_stack();
                self.setitem(lhs, subscript, rhs.clone(), lineno)?;
                self.stack.push(rhs);
            }
        }
        Ok(())
    }

    fn setitem(
        &mut self,
        lhs: value::Value,
        subscript: value::Value,
        rhs: value::Value,
        lineno: firnas_bytecode::Lineno,
    ) -> Result<(), VmError> {
        if let value::Value::List(id) = lhs {
            if let value::Value::Number(index_float) = subscript {
                let elements = self.get_list_elements_mut(id);
                match VirtualMachine::subscript_to_inbound_index(
                    elements.len(),
                    index_float,
                    lineno,
                ) {
                    Ok(index_int) => {
                        elements[index_int] = rhs;
                        Ok(())
                    }
                    Err(err) => Err(VmError::Runtime(err)),
                }
            } else {
                Err(VmError::Runtime(format!(
                    "Invalid subscript of type {:?} in subscript expression",
                    value::type_of(&lhs)
                )))
            }
        } else {
            Err(VmError::Runtime(format!(
                "Invalid value of type {:?} in subscript expression",
                value::type_of(&subscript)
            )))
        }
    }

    fn subscript(
        &mut self,
        value: value::Value,
        subscript: value::Value,
        lineno: firnas_bytecode::Lineno,
    ) -> Result<value::Value, VmError> {
        if let value::Value::List(id) = value {
            if let value::Value::Number(index_float) = subscript {
                let elements = self.get_list_elements(id);
                match VirtualMachine::subscript_to_inbound_index(
                    elements.len(),
                    index_float,
                    lineno,
                ) {
                    Ok(index_int) => Ok(elements[index_int].clone()),
                    Err(err) => Err(VmError::Runtime(err)),
                }
            } else {
                Err(VmError::Runtime(format!(
                    "Invalid subscript of type {:?} in subscript expression",
                    value::type_of(&value)
                )))
            }
        } else {
            Err(VmError::Runtime(format!(
                "Invalid value of type {:?} in subscript expression",
                value::type_of(&value)
            )))
        }
    }

    fn subscript_to_inbound_index(
        list_len: usize,
        index_float: f64,
        lineno: firnas_bytecode::Lineno,
    ) -> Result<usize, String> {
        let index_int = index_float as i64;
        if 0 <= index_int && index_int < list_len as i64 {
            return Ok(index_int as usize);
        }
        if index_int < 0 && -index_int <= list_len as i64 {
            return Ok((list_len as i64 + index_int) as usize);
        }
        Err(format!(
            "List subscript index out of range at {}",
            lineno.value
        ))
    }

    fn invoke(&mut self, method_name: &str, arg_count: u8) -> Result<(), VmError> {
        let receiver_id = match self.peek_by(arg_count.into()) {
            value::Value::Instance(id) => *id,
            _ => {
                return Err(VmError::Runtime("Only instances have methods.".to_string()));
            }
        };

        if let Some(field) = self
            .get_instance(receiver_id)
            .fields
            .get(&String::from(method_name))
            .cloned()
        {
            return self.call_value(field, arg_count);
        }

        let class_id = self.get_instance(receiver_id).class_id;
        self.invoke_from_class(class_id, method_name, arg_count)
    }

    fn frame_mut(&mut self) -> &mut CallFrame {
        let frames_len = self.frames.len();
        &mut self.frames[frames_len - 1]
    }

    pub fn maybe_frame(&self) -> Option<&CallFrame> {
        self.frames.last()
    }

    pub fn frame(&self) -> &CallFrame {
        self.maybe_frame().unwrap()
    }

    fn invoke_from_class(
        &mut self,
        class_id: gc::HeapId,
        method_name: &str,
        arg_count: u8,
    ) -> Result<(), VmError> {
        let method_id = match self
            .get_class(class_id)
            .methods
            .get(&String::from(method_name))
        {
            Some(method_id) => *method_id,
            None => {
                return Err(VmError::Runtime(format!(
                    "Undefined property {}.",
                    method_name
                )))
            }
        };

        self.call_value(value::Value::Function(method_id), arg_count)
    }

    fn close_upvalues(&mut self, index: usize) {
        let value = &self.stack[index];
        for upval in &self.upvalues {
            if upval.borrow().is_open_with_index(index) {
                upval.replace(value::Upvalue::Closed(value.clone()));
            }
        }

        self.upvalues.retain(|u| u.borrow().is_open());
    }

    fn find_open_uval(&self, index: usize) -> Option<Rc<RefCell<value::Upvalue>>> {
        for upval in self.upvalues.iter().rev() {
            if upval.borrow().is_open_with_index(index) {
                return Some(upval.clone());
            }
        }

        None
    }

    pub fn call_value(&mut self, val_to_call: value::Value, arg_count: u8) -> Result<(), VmError> {
        match val_to_call {
            value::Value::Function(func) => {
                self.prepare_call(func, arg_count)?;
                Ok(())
            }
            value::Value::NativeFunction(native_func) => {
                self.call_native_func(native_func, arg_count)?;
                Ok(())
            }
            value::Value::Class(class_id) => {
                let new_instance =
                    value::Value::Instance(self.heap.manage_instance(value::Instance {
                        class_id,
                        fields: HashMap::new(),
                    }));

                let arg_count_usize: usize = arg_count.into();
                let stack_len = self.stack.len();
                self.stack[stack_len - 1 - arg_count_usize] = new_instance;

                {
                    let init_name = if cfg!(feature = "en") {
                        "init"
                    } else {
                        "تهيئة"
                    };

                    let maybe_method_id = self
                        .get_class(class_id)
                        .methods
                        .get(&init_name.to_string())
                        .copied();

                    if let Some(method_id) = maybe_method_id {
                        return self.prepare_call(method_id, arg_count);
                    }
                }

                if arg_count > 0 {
                    return Err(VmError::Runtime(format!(
                        "Call to class ctor expected 0 arguments, got {}.",
                        arg_count,
                    )));
                }

                self.create_instance(class_id);
                Ok(())
            }
            value::Value::BoundMethod(method_id) => {
                self.call_bound_method(method_id, arg_count)?;
                Ok(())
            }
            _ => Err(VmError::Runtime(format!(
                "attempted to call non-callable value of type {:?}.",
                value::type_of(&val_to_call)
            ))),
        }
    }

    fn call_native_func(
        &mut self,
        native_func: value::NativeFunction,
        arg_count: u8,
    ) -> Result<(), VmError> {
        if arg_count != native_func.arity {
            return Err(VmError::Runtime(format!(
                "Native function {} expected {} arguments but found {}.",
                native_func.name, native_func.arity, arg_count
            )));
        }

        let mut args = Vec::new();
        for _ in 0..arg_count {
            args.push(self.pop_stack()) // pop args
        }
        args.reverse();
        let args = args;
        self.pop_stack(); // native function value

        let res = (native_func.func)(self, &args);

        match res {
            Ok(result) => {
                self.stack.push(result);
                Ok(())
            }
            Err(err) => Err(VmError::Runtime(format!(
                "When calling {}: {}.",
                native_func.name, err
            ))),
        }
    }

    fn create_instance(&mut self, class_id: gc::HeapId) {
        self.pop_stack(); // class object
        let instance_id = self.heap.manage_instance(value::Instance {
            class_id,
            fields: HashMap::new(),
        });
        self.stack.push(value::Value::Instance(instance_id));
    }

    fn call_bound_method(&mut self, method_id: gc::HeapId, arg_count: u8) -> Result<(), VmError> {
        let bound_method = self.get_bound_method(method_id).clone();
        let closure_id = bound_method.closure_id;
        let arg_count_usize: usize = arg_count.into();
        let stack_len = self.stack.len();
        self.stack[stack_len - arg_count_usize - 1] =
            value::Value::Instance(bound_method.instance_id);
        self.prepare_call(closure_id, arg_count)
    }

    /*
    Set up a few call frame so that on the next interpreter step we'll start executing code inside the function.
     */
    fn prepare_call(&mut self, closure_handle: gc::HeapId, arg_count: u8) -> Result<(), VmError> {
        let closure = self.get_closure(closure_handle).clone();
        let func = &closure.function;
        if arg_count != func.arity {
            return Err(VmError::Runtime(format!(
                "Expected {} arguments but found {}.",
                func.arity, arg_count
            )));
        }

        self.frames.push(CallFrame::default());
        let frame = self.frames.last_mut().unwrap();
        frame.closure = closure;
        frame.slots_offset = self.stack.len() - usize::from(arg_count);
        Ok(())
    }

    fn pop_stack_n_times(&mut self, num_to_pop: usize) {
        for _ in 0..num_to_pop {
            self.pop_stack();
        }
    }

    fn is_falsey(&self, val: &value::Value) -> bool {
        match val {
            value::Value::Nil => true,
            value::Value::Bool(b) => !*b,
            value::Value::Number(f) => *f == 0.0,
            value::Value::Function(_) => false,
            value::Value::NativeFunction(_) => false,
            value::Value::Class(_) => false,
            value::Value::Instance(_) => false,
            value::Value::BoundMethod(_) => false,
            value::Value::String(id) => self.get_str(*id).is_empty(),
            value::Value::List(id) => self.get_list_elements(*id).is_empty(),
        }
    }

    fn print_val(&mut self, val: &value::Value) {
        let output = self.format_val(val);
        println!("{}", output);
        self.output.push(output);
    }

    fn values_equal(&self, val1: &value::Value, val2: &value::Value) -> bool {
        match (val1, val2) {
            (value::Value::Number(n1), value::Value::Number(n2)) => (n1 - n2).abs() < f64::EPSILON,
            (value::Value::Bool(b1), value::Value::Bool(b2)) => b1 == b2,
            (value::Value::String(s1), value::Value::String(s2)) => {
                self.get_str(*s1) == self.get_str(*s2)
            }
            (value::Value::Nil, value::Value::Nil) => true,
            (_, _) => false,
        }
    }

    fn numeric_binop(
        &mut self,
        binop: Binop,
        lineno: firnas_bytecode::Lineno,
    ) -> Result<(), VmError> {
        let val1 = self.peek_by(0).clone();
        let val2 = self.peek_by(1).clone();

        match (&val1, &val2) {
            (value::Value::Number(n1), value::Value::Number(n2)) => {
                self.pop_stack();
                self.pop_stack();
                self.stack
                    .push(value::Value::Number(VirtualMachine::apply_numeric_binop(
                        *n2, *n1, binop, // note the order!
                    )));
                Ok(())
            }
            _ => Err(VmError::Runtime(format!(
                "Expected numbers in {:?} expression. Found {:?} and {:?} (line={})",
                binop,
                value::type_of(&val1),
                value::type_of(&val2),
                lineno.value
            ))),
        }
    }

    fn apply_numeric_binop(left: f64, right: f64, binop: Binop) -> f64 {
        match binop {
            Binop::Add => left + right,
            Binop::Sub => left - right,
            Binop::Mul => left * right,
            Binop::Div => left / right,
        }
    }

    fn setattr(
        &mut self,
        maybe_instance: value::Value,
        val: value::Value,
        attr_id: gc::HeapId,
    ) -> Result<(), VmError> {
        let attr_name = self.get_str(attr_id).clone();
        match maybe_instance {
            value::Value::Instance(instance_id) => {
                let instance = self.heap.get_instance_mut(instance_id);
                instance.fields.insert(attr_name, val);
                Ok(())
            }
            _ => Err(VmError::Runtime(format!(
                "can't set attribute on value of type {:?}. Need class instance. val = {:?}",
                value::type_of(&maybe_instance),
                self.format_val(&maybe_instance)
            ))),
        }
    }

    fn getattr(
        &self,
        maybe_instance: value::Value,
        attr_id: gc::HeapId,
    ) -> Result<Option<value::Value>, VmError> {
        let attr_name = self.get_str(attr_id).clone();
        match maybe_instance {
            value::Value::Instance(instance_id) => {
                let instance = self.heap.get_instance(instance_id);
                match instance.fields.get(&attr_name) {
                    Some(val) => Ok(Some(val.clone())),
                    None => Ok(None),
                }
            }
            _ => Err(VmError::Runtime(format!(
                "can't get attribute {}  on value of type {:?}. Need class instance.",
                attr_name,
                value::type_of(&maybe_instance)
            ))),
        }
    }

    fn bind_method(
        &mut self,
        instance_id: gc::HeapId,
        class: value::Class,
        attr_id: gc::HeapId,
    ) -> Result<bool, VmError> {
        let attr_name = self.get_str(attr_id).clone();
        if let Some(closure_id) = class.methods.get(&attr_name) {
            self.pop_stack();
            self.stack
                .push(value::Value::BoundMethod(self.heap.manage_bound_method(
                    value::BoundMethod {
                        instance_id,
                        closure_id: *closure_id,
                    },
                )));
            Ok(true)
        } else {
            Ok(false)
        }
    }

    pub fn pop_stack(&mut self) -> value::Value {
        match self.stack.pop() {
            Some(val) => val,
            None => panic!("attempted to pop empty stack!"),
        }
    }

    fn peek(&self) -> &value::Value {
        self.peek_by(0)
    }

    fn peek_by(&self, n: usize) -> &value::Value {
        &self.stack[self.stack.len() - n - 1]
    }

    pub fn next_line(&self) -> usize {
        self.next_op().1.value
    }

    pub fn next_op(&self) -> (firnas_bytecode::Op, firnas_bytecode::Lineno) {
        self.frame().next_op()
    }

    fn next_op_and_advance(&mut self) -> (firnas_bytecode::Op, firnas_bytecode::Lineno) {
        self.frame_mut().next_op_and_advance()
    }

    fn read_constant(&mut self, idx: usize) -> value::Value {
        let constant = self.frame().read_constant(idx);
        match constant {
            firnas_bytecode::Constant::Number(num) => value::Value::Number(num),
            firnas_bytecode::Constant::String(s) => value::Value::String(self.heap.manage_str(s)),
            firnas_bytecode::Constant::Function(f) => {
                value::Value::Function(self.heap.manage_closure(value::Closure {
                    function: f.function,
                    upvalues: Vec::new(),
                }))
            }
        }
    }

    fn extract_number(val: &value::Value) -> Option<f64> {
        match val {
            value::Value::Number(f) => Some(*f),
            _ => None,
        }
    }

    fn extract_bool(val: &value::Value) -> Option<bool> {
        match val {
            value::Value::Bool(b) => Some(*b),
            _ => None,
        }
    }

    fn get_str(&self, str_handle: gc::HeapId) -> &String {
        self.heap.get_str(str_handle)
    }

    fn get_closure(&self, closure_handle: gc::HeapId) -> &value::Closure {
        self.heap.get_closure(closure_handle)
    }

    fn get_class(&self, class_handle: gc::HeapId) -> &value::Class {
        self.heap.get_class(class_handle)
    }

    fn get_class_mut(&mut self, class_handle: gc::HeapId) -> &mut value::Class {
        self.heap.get_class_mut(class_handle)
    }

    fn get_bound_method(&self, method_handle: gc::HeapId) -> &value::BoundMethod {
        self.heap.get_bound_method(method_handle)
    }

    fn get_list_elements(&self, list_handle: gc::HeapId) -> &Vec<value::Value> {
        self.heap.get_list_elements(list_handle)
    }

    fn get_list_elements_mut(&mut self, list_handle: gc::HeapId) -> &mut Vec<value::Value> {
        self.heap.get_list_elements_mut(list_handle)
    }

    fn get_instance(&self, instance_handle: gc::HeapId) -> &value::Instance {
        self.heap.get_instance(instance_handle)
    }

    fn collect_garbage(&mut self) {
        self.heap.unmark();
        self.mark_roots();
        self.trace_references();

        self.heap.sweep();
    }

    fn trace_references(&mut self) {
        loop {
            let maybe_val = self.gray_stack.pop();
            match maybe_val {
                Some(val) => self.blacken_object(val),
                None => break,
            }
        }
    }

    fn blacken_object(&mut self, val: gc::HeapId) {
        let children_to_walk = self.heap.children(val);
        for child_val in children_to_walk {
            if !self.heap.is_marked(child_val) {
                self.heap.mark(child_val);
                self.blacken_object(child_val);
            }
        }
    }

    fn mark_roots(&mut self) {
        let stack_vals_to_mark: Vec<gc::HeapId> =
            self.stack.iter().filter_map(gc::Heap::extract_id).collect();

        let frame_closure_children: Vec<gc::HeapId> = self
            .frames
            .iter()
            .flat_map(|frame| self.heap.closure_children(&frame.closure))
            .collect();

        let globals_to_mark: Vec<gc::HeapId> = self
            .globals
            .values()
            .flat_map(gc::Heap::extract_id)
            .collect();

        for val in stack_vals_to_mark
            .iter()
            .chain(frame_closure_children.iter())
            .chain(globals_to_mark.iter())
        {
            self.mark_value(*val);
        }
    }

    fn mark_value(&mut self, handle: gc::HeapId) {
        let is_marked = self.heap.is_marked(handle);
        if !is_marked {
            self.heap.mark(handle);
        }
        self.gray_stack.push(handle)
    }
}
