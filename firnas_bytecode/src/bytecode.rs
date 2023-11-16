use std::f64;
use std::fmt;

#[derive(Default, Copy, Clone, Debug)]
pub struct Lineno {
    pub value: usize,
}

impl Lineno {
    pub fn new(value: usize) -> Self {
        Self { value }
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum UpvalueLoc {
    Upvalue(/*upvalue idx*/ usize),
    Local(/*stack idx*/ usize),
}

#[derive(Debug, Clone)]
pub enum Op {
    Return,
    Constant(usize),
    Closure(usize, Vec<UpvalueLoc>),
    Nil,
    True,
    False,
    Negate,
    Add,
    Subtract,
    Multiply,
    Divide,
    Not,
    Equal,
    Greater,
    Less,
    Pop,
    DefineGlobal(usize),
    GetGlobal(usize),
    SetGlobal(usize),
    GetLocal(usize),
    SetLocal(usize),
    GetUpval(usize),
    SetUpval(usize),
    JumpIfFalse(usize),
    Jump(usize),
    Loop(usize),
    Call(u8),
    CloseUpvalue,
    Class(usize),
    SetProperty(usize),
    GetProperty(usize),
    Method(usize),
    Invoke(/*method_name*/ String, /*arg count*/ u8),
    Inherit,
    GetSuper(usize),
    SuperInvoke(/*method_name*/ String, /*arg count*/ u8),
    BuildList(usize),
    Subscr,
    SetItem,
}

#[derive(Default, Clone, Debug)]
pub struct Function {
    pub arity: u8,
    pub chunk: Chunk,
    pub name: String,
}

#[derive(Debug, Clone, Default)]
pub struct Closure {
    pub function: Function,
    pub upvalues: Vec<UpvalueLoc>,
}

#[derive(Debug, Clone)]
pub enum Constant {
    Number(f64),
    String(String),
    Function(Closure),
}

impl fmt::Display for Constant {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Constant::Number(n) => write!(f, "{}", n),
            Constant::String(s) => write!(f, "\"{}\"", s),
            Constant::Function(Closure {
                function:
                    Function {
                        arity: _,
                        chunk: _,
                        name,
                    },
                upvalues: _,
            }) => write!(f, "<fn {}>", name),
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct Chunk {
    pub code: Vec<(Op, Lineno)>,
    pub constants: Vec<Constant>,
}

impl Chunk {
    pub fn add_constant_number(&mut self, c: f64) -> usize {
        if let Some(id) = self.find_number(c) {
            id
        } else {
            self.add_constant(Constant::Number(c))
        }
    }

    pub fn add_constant_string(&mut self, s: String) -> usize {
        if let Some(id) = self.find_string(&s) {
            id
        } else {
            self.add_constant(Constant::String(s))
        }
    }

    pub fn add_constant(&mut self, val: Constant) -> usize {
        let const_idx = self.constants.len();
        self.constants.push(val);
        const_idx
    }

    fn find_string(&self, s: &str) -> Option<usize> {
        self.constants.iter().position(|c| {
            if let Constant::String(s2) = c {
                s == s2
            } else {
                false
            }
        })
    }

    fn find_number(&self, num: f64) -> Option<usize> {
        self.constants.iter().position(|c| {
            if let Constant::Number(num2) = c {
                (num - num2).abs() < f64::EPSILON
            } else {
                false
            }
        })
    }
}

pub fn disassemble_code(chunk: &Chunk) -> Vec<String> {
    let mut lines: Vec<String> = Vec::new();

    for (idx, (op, lineno)) in chunk.code.iter().enumerate() {
        let formatted_op = match op {
            Op::Return => "OP_RETURN".to_string(),
            Op::Constant(const_idx) => format!(
                "OP_CONSTANT {} (idx={})",
                chunk.constants[*const_idx], *const_idx
            ),
            Op::Nil => "OP_NIL".to_string(),
            Op::True => "OP_TRUE".to_string(),
            Op::False => "OP_FALSE".to_string(),
            Op::Negate => "OP_NEGATE".to_string(),
            Op::Add => "OP_ADD".to_string(),
            Op::Subtract => "OP_SUBTRACT".to_string(),
            Op::Multiply => "OP_MULTIPLY".to_string(),
            Op::Divide => "OP_DIVIDE".to_string(),
            Op::Not => "OP_NOT".to_string(),
            Op::Equal => "OP_NOT".to_string(),
            Op::Greater => "OP_GREATER".to_string(),
            Op::Less => "OP_LESS".to_string(),
            Op::Pop => "OP_POP".to_string(),
            Op::DefineGlobal(global_idx) => format!(
                "OP_DEFINE_GLOBAL {:?} (idx={})",
                chunk.constants[*global_idx], *global_idx
            ),
            Op::GetGlobal(global_idx) => format!(
                "OP_GET_GLOBAL {:?} (idx={})",
                chunk.constants[*global_idx], *global_idx
            ),
            Op::SetGlobal(global_idx) => format!(
                "OP_SET_GLOBAL {:?} (idx={})",
                chunk.constants[*global_idx], *global_idx
            ),
            Op::GetLocal(idx) => format!("OP_GET_LOCAL idx={}", *idx),
            Op::SetLocal(idx) => format!("OP_SET_LOCAL idx={}", *idx),
            Op::GetUpval(idx) => format!("OP_GET_UPVAL idx={}", *idx),
            Op::SetUpval(idx) => format!("OP_SET_UPVAL idx={}", *idx),
            Op::JumpIfFalse(loc) => format!("OP_JUMP_IF_FALSE {}", *loc),
            Op::Jump(offset) => format!("OP_JUMP {}", *offset),
            Op::Loop(offset) => format!("OP_LOOP {}", *offset),
            Op::Call(arg_count) => format!("OP_CALL {}", *arg_count),
            Op::Closure(idx, _) => format!("OP_CLOSURE {}", chunk.constants[*idx],),
            Op::CloseUpvalue => "OP_CLOSE_UPVALUE".to_string(),
            Op::Class(idx) => format!("OP_CLASS {}", idx),
            Op::SetProperty(idx) => format!("OP_SET_PROPERTY {}", idx),
            Op::GetProperty(idx) => format!("OP_GET_PROPERTY {}", idx),
            Op::Method(idx) => format!("OP_METHOD {}", idx),
            Op::Invoke(method_name, arg_count) => {
                format!("OP_INVOKE {} nargs={}", method_name, arg_count)
            }
            Op::Inherit => "OP_INHERIT".to_string(),
            Op::GetSuper(idx) => format!("OP_GET_SUPER {}", idx),
            Op::SuperInvoke(method_name, arg_count) => {
                format!("OP_SUPER_INOKE {} nargs={}", method_name, arg_count)
            }
            Op::BuildList(size) => format!("OP_BUILD_LIST {}", size),
            Op::Subscr => "OP_SUBSCR".to_string(),
            Op::SetItem => "OP_SETITEM".to_string(),
        };

        lines.push(format!(
            "{0: <04}   {1: <50} line {2: <50}",
            idx, formatted_op, lineno.value
        ));
    }
    lines
}

pub fn disassemble_chunk(chunk: &Chunk, name: &str) -> String {
    let mut lines: Vec<String> = Vec::new();

    if !name.is_empty() {
        lines.push(format!("============ {} ============", name));
    }

    lines.push("------------ constants -----------".to_string());
    for (idx, constant) in chunk.constants.iter().enumerate() {
        lines.push(format!("{:<4} {}", idx, constant));
    }

    lines.push("\n------------ code -----------------".to_string());

    for code_line in disassemble_code(chunk) {
        lines.push(code_line)
    }

    lines.join("\n")
}
