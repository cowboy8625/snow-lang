use super::*;
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq, Eq)]
#[repr(u8)]
pub enum TokenOp {
    Load(u8, u8, u8),
    Add(u8, u8, u8),
    Sub(u8, u8, u8),
    Div(u8, u8, u8),
    Mod(u8, u8, u8),
    Mul(u8, u8, u8),
    Jmp(Location),
    Jeq(Location),
    Jne(Location),
    Prts(Location),
    Prti(u8),
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
    Hlt,
    Nop,
}

impl TokenOp {
    pub fn into_bytes(self, labels: &SymbolTable) -> Result<[u8; 4], Error> {
        let code = OpCode::from(&self) as u8;
        match self {
            Self::Load(a, b, c) => Ok([code, a, b, c]),
            Self::Add(a, b, c)
            | Self::Sub(a, b, c)
            | Self::Div(a, b, c)
            | Self::Mod(a, b, c)
            | Self::Mul(a, b, c) => Ok([code, a, b, c]),
            Self::Jmp(Location(ref name))
            | Self::Jeq(Location(ref name))
            | Self::Jne(Location(ref name))
            | Self::Prts(Location(ref name)) => {
                let Some(value) = labels.get(name) else {
                    return Err(Error::LabelNotDefined(name.into()));
                };
                let [_, b3, b2, b1] = value.to_be_bytes();
                Ok([code, b3, b2, b1])
            }
            Self::Setm(a, b)
            | Self::Eq(a, b)
            | Self::Neq(a, b)
            | Self::Geq(a, b)
            | Self::Gt(a, b) => Ok([code, a, b, 0]),
            Self::Leq(a, b) | Self::Lt(a, b) => Ok([code, a, b, 0]),
            Self::Prti(a)
            | Self::Inc(a)
            | Self::Push(a)
            | Self::Pop(a)
            | Self::Aloc(a)
            | Self::Dec(a) => Ok([code, a, 0, 0]),
            Self::Hlt | Self::Nop => Ok([code, 0, 0, 0]),
        }
    }
}

impl FromStr for TokenOp {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (head, tail) = s.split_once(' ').unwrap_or((s, ""));
        match head {
            "load" => parse_1reg_u16(tail, TokenOp::Load, head),
            "push" => parse_1reg(tail, TokenOp::Push, head),
            "pop" => parse_1reg(tail, TokenOp::Pop, head),
            "inc" => parse_1reg(tail, TokenOp::Inc, head),
            "dec" => parse_1reg(tail, TokenOp::Dec, head),
            "prti" => parse_1reg(tail, TokenOp::Prti, head),
            "aloc" => parse_1reg(tail, TokenOp::Aloc, head),
            "setm" => parse_2reg(tail, TokenOp::Setm, head),
            "eq" => parse_2reg(tail, TokenOp::Eq, head),
            "neq" => parse_2reg(tail, TokenOp::Neq, head),
            "gt" => parse_2reg(tail, TokenOp::Gt, head),
            "geq" => parse_2reg(tail, TokenOp::Geq, head),
            "lt" => parse_2reg(tail, TokenOp::Lt, head),
            "leq" => parse_2reg(tail, TokenOp::Leq, head),
            "add" => parse_3reg(tail, TokenOp::Add, head),
            "sub" => parse_3reg(tail, TokenOp::Sub, head),
            "div" => parse_3reg(tail, TokenOp::Div, head),
            "mod" => parse_3reg(tail, TokenOp::Mod, head),
            "mul" => parse_3reg(tail, TokenOp::Mul, head),
            "prts" => parse_loc(tail, TokenOp::Prts, head),
            "jmp" => parse_loc(tail, TokenOp::Jmp, head),
            "jeq" => parse_loc(tail, TokenOp::Jeq, head),
            "jne" => parse_loc(tail, TokenOp::Jne, head),
            "hlt" => Ok(TokenOp::Hlt),
            "nop" => Ok(TokenOp::Nop),
            _ => Err(Error::Unexpected("not a opcode".into(), head.into())),
        }
    }
}

fn parse_3reg<F>(input: &str, t: F, msg: &str) -> Result<TokenOp, Error>
where
    F: FnOnce(u8, u8, u8) -> TokenOp,
{
    let &[reg1, reg2, reg3] = input
        .split(' ')
        .filter(|t|!t.is_empty())
        .collect::<Vec<_>>()
        .as_slice() else {
        return Err(Error::Unexpected(format!("{msg}reg reg reg"), input.into()));
    };
    let Reg(r1) = reg1.parse::<Reg>()?;
    let Reg(r2) = reg2.parse::<Reg>()?;
    let Reg(r3) = reg3.parse::<Reg>()?;
    Ok(t(r1, r2, r3))
}

fn parse_2reg<F>(input: &str, t: F, msg: &str) -> Result<TokenOp, Error>
where
    F: FnOnce(u8, u8) -> TokenOp,
{
    let &[reg1, reg2] = input
        .split(' ')
        .filter(|t|!t.is_empty())
        .collect::<Vec<_>>()
        .as_slice() else {
        return Err(Error::Unexpected(format!("{msg}reg reg reg"), input.into()));
    };
    let Reg(r1) = reg1.parse::<Reg>()?;
    let Reg(r2) = reg2.parse::<Reg>()?;
    Ok(t(r1, r2))
}

fn parse_1reg<F>(input: &str, t: F, msg: &str) -> Result<TokenOp, Error>
where
    F: FnOnce(u8) -> TokenOp,
{
    if input.is_empty() {
        return Err(Error::Unexpected(format!("{msg}reg reg reg"), input.into()));
    }
    let Reg(r1) = input.trim().parse::<Reg>()?;
    Ok(t(r1))
}
fn parse_loc<F>(input: &str, t: F, msg: &str) -> Result<TokenOp, Error>
where
    F: FnOnce(Location) -> TokenOp,
{
    if input.is_empty() {
        return Err(Error::Unexpected(format!("{msg}reg reg reg"), input.into()));
    }
    input.trim().parse::<Location>().and_then(|loc| Ok(t(loc)))
}

fn parse_1reg_u16<F>(input: &str, t: F, msg: &str) -> Result<TokenOp, Error>
where
    F: FnOnce(u8, u8, u8) -> TokenOp,
{
    let Some((reg, imm)) = input.trim_start().split_once(' ') else {
        return Err(Error::Unexpected("load reg imm".into(), input.into()));
    };
    let Reg(r) = reg.trim().parse::<Reg>()?;
    let i = imm
        .trim()
        .parse::<u32>()
        .map_err(|_| Error::Unexpected(format!("{msg} <num>"), imm.into()))?;
    let [_, _, b2, b1] = i.to_be_bytes();
    Ok(t(r, b2, b1))
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn from_str_opcode_load() {
        let src = "load    %0      123     ";
        let left = src.parse::<TokenOp>();
        let right = Ok(TokenOp::Load(0, 0, 123));
        assert_eq!(left, right);
    }

    #[test]
    fn from_str_opcode_push() {
        let src = "push      %0     ";
        let left = src.parse::<TokenOp>();
        let right = Ok(TokenOp::Push(0));
        assert_eq!(left, right);
    }

    #[test]
    fn from_str_opcode_pop() {
        let src = "pop     %0    ";
        let left = src.parse::<TokenOp>();
        let right = Ok(TokenOp::Pop(0));
        assert_eq!(left, right);
    }

    #[test]
    fn from_str_opcode_aloc() {
        let src = "aloc     %0   ";
        let left = src.parse::<TokenOp>();
        let right = Ok(TokenOp::Aloc(0));
        assert_eq!(left, right);
    }

    #[test]
    fn from_str_opcode_setm() {
        let src = "setm        %0           %21    ";
        let left = src.parse::<TokenOp>();
        let right = Ok(TokenOp::Setm(0, 21));
        assert_eq!(left, right);
    }

    #[test]
    fn from_str_opcode_add() {
        let src = "add %0 %1 %2";
        let left = src.parse::<TokenOp>();
        let right = Ok(TokenOp::Add(0, 1, 2));
        assert_eq!(left, right);
    }

    #[test]
    fn from_str_opcode_sub() {
        let src = "sub %0 %1 %2";
        let left = src.parse::<TokenOp>();
        let right = Ok(TokenOp::Sub(0, 1, 2));
        assert_eq!(left, right);
    }

    #[test]
    fn from_str_opcode_div() {
        let src = "div %0 %1 %2";
        let left = src.parse::<TokenOp>();
        let right = Ok(TokenOp::Div(0, 1, 2));
        assert_eq!(left, right);
    }

    #[test]
    fn from_str_opcode_mul() {
        let src = "mul %0 %1 %2";
        let left = src.parse::<TokenOp>();
        let right = Ok(TokenOp::Mul(0, 1, 2));
        assert_eq!(left, right);
    }

    #[test]
    fn from_str_opcode_jmp() {
        let src = "jmp __start__";
        let left = src.parse::<TokenOp>();
        let right = Ok(TokenOp::Jmp(Location("__start__".into())));
        assert_eq!(left, right);
        let src = "jmp can_you_parse_this";
        let left = src.parse::<TokenOp>();
        let right = Ok(TokenOp::Jmp(Location("can_you_parse_this".into())));
        assert_eq!(left, right);
        assert!("jmp 123".parse::<TokenOp>().is_err());
    }

    #[test]
    fn from_str_opcode_jeq() {
        let src = "jeq start";
        let left = src.parse::<TokenOp>();
        let right = Ok(TokenOp::Jeq(Location("start".into())));
        assert_eq!(left, right);
    }

    #[test]
    fn from_str_opcode_jne() {
        let src = "jne start";
        let left = src.parse::<TokenOp>();
        let right = Ok(TokenOp::Jne(Location("start".into())));
        assert_eq!(left, right);
    }

    #[test]
    fn from_str_opcode_eq() {
        let src = "eq %10 %11";
        let left = src.parse::<TokenOp>();
        let right = Ok(TokenOp::Eq(10, 11));
        assert_eq!(left, right);
    }

    #[test]
    fn from_str_opcode_neq() {
        let src = "neq %10 %11";
        let left = src.parse::<TokenOp>();
        let right = Ok(TokenOp::Neq(10, 11));
        assert_eq!(left, right);
    }

    #[test]
    fn from_str_opcode_gt() {
        let src = "gt %10 %0";
        let left = src.parse::<TokenOp>();
        let right = Ok(TokenOp::Gt(10, 0));
        assert_eq!(left, right);
    }

    #[test]
    fn from_str_opcode_geq() {
        let src = "geq %10 %23";
        let left = src.parse::<TokenOp>();
        let right = Ok(TokenOp::Geq(10, 23));
        assert_eq!(left, right);
    }

    #[test]
    fn from_str_opcode_inc() {
        let src = "inc %10";
        let left = src.parse::<TokenOp>();
        let right = Ok(TokenOp::Inc(10));
        assert_eq!(left, right);
    }

    #[test]
    fn from_str_opcode_dec() {
        let src = "dec %20";
        let left = src.parse::<TokenOp>();
        let right = Ok(TokenOp::Dec(20));
        assert_eq!(left, right);
    }

    #[test]
    fn from_str_opcode_hlt() {
        let src = "hlt";
        let left = src.parse::<TokenOp>();
        let right = Ok(TokenOp::Hlt);
        assert_eq!(left, right);
    }
}
