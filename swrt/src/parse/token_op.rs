use super::*;

#[derive(Debug, Clone, PartialEq, Eq)]
#[repr(u8)]
pub enum TokenOp {
    Load(u8, u8, u8),
    Add(u8, u8, u8),
    Sub(u8, u8, u8),
    Div(u8, u8, u8),
    Mod(u8, u8, u8),
    Mul(u8, u8, u8),
    Call(Label),
    Jmp(Label),
    Jeq(Label),
    Jne(Label),
    Prts(Label),
    Prti(u8),
    LoadM(u8, u8),
    Setm(u8, u8),
    Eq(u8, u8),
    Neq(u8, u8),
    Gt(u8, u8),
    Geq(u8, u8),
    Lt(u8, u8),
    Leq(u8, u8),
    Aloc(u8),
    Push(u8),
    Pop(u8),
    Inc(u8),
    Dec(u8),
    Ret,
    Hlt,
    Nop,
}

impl TokenOp {
    pub fn as_bytes(&self, labels: &SymbolTable) -> Result<[u8; 4], Error> {
        let code = OpCode::from(self) as u8;
        match self {
            Self::Load(a, b, c) => Ok([code, *a, *b, *c]),
            Self::Add(a, b, c)
            | Self::Sub(a, b, c)
            | Self::Div(a, b, c)
            | Self::Mod(a, b, c)
            | Self::Mul(a, b, c) => Ok([code, *a, *b, *c]),
            Self::Jmp(Label { ref name, span, .. })
            | Self::Call(Label { ref name, span, .. })
            | Self::Jeq(Label { ref name, span, .. })
            | Self::Jne(Label { ref name, span, .. })
            | Self::Prts(Label { ref name, span, .. }) => {
                let Some(value) = labels.get(name) else {
                    return Err(error("E0020", &format!("undefined '{name}'"), *span));
                };
                let [_, b3, b2, b1] = value.to_be_bytes();
                Ok([code, b3, b2, b1])
            }
            Self::Setm(a, b)
            | Self::LoadM(a, b)
            | Self::Eq(a, b)
            | Self::Neq(a, b)
            | Self::Geq(a, b)
            | Self::Gt(a, b) => Ok([code, *a, *b, 0]),
            Self::Leq(a, b) | Self::Lt(a, b) => Ok([code, *a, *b, 0]),
            Self::Prti(a)
            | Self::Inc(a)
            | Self::Push(a)
            | Self::Pop(a)
            | Self::Aloc(a)
            | Self::Dec(a) => Ok([code, *a, 0, 0]),
            Self::Ret | Self::Hlt | Self::Nop => Ok([code, 0, 0, 0]),
        }
    }
}
