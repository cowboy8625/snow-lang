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
            Self::Add(a, b, c) |
            Self::Sub(a, b, c) |
            Self::Div(a, b, c) |
            Self::Mod(a, b, c) |
            Self::Mul(a, b, c) => Ok([code, a, b, c]),
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
            Self::Setm(a, b) |
            Self::Eq(a, b) |
            Self::Neq(a, b)|
            Self::Geq(a, b)|
            Self::Gt(a, b) =>  Ok([code, a, b, 0]),
            Self::Leq(a, b)|
            Self::Lt(a, b) =>  Ok([code, a, b, 0]),
            Self::Prti(a) |
            Self::Inc(a) |
            Self::Push(a) |
            Self::Pop(a) |
            Self::Aloc(a) |
            Self::Dec(a) => Ok([code, a, 0, 0]),
            Self::Hlt |
            Self::Nop => Ok([code, 0, 0, 0]),
        }
    }
}

impl FromStr for TokenOp {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (head, tail) = s.split_once(' ').unwrap_or((s, ""));
        match head {
            "load" => parse_load(tail),
            "push" => parse_push(tail),
            "pop" => parse_pop(tail),
            "aloc" => parse_aloc(tail),
            "setm" => parse_setm(tail),
            "add" => parse_add(tail),
            "sub" => parse_sub(tail),
            "div" => parse_div(tail),
            "mod" => parse_mod(tail),
            "mul" => parse_mul(tail),
            "jmp" => parse_jmp(tail),
            "jeq" => parse_jeq(tail),
            "jne" => parse_jne(tail),
            "eq" => parse_eq(tail),
            "neq" => parse_neq(tail),
            "gt" => parse_gt(tail),
            "geq" => parse_geq(tail),
            "lt" => parse_lt(tail),
            "leq" => parse_leq(tail),
            "inc" => parse_inc(tail),
            "dec" => parse_dec(tail),
            "prts" => parse_prts(tail),
            "prti" => parse_prti(tail),
            "hlt" => Ok(TokenOp::Hlt),
            "nop" => Ok(TokenOp::Nop),
            _ => Err(Error::Unexpected("not a opcode".into(), head.into())),
        }
    }
}

fn parse_load(input: &str) -> Result<TokenOp, Error> {
    let Some((reg, imm)) = input.split_once(' ') else {
        return Err(Error::Unexpected("load reg imm".into(), input.into()));
    };
    let Reg(r) = reg.parse::<Reg>()?;
    let i = imm
        .parse::<u32>()
        .map_err(|_| Error::Unexpected("<num>".into(), imm.into()))?;
    let [_, _, b2, b1] = i.to_be_bytes();
    Ok(TokenOp::Load(r, b2, b1))
}

fn parse_aloc(input: &str) -> Result<TokenOp, Error> {
    let Ok(TokenOp::Inc(r1)) = parse_inc(input) else {
        return Err(Error::Unexpected("aloc reg".into(), input.into()));
    };
    Ok(TokenOp::Aloc(r1))
}

fn parse_setm(input: &str) -> Result<TokenOp, Error> {
    let Ok(TokenOp::Eq(r1, r2)) = parse_eq(input) else {
        return Err(Error::Unexpected("setm reg reg".into(), input.into()));
    };
    Ok(TokenOp::Setm(r1, r2))
}

fn parse_add(input: &str) -> Result<TokenOp, Error> {
    let &[reg1, reg2, reg3] = input.split(' ').collect::<Vec<_>>().as_slice() else {
        return Err(Error::Unexpected("add reg reg reg".into(), input.into()));
    };
    let Reg(r1) = reg1.parse::<Reg>()?;
    let Reg(r2) = reg2.parse::<Reg>()?;
    let Reg(r3) = reg3.parse::<Reg>()?;
    Ok(TokenOp::Add(r1, r2, r3))
}

fn parse_sub(input: &str) -> Result<TokenOp, Error> {
    let Ok(TokenOp::Add(r1, r2, r3)) = parse_add(input) else {
        return Err(Error::Unexpected("sub reg reg reg".into(), input.into()));
    };
    Ok(TokenOp::Sub(r1, r2, r3))
}

fn parse_div(input: &str) -> Result<TokenOp, Error> {
    let Ok(TokenOp::Add(r1, r2, r3)) = parse_add(input) else {
        return Err(Error::Unexpected("div reg reg reg".into(), input.into()));
    };
    Ok(TokenOp::Div(r1, r2, r3))
}

fn parse_mod(input: &str) -> Result<TokenOp, Error> {
    let Ok(TokenOp::Add(r1, r2, r3)) = parse_add(input) else {
        return Err(Error::Unexpected("mod reg reg reg".into(), input.into()));
    };
    Ok(TokenOp::Mod(r1, r2, r3))
}

fn parse_mul(input: &str) -> Result<TokenOp, Error> {
    let Ok(TokenOp::Add(r1, r2, r3)) = parse_add(input) else {
        return Err(Error::Unexpected("mul reg reg reg".into(), input.into()));
    };
    Ok(TokenOp::Mul(r1, r2, r3))
}

fn parse_jmp(input: &str) -> Result<TokenOp, Error> {
    let &[loc] = input.split(' ').collect::<Vec<_>>().as_slice() else {
        return Err(Error::Unexpected("jum label_name".into(), input.into()));
    };
    let loc = loc.parse::<Location>()?;
    Ok(TokenOp::Jmp(loc))
}

fn parse_jeq(input: &str) -> Result<TokenOp, Error> {
    let Ok(TokenOp::Jmp(loc)) = parse_jmp(input) else {
        return Err(Error::Unexpected("jeq label_name".into(), input.into()));
    };
    Ok(TokenOp::Jeq(loc))
}

fn parse_jne(input: &str) -> Result<TokenOp, Error> {
    let Ok(TokenOp::Jmp(loc)) = parse_jmp(input) else {
        return Err(Error::Unexpected("jne label_name".into(), input.into()));
    };
    Ok(TokenOp::Jne(loc))
}

fn parse_eq(input: &str) -> Result<TokenOp, Error> {
    let &[reg1, reg2] = input.split(' ').collect::<Vec<_>>().as_slice() else {
        return Err(Error::Unexpected("eq reg reg".into(), input.into()));
    };
    let Reg(r1) = reg1.parse::<Reg>()?;
    let Reg(r2) = reg2.parse::<Reg>()?;
    Ok(TokenOp::Eq(r1, r2))
}

fn parse_neq(input: &str) -> Result<TokenOp, Error> {
    let Ok(TokenOp::Eq(r1, r2)) = parse_eq(input) else {
        return Err(Error::Unexpected("neq reg reg".into(), input.into()));
    };
    Ok(TokenOp::Neq(r1, r2))
}
fn parse_gt(input: &str) -> Result<TokenOp, Error> {
    let Ok(TokenOp::Eq(r1, r2)) = parse_eq(input) else {
        return Err(Error::Unexpected("gt reg reg".into(), input.into()));
    };
    Ok(TokenOp::Gt(r1, r2))
}

fn parse_geq(input: &str) -> Result<TokenOp, Error> {
    let Ok(TokenOp::Eq(r1, r2)) = parse_eq(input) else {
        return Err(Error::Unexpected("geq reg reg".into(), input.into()));
    };
    Ok(TokenOp::Geq(r1, r2))
}

fn parse_lt(input: &str) -> Result<TokenOp, Error> {
    let Ok(TokenOp::Eq(r1, r2)) = parse_eq(input) else {
        return Err(Error::Unexpected("lt reg reg".into(), input.into()));
    };
    Ok(TokenOp::Lt(r1, r2))
}

fn parse_leq(input: &str) -> Result<TokenOp, Error> {
    let Ok(TokenOp::Eq(r1, r2)) = parse_eq(input) else {
        return Err(Error::Unexpected("leq reg reg".into(), input.into()));
    };
    Ok(TokenOp::Leq(r1, r2))
}

fn parse_inc(input: &str) -> Result<TokenOp, Error> {
    let &[reg1] = input.split(' ').collect::<Vec<_>>().as_slice() else {
        return Err(Error::Unexpected("inc reg".into(), input.into()));
    };
    let Reg(r1) = reg1.parse::<Reg>()?;
    Ok(TokenOp::Inc(r1))
}

fn parse_dec(input: &str) -> Result<TokenOp, Error> {
    let Ok(TokenOp::Inc(r1)) = parse_inc(input) else {
        return Err(Error::Unexpected("dec reg".into(), input.into()));
    };
    Ok(TokenOp::Dec(r1))
}
fn parse_push(input: &str) -> Result<TokenOp, Error> {
    let Ok(TokenOp::Inc(r1)) = parse_inc(input)else {
        return Err(Error::Unexpected("push reg".into(), input.into()));
    };
    Ok(TokenOp::Push(r1))
}
fn parse_pop(input: &str) -> Result<TokenOp, Error> {
    let Ok(TokenOp::Inc(r1)) = parse_inc(input) else {
        return Err(Error::Unexpected("pop reg".into(), input.into()));
    };
    Ok(TokenOp::Pop(r1))
}

fn parse_prts(input: &str) -> Result<TokenOp, Error> {
    let Ok(TokenOp::Jmp(loc)) = parse_jmp(input) else {
        return Err(Error::Unexpected("prts label_name".into(), input.into()));
    };
    Ok(TokenOp::Prts(loc))
}

fn parse_prti(input: &str) -> Result<TokenOp, Error> {
    let Ok(TokenOp::Inc(r1)) = parse_inc(input) else {
        return Err(Error::Unexpected("dec reg".into(), input.into()));
    };
    Ok(TokenOp::Prti(r1))
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
    assert_eq!(
        "push %0".parse::<TokenOp>(),
        Ok(TokenOp::Push(0))
    );
}

#[test]
fn from_str_opcode_pop() {
    assert_eq!(
        "pop %0".parse::<TokenOp>(),
        Ok(TokenOp::Pop(0))
    );
}

#[test]
fn from_str_opcode_aloc() {
    assert_eq!(
        "aloc %0".parse::<TokenOp>(),
        Ok(TokenOp::Aloc(0))
    );
}

#[test]
fn from_str_opcode_setm() {
    assert_eq!(
        "setm %0 %21".parse::<TokenOp>(),
        Ok(TokenOp::Setm(0, 21))
    );
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
