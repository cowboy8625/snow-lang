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
            "aloc" => parse_1reg(tail, TokenOp::Aloc, head),
            "setm" => parse_2reg(tail, TokenOp::Setm, head),
            "add" => parse_3reg(tail, TokenOp::Add, head),
            "sub" => parse_3reg(tail, TokenOp::Sub, head),
            "div" => parse_3reg(tail, TokenOp::Div, head),
            "mod" => parse_3reg(tail, TokenOp::Mod, head),
            "mul" => parse_3reg(tail, TokenOp::Mul, head),
            "jmp" => parse_loc(tail, TokenOp::Jmp, head),
            "jeq" => parse_loc(tail, TokenOp::Jeq, head),
            "jne" => parse_loc(tail, TokenOp::Jne, head),
            "eq" => parse_2reg(tail, TokenOp::Eq, head),
            "neq" => parse_2reg(tail, TokenOp::Neq, head),
            "gt" => parse_2reg(tail, TokenOp::Gt, head),
            "geq" => parse_2reg(tail, TokenOp::Geq, head),
            "lt" => parse_2reg(tail, TokenOp::Lt, head),
            "leq" => parse_2reg(tail, TokenOp::Leq, head),
            "inc" => parse_1reg(tail, TokenOp::Inc, head),
            "dec" => parse_1reg(tail, TokenOp::Dec, head),
            "prts" => parse_loc(tail, TokenOp::Prts, head),
            "prti" => parse_1reg(tail, TokenOp::Prti, head),
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
    let Some((reg, imm)) = input.split_once(' ') else {
        return Err(Error::Unexpected("load reg imm".into(), input.into()));
    };
    let Reg(r) = reg.parse::<Reg>()?;
    let i = imm.trim()
        .parse::<u32>()
        .map_err(|_| Error::Unexpected(format!("{msg} <num>"), imm.into()))?;
    let [_, _, b2, b1] = i.to_be_bytes();
    Ok(t(r, b2, b1))
}

#[test]
fn from_str_opcode_load() {
    assert_eq!(
        "load %0 123".parse::<TokenOp>(),
        Ok(TokenOp::Load(0, 0, 123))
    );
}

#[test]
fn from_str_opcode_push() {
    assert_eq!("push %0".parse::<TokenOp>(), Ok(TokenOp::Push(0)));
}

#[test]
fn from_str_opcode_pop() {
    assert_eq!("pop %0".parse::<TokenOp>(), Ok(TokenOp::Pop(0)));
}

#[test]
fn from_str_opcode_aloc() {
    assert_eq!("aloc %0".parse::<TokenOp>(), Ok(TokenOp::Aloc(0)));
}

#[test]
fn from_str_opcode_setm() {
    assert_eq!("setm %0 %21".parse::<TokenOp>(), Ok(TokenOp::Setm(0, 21)));
}

#[test]
fn from_str_opcode_add() {
    assert_eq!("add %0 %1 %2".parse::<TokenOp>(), Ok(TokenOp::Add(0, 1, 2)));
}

#[test]
fn from_str_opcode_sub() {
    assert_eq!("sub %0 %1 %2".parse::<TokenOp>(), Ok(TokenOp::Sub(0, 1, 2)));
}

#[test]
fn from_str_opcode_div() {
    assert_eq!("div %0 %1 %2".parse::<TokenOp>(), Ok(TokenOp::Div(0, 1, 2)));
}

#[test]
fn from_str_opcode_mul() {
    assert_eq!("mul %0 %1 %2".parse::<TokenOp>(), Ok(TokenOp::Mul(0, 1, 2)));
}

#[test]
fn from_str_opcode_jmp() {
    assert_eq!(
        "jmp __start__".parse::<TokenOp>(),
        Ok(TokenOp::Jmp(Location("__start__".into())))
    );
    assert_eq!(
        "jmp can_you_parse_this".parse::<TokenOp>(),
        Ok(TokenOp::Jmp(Location("can_you_parse_this".into())))
    );
    assert!("jmp 123".parse::<TokenOp>().is_err());
}

#[test]
fn from_str_opcode_jeq() {
    assert_eq!(
        "jeq start".parse::<TokenOp>(),
        Ok(TokenOp::Jeq(Location("start".into())))
    );
}

#[test]
fn from_str_opcode_jne() {
    assert_eq!(
        "jne start".parse::<TokenOp>(),
        Ok(TokenOp::Jne(Location("start".into())))
    );
}

#[test]
fn from_str_opcode_eq() {
    assert_eq!("eq %10 %11".parse::<TokenOp>(), Ok(TokenOp::Eq(10, 11)));
}

#[test]
fn from_str_opcode_neq() {
    assert_eq!("neq %10 %11".parse::<TokenOp>(), Ok(TokenOp::Neq(10, 11)));
}

#[test]
fn from_str_opcode_gt() {
    assert_eq!("gt %10 %0".parse::<TokenOp>(), Ok(TokenOp::Gt(10, 0)));
}

#[test]
fn from_str_opcode_geq() {
    assert_eq!("geq %10 %23".parse::<TokenOp>(), Ok(TokenOp::Geq(10, 23)));
}

#[test]
fn from_str_opcode_inc() {
    assert_eq!("inc %10".parse::<TokenOp>(), Ok(TokenOp::Inc(10)));
}

#[test]
fn from_str_opcode_dec() {
    assert_eq!("dec %20".parse::<TokenOp>(), Ok(TokenOp::Dec(20)));
}

#[test]
fn from_str_opcode_hlt() {
    assert_eq!("hlt".parse::<TokenOp>(), Ok(TokenOp::Hlt));
}
