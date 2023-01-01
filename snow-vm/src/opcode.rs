use super::parse::TokenOp;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OpCode {
    Load,
    Add,
    Sub,
    Div,
    Mul,
    Jmp,
    Jeq,
    Jne,
    Eq,
    Inc,
    Dec,
    // Push,
    // Pop,
    // Call
    // Ret,
    Prts,
    Hlt,
    Nop,
    Ige,
}

impl From<&TokenOp> for OpCode {
    fn from(value: &TokenOp) -> Self {
        match value {
            TokenOp::Load(..) => Self::Load,
            TokenOp::Add(..) => Self::Add,
            TokenOp::Sub(..) => Self::Sub,
            TokenOp::Div(..) => Self::Div,
            TokenOp::Mul(..) => Self::Mul,
            TokenOp::Jmp(..) => Self::Jmp,
            TokenOp::Jeq(..) => Self::Jeq,
            TokenOp::Jne(..) => Self::Jne,
            TokenOp::Inc(..) => Self::Inc,
            TokenOp::Dec(..) => Self::Dec,
            TokenOp::Eq(..) => Self::Eq,
            TokenOp::Prts(..) => Self::Prts,
            TokenOp::Hlt => Self::Hlt,
            TokenOp::Nop => Self::Nop,
        }
    }
}
impl From<u8> for OpCode {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::Load,
            1 => Self::Add,
            2 => Self::Sub,
            3 => Self::Div,
            4 => Self::Mul,
            5 => Self::Jmp,
            6 => Self::Jeq,
            7 => Self::Jne,
            8 => Self::Eq,
            9 => Self::Inc,
            10 => Self::Dec,
            11 => Self::Prts,
            12 => Self::Hlt,
            13 => Self::Nop,
            _ => Self::Ige,
        }
    }
}
