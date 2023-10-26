#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum OpCode {
    Return,
    Constant,
    ConstantLong,
    Negate,
    Add,
    Subtract,
    Multiply,
    Divide,
}

#[cfg(feature = "dbg")]
impl OpCode {
    pub fn dbg_str(&self) -> String {
        match self {
            OpCode::Return => "OP_RETURN",
            OpCode::Constant => "OP_CONSTANT",
            OpCode::ConstantLong => "OP_CONSTANT_LONG",
            OpCode::Negate => "OP_NEGATE",
            OpCode::Add => "OP_ADD",
            OpCode::Subtract => "OP_SUBTRACT",
            OpCode::Multiply => "OP_MULTIPLY",
            OpCode::Divide => "OP_DIVIDE",
        }
        .to_string()
    }
}

impl From<OpCode> for u8 {
    fn from(val: OpCode) -> Self {
        match val {
            OpCode::Return => 0x00,
            OpCode::Constant => 0x01,
            OpCode::ConstantLong => 0x02,
            OpCode::Negate => 0x03,
            OpCode::Add => 0x04,
            OpCode::Subtract => 0x05,
            OpCode::Multiply => 0x06,
            OpCode::Divide => 0x07,
        }
    }
}

impl From<u8> for OpCode {
    fn from(val: u8) -> Self {
        match val {
            0x00 => OpCode::Return,
            0x01 => OpCode::Constant,
            0x02 => OpCode::ConstantLong,
            0x03 => OpCode::Negate,
            0x04 => OpCode::Add,
            0x05 => OpCode::Subtract,
            0x06 => OpCode::Multiply,
            0x07 => OpCode::Divide,
            _ => panic!("Undefined state"),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::op_code::OpCode;

    impl OpCode {
        pub fn iterator() -> impl Iterator<Item = Self> {
            use crate::op_code::OpCode as OC;

            [
                OC::Return,
                OC::Constant,
                OC::ConstantLong,
                OC::Negate,
                OC::Add,
                OC::Subtract,
                OC::Multiply,
                OC::Divide,
            ]
            .iter()
            .copied()
        }
    }

    #[test]
    fn it_should_be_inverse() {
        for op_code in OpCode::iterator() {
            let byte: u8 = op_code.into();
            assert_eq!(op_code, byte.into());
        }
    }
}
