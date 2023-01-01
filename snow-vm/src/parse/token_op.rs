use super::*;
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq, Eq)]
#[repr(u8)]
pub enum TokenOp {
    Load(u8, u8, u8),
    Add(u8, u8, u8),
    Sub(u8, u8, u8),
    Div(u8, u8, u8),
    Mul(u8, u8, u8),
    Jmp(Location),
    Jeq(Location),
    Jne(Location),
    Eq(u8, u8),
    Neq(u8, u8),
    Inc(u8),
    Dec(u8),
    Prts(Location),
    Hlt,
    Nop,
}

impl TokenOp {
    pub fn into_bytes(self, labels: &SymbolTable) -> Result<[u8; 4], Error> {
        let code = OpCode::from(&self) as u8;
        match self {
            Self::Load(a, b, c) => Ok([code, a, b, c]),
            Self::Add(a, b, c) => Ok([code, a, b, c]),
            Self::Sub(a, b, c) => Ok([code, a, b, c]),
            Self::Div(a, b, c) => Ok([code, a, b, c]),
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
            Self::Eq(a, b) => Ok([code, a, b, 0]),
            Self::Neq(a, b) => Ok([code, a, b, 0]),
            Self::Inc(a) => Ok([code, a, 0, 0]),
            Self::Dec(a) => Ok([code, a, 0, 0]),
            Self::Hlt => Ok([code, 0, 0, 0]),
            Self::Nop => Ok([code, 0, 0, 0]),
        }
    }
}

impl FromStr for TokenOp {
    type Err = UnrecognizedTokenOpError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (head, tail) = s.split_once(' ').unwrap_or((s, ""));
        match head {
            "load" => parse_load(tail),
            "add" => parse_add(tail),
            "sub" => parse_sub(tail),
            "div" => parse_div(tail),
            "mul" => parse_mul(tail),
            "jmp" => parse_jmp(tail),
            "jeq" => parse_jeq(tail),
            "jne" => parse_jne(tail),
            "eq" => parse_eq(tail),
            "neq" => parse_neq(tail),
            "inc" => parse_inc(tail),
            "dec" => parse_dec(tail),
            "prts" => parse_prts(tail),
            "hlt" => Ok(TokenOp::Hlt),
            "nop" => Ok(TokenOp::Nop),
            _ => Err(UnrecognizedTokenOpError(tail.into())),
        }
    }
}

fn parse_load(input: &str) -> Result<TokenOp, UnrecognizedTokenOpError> {
    let Some((reg, imm)) = input.split_once(' ') else {
        return Err(UnrecognizedTokenOpError(input.into()));
    };
    let Reg(r) = reg.parse::<Reg>()?;
    let i = imm
        .parse::<u32>()
        .map_err(|_| UnrecognizedTokenOpError(imm.into()))?;
    let [_, _, b2, b1] = i.to_be_bytes();
    Ok(TokenOp::Load(r, b2, b1))
}

fn parse_add(input: &str) -> Result<TokenOp, UnrecognizedTokenOpError> {
    let &[reg1, reg2, reg3] = input.split(' ').collect::<Vec<_>>().as_slice() else {
        return Err(UnrecognizedTokenOpError(input.into()));
    };
    let Reg(r1) = reg1.parse::<Reg>()?;
    let Reg(r2) = reg2.parse::<Reg>()?;
    let Reg(r3) = reg3.parse::<Reg>()?;
    Ok(TokenOp::Add(r1, r2, r3))
}

fn parse_sub(input: &str) -> Result<TokenOp, UnrecognizedTokenOpError> {
    let TokenOp::Add(r1, r2, r3) = parse_add(input)? else {
        return Err(UnrecognizedTokenOpError(input.into()));
    };
    Ok(TokenOp::Sub(r1, r2, r3))
}

fn parse_div(input: &str) -> Result<TokenOp, UnrecognizedTokenOpError> {
    let TokenOp::Add(r1, r2, r3) = parse_add(input)? else {
        return Err(UnrecognizedTokenOpError(input.into()));
    };
    Ok(TokenOp::Div(r1, r2, r3))
}

fn parse_mul(input: &str) -> Result<TokenOp, UnrecognizedTokenOpError> {
    let TokenOp::Add(r1, r2, r3) = parse_add(input)? else {
        return Err(UnrecognizedTokenOpError(input.into()));
    };
    Ok(TokenOp::Mul(r1, r2, r3))
}

fn parse_jmp(input: &str) -> Result<TokenOp, UnrecognizedTokenOpError> {
    let &[loc] = input.split(' ').collect::<Vec<_>>().as_slice() else {
        return Err(UnrecognizedTokenOpError(input.into()));
    };
    let loc = loc.parse::<Location>()?;
    Ok(TokenOp::Jmp(loc))
}

fn parse_jeq(input: &str) -> Result<TokenOp, UnrecognizedTokenOpError> {
    let TokenOp::Jmp(loc) = parse_jmp(input)? else {
        return Err(UnrecognizedTokenOpError(input.into()));
    };
    Ok(TokenOp::Jeq(loc))
}

fn parse_jne(input: &str) -> Result<TokenOp, UnrecognizedTokenOpError> {
    let TokenOp::Jmp(loc) = parse_jmp(input)? else {
        return Err(UnrecognizedTokenOpError(input.into()));
    };
    Ok(TokenOp::Jne(loc))
}

fn parse_eq(input: &str) -> Result<TokenOp, UnrecognizedTokenOpError> {
    let &[reg1, reg2] = input.split(' ').collect::<Vec<_>>().as_slice() else {
        return Err(UnrecognizedTokenOpError(input.into()));
    };
    let Reg(r1) = reg1.parse::<Reg>()?;
    let Reg(r2) = reg2.parse::<Reg>()?;
    Ok(TokenOp::Eq(r1, r2))
}

fn parse_neq(input: &str) -> Result<TokenOp, UnrecognizedTokenOpError> {
    let TokenOp::Eq(r1, r2) = parse_eq(input)? else {
        return Err(UnrecognizedTokenOpError(input.into()));
    };
    Ok(TokenOp::Neq(r1, r2))
}

fn parse_inc(input: &str) -> Result<TokenOp, UnrecognizedTokenOpError> {
    let &[reg1] = input.split(' ').collect::<Vec<_>>().as_slice() else {
        return Err(UnrecognizedTokenOpError(input.into()));
    };
    let Reg(r1) = reg1.parse::<Reg>()?;
    Ok(TokenOp::Inc(r1))
}

fn parse_dec(input: &str) -> Result<TokenOp, UnrecognizedTokenOpError> {
    let TokenOp::Inc(r1) = parse_inc(input)? else {
        return Err(UnrecognizedTokenOpError(input.into()));
    };
    Ok(TokenOp::Dec(r1))
}

fn parse_prts(input: &str) -> Result<TokenOp, UnrecognizedTokenOpError> {
    let TokenOp::Jmp(loc) = parse_jmp(input)? else {
        return Err(UnrecognizedTokenOpError(input.into()));
    };
    Ok(TokenOp::Prts(loc))
}

#[test]
fn from_str_opcode_load() {
    assert_eq!(
        "load %0 123".parse::<TokenOp>(),
        Ok(TokenOp::Load(0, 0, 123))
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
