use super::parse::TokenOp;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OpCode {
    Load,
    Push,
    Pop,
    Aloc,
    Setm,
    Add,
    Sub,
    Div,
    Mod,
    Mul,
    Jmp,
    Jeq,
    Jne,
    Eq,
    Neq,
    Gt,
    Geq,
    Lt,
    Leq,
    Inc,
    Dec,
    // Call
    // Ret,
    Prts,
    Prti,
    Hlt,
    Nop,
    Ige,
}

impl From<&TokenOp> for OpCode {
    fn from(value: &TokenOp) -> Self {
        match value {
            TokenOp::Load(..) => Self::Load,
            TokenOp::Push(..) => Self::Push,
            TokenOp::Pop(..) => Self::Pop,
            TokenOp::Aloc(..) => Self::Aloc,
            TokenOp::Setm(..) => Self::Setm,
            TokenOp::Add(..) => Self::Add,
            TokenOp::Sub(..) => Self::Sub,
            TokenOp::Div(..) => Self::Div,
            TokenOp::Mod(..) => Self::Mod,
            TokenOp::Mul(..) => Self::Mul,
            TokenOp::Jmp(..) => Self::Jmp,
            TokenOp::Jeq(..) => Self::Jeq,
            TokenOp::Jne(..) => Self::Jne,
            TokenOp::Neq(..) => Self::Neq,
            TokenOp::Inc(..) => Self::Inc,
            TokenOp::Dec(..) => Self::Dec,
            TokenOp::Eq(..) => Self::Eq,
            TokenOp::Gt(..) => Self::Gt,
            TokenOp::Geq(..) => Self::Geq,
            TokenOp::Lt(..) => Self::Lt,
            TokenOp::Leq(..) => Self::Leq,
            TokenOp::Prts(..) => Self::Prts,
            TokenOp::Prti(..) => Self::Prti,
            TokenOp::Hlt => Self::Hlt,
            TokenOp::Nop => Self::Nop,
        }
    }
}
impl From<u8> for OpCode {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::Load,
            1 => Self::Push,
            2 => Self::Pop,
            3 => Self::Aloc,
            4 => Self::Setm,
            5 => Self::Add,
            6 => Self::Sub,
            7 => Self::Div,
            8 => Self::Mod,
            9 => Self::Mul,
            10 => Self::Jmp,
            11 => Self::Jeq,
            12 => Self::Jne,
            13 => Self::Eq,
            14 => Self::Neq,
            15 => Self::Gt,
            16 => Self::Geq,
            17 => Self::Lt,
            18 => Self::Leq,
            19 => Self::Inc,
            20 => Self::Dec,
            21 => Self::Prts,
            22 => Self::Prti,
            23 => Self::Hlt,
            24 => Self::Nop,
            _ => Self::Ige,
        }
    }
}
