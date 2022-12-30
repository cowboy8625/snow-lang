#![allow(unused)]
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq, Eq)]
struct UnrecognizedTokenOpError(String);

pub fn parse(src: &str) -> Vec<u8> {
    let mut pc = 0;
    let mut labels = vec![];
    let mut tokens = vec![];
    for line in src.lines() {
        let line = line.trim();
        if let Ok(opcode) = line.parse::<TokenOp>() {
            tokens.push(opcode);
            pc += 4;
            continue;
        }
        if let Ok(Label(label)) = line.parse::<Label>() {
            labels.push((label, pc));
            continue;
        }
    }
    tokens.into_iter().map(|opcode| opcode.into_bytes(&labels)).fold(vec![], |mut acc, item| {
        for code in item {
            acc.push(code);
        }
        acc
    })
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[repr(u8)]
enum TokenOp {
    Load(u8, u8, u8),
    Add(u8, u8, u8),
    Sub(u8, u8, u8),
    Div(u8, u8, u8),
    Mul(u8, u8, u8),
    Jmp(Location),
    Jeq(Location),
    Jne(Location),
    Eq(u8, u8),
    Inc(u8),
    Dec(u8),
    Hlt,
    Ige,
}

impl TokenOp {
    fn into_bytes(self, labels: &[(String, u32)]) -> [u8; 4] {
        let code = OpCode::from(&self) as u8;
        match self {
            Self::Load(a, b, c) => [code, a, b, c],
            Self::Add(a, b, c) => [code, a, b, c],
            Self::Sub(a, b, c) => [code, a, b, c],
            Self::Div(a, b, c) => [code, a, b, c],
            Self::Mul(a, b, c) => [code, a, b, c],
            Self::Jmp(Location(ref name)) |
            Self::Jeq(Location(ref name)) |
            Self::Jne(Location(ref name))=> {
                let Some((_, value)) = labels.iter().find(|(label, _)| label == name) else {
                    panic!("undefined label")
                };
                let [_, b3, b2, b1] = value.to_be_bytes();
                [code,b3,b2,b1]
            },
            Self::Eq(a, b) => [code,a,b,0],
            Self::Inc(a) => [code,a,0,0],
            Self::Dec(a) => [code,a,0,0],
            Self::Hlt => [code, 0, 0, 0],
            Self::Ige => panic!("unknown opcode"),
        }
    }
}

impl FromStr for TokenOp {
    type Err = UnrecognizedTokenOpError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let idx = s.split(' ').collect::<Vec<_>>().first()
            .map(|word| word.len())
            .unwrap_or_default();
        match &s[..idx] {
            "load" => parse_load(&s[idx+1..]),
            "add" => parse_add(&s[idx+1..]),
            "sub" => parse_sub(&s[idx+1..]),
            "div" => parse_div(&s[idx+1..]),
            "mul" => parse_mul(&s[idx+1..]),
            "jmp" => parse_jmp(&s[idx+1..]),
            "jeq" => parse_jeq(&s[idx+1..]),
            "jne" => parse_jne(&s[idx+1..]),
            "eq" => parse_eq(&s[idx+1..]),
            "inc" => parse_inc(&s[idx+1..]),
            "dec" => parse_dec(&s[idx+1..]),
            "hlt" => Ok(TokenOp::Hlt),
            _ => Err(UnrecognizedTokenOpError(s[..idx].into())),
        }
    }
}

fn parse_load(input: &str) -> Result<TokenOp, UnrecognizedTokenOpError> {
    let &[reg, imm] = input.split(' ').collect::<Vec<_>>().as_slice() else {
        return Err(UnrecognizedTokenOpError(input.into()));
    };
    let Reg(r) = reg.parse::<Reg>()?;
    let i = imm.parse::<u32>().map_err(|_| UnrecognizedTokenOpError(imm.into()))?;
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

#[test]
fn from_str_opcode_load() {
    assert_eq!("load %0 123".parse::<TokenOp>(), Ok(TokenOp::Load(0, 0, 123)));
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
    assert_eq!("jmp __start__".parse::<TokenOp>(), Ok(TokenOp::Jmp(Location("__start__".into()))));
    assert_eq!("jmp can_you_parse_this".parse::<TokenOp>(), Ok(TokenOp::Jmp(Location("can_you_parse_this".into()))));
    assert!("jmp 123".parse::<TokenOp>().is_err());

}

#[test]
fn from_str_opcode_jeq() {
    assert_eq!("jeq start".parse::<TokenOp>(), Ok(TokenOp::Jeq(Location("start".into()))));
}

#[test]
fn from_str_opcode_jne() {
    assert_eq!("jne start".parse::<TokenOp>(), Ok(TokenOp::Jne(Location("start".into()))));
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

#[derive(Debug, Clone, PartialEq, Eq)]
struct Label(String);

impl FromStr for Label {
    type Err = UnrecognizedTokenOpError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            return Err(UnrecognizedTokenOpError(s.into()));
        }
        let ":" = &s[s.len()-1..] else {
            return Err(UnrecognizedTokenOpError(s.into()));
        };

        let mut chars = s[..s.len()-1].chars();
        let Some(c) = chars.next() else {
            return Err(UnrecognizedTokenOpError(s.into()));
        };
        if !(c.is_ascii_alphabetic() || c == '_') {
            return Err(UnrecognizedTokenOpError(s.into()));
        }
        for c in chars {
            if !(c.is_ascii_alphanumeric() || c == '_') {
                return Err(UnrecognizedTokenOpError(s.into()));
            }
        }
        Ok(Self(s[..s.len()-1].to_string()))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Reg(u8);

impl FromStr for Reg {
    type Err = UnrecognizedTokenOpError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let "%" = &s[..1] else {
            return Err(UnrecognizedTokenOpError(s.into()));
        };
        let reg = &s[1..].parse::<u8>().map_err(|_| UnrecognizedTokenOpError(s.into()))?;
        Ok(Self(*reg))
    }
}


#[derive(Debug, Clone, PartialEq, Eq)]
struct Location(String);

impl FromStr for Location {
    type Err = UnrecognizedTokenOpError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            return Err(UnrecognizedTokenOpError(s.into()));
        }
        let mut chars = s[..s.len()-1].chars();
        let Some(c) = chars.next() else {
            return Err(UnrecognizedTokenOpError(s.into()));
        };
        if !(c.is_ascii_alphabetic() || c == '_') {
            return Err(UnrecognizedTokenOpError(s.into()));
        }
        for c in chars {
            if !(c.is_ascii_alphanumeric() || c == '_') {
                return Err(UnrecognizedTokenOpError(s.into()));
            }
        }
        Ok(Self(s[..s.len()].to_string()))
    }
}


#[test]
fn parse_test() {
    let src = r#"
start:
    load %0 100
    add %0 %1 %0
    jmp start
    "#;
    let bytes = parse(src);
    assert_eq!(bytes, vec![
               0, 0, 0, 100,
               1, 0, 1, 0,
               5, 0, 0, 0,
    ]);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OpCode {
    Load, // ✓
    Add, // ✓
    Sub, // ✓
    Div, // ✓
    Mul, // ✓
    Jmp, // ✓
    Jeq, // ✓
    Jne, // ✓
    Eq, // ✓
    Inc,
    Dec,
    // Push,
    // Pop,
    // Call
    // Ret,
    Hlt, // ✓
    Ige, // ✓
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
            TokenOp::Hlt => Self::Hlt,
            _ => unimplemented!("{:?}", value),
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
            11 => Self::Hlt,
            _ => Self::Ige,
        }
        
    }
}

pub fn debug_program(program: &[u8]) {
    // FIXME:  check if none used btyes are 0
    for chunk in program.chunks(4) {
        let &[a, b, c, d] = chunk else {
            eprintln!("{chunk:?}");
            return;
        };
        let opcode = OpCode::from(a);
        let addr = match opcode {
            OpCode::Jmp | OpCode::Jeq | OpCode::Jne => {
                u32::from_be_bytes([0, b, c ,d])
            }
            OpCode::Load => {
                u32::from_be_bytes([0, 0, c, d])
            },
            _ => 0,
        };
        match opcode {
            OpCode::Load => eprintln!("load %{b} {addr}"),
            OpCode::Add  => eprintln!("add %{b} %{c} %{d}"),
            OpCode::Sub  => eprintln!("sub %{b} %{c} %{d}"),
            OpCode::Div  => eprintln!("div %{b} %{c} %{d}"),
            OpCode::Mul  => eprintln!("mul %{b} %{c} %{d}"),
            OpCode::Jmp  => eprintln!("jmp {addr}"),
            OpCode::Jeq  => eprintln!("jeq {addr}"),
            OpCode::Jne  => eprintln!("jne {addr}"),
            OpCode::Eq   => eprintln!("eq %{b} {c}"),
            OpCode::Inc   => eprintln!("inc %{b}"),
            OpCode::Dec   => eprintln!("dec %{b}"),
            OpCode::Hlt  => eprintln!("hlt"),
            OpCode::Ige  => eprintln!("ige {a} {c} {c} {d}"),

        }
    }
}


pub struct Machine {
    registers: [u32; 32],
    program: Vec<u8>,
    pc: usize,
    running: bool,
    compare: bool,
}

impl Machine {
    pub fn new(program: Vec<u8>) -> Self {
    Self {
        program,
        registers: [0; 32],
        pc: 0,
        compare: false,
        running: true,
    }
    }
    fn get_next_u8(&mut self) -> u8 {
        let pc = self.pc;
        self.pc += 1;
        self.program[pc]
    }
    fn load(&mut self) {
        let des = self.get_next_u8() as usize;
        let v1 = self.get_next_u8() as u32;
        let v2 = self.get_next_u8() as u32;
        let value = (v1 << 8) | v2;
        self.registers[des] = value;
    }

    fn add(&mut self) {
        let lhs = self.registers[self.get_next_u8() as usize];
        let rhs = self.registers[self.get_next_u8() as usize];
        let des = self.get_next_u8() as usize;
        self.registers[des] = lhs + rhs;
    }

    fn sub(&mut self) {
        let lhs = self.registers[self.get_next_u8() as usize];
        let rhs = self.registers[self.get_next_u8() as usize];
        let des = self.get_next_u8() as usize;
        self.registers[des] = lhs - rhs;
    }

    fn div(&mut self) {
        let lhs = self.registers[self.get_next_u8() as usize];
        let rhs = self.registers[self.get_next_u8() as usize];
        let des = self.get_next_u8() as usize;
        self.registers[des] = lhs / rhs;
    }

    fn mult(&mut self) {
        let lhs = self.registers[self.get_next_u8() as usize];
        let rhs = self.registers[self.get_next_u8() as usize];
        let des = self.get_next_u8() as usize;
        self.registers[des] = lhs * rhs;
    }

    fn jmp(&mut self) {
        let v0 = self.get_next_u8() as u32;
        let v1 = self.get_next_u8() as u32;
        let v2 = self.get_next_u8() as u32;
        let value = ((v0 << 8) | (v1 << 4) | v2) as usize;
        self.pc = value;
    }

    fn jeq(&mut self) {
        if self.compare {
            self.jmp();
            self.compare = false;
            return;
        }
        self.pc += 3;
    }

    fn jne(&mut self) {
        if !self.compare {
            self.jmp();
            self.compare = false;
            return;
        }
        self.pc += 3;
    }

    fn eq(&mut self) {
        let lhs = self.registers[self.get_next_u8() as usize];
        let rhs = self.registers[self.get_next_u8() as usize];
        self.get_next_u8();
        self.compare = lhs == rhs;
    }

    fn inc(&mut self) {
        let des = self.get_next_u8() as usize;
        self.registers[des] += 1;
    }

    fn dec(&mut self) {
        let des = self.get_next_u8() as usize;
        self.registers[des] -= 1;
    }

    fn hlt(&mut self) {
        self.running = false;
    }

    pub fn run_once(&mut self) {
        let Self { program, pc, .. } = self;
        if program.is_empty() {
            eprintln!("nothing to run");
            self.hlt();
            return;
        }
        let opcode = (program[*pc]).try_into();
        let Ok(opcode) = opcode else {
            eprintln!("encountered unknown byte code");
            self.hlt();
            return;
        };
        self.pc += 1;
        match opcode {
            OpCode::Load => self.load(),
            OpCode::Add => self.add(),
            OpCode::Sub => self.sub(),
            OpCode::Div => self.div(),
            OpCode::Mul => self.mult(),
            OpCode::Jmp => self.jmp(),
            OpCode::Jeq => self.jeq(),
            OpCode::Jne => self.jne(),
            OpCode::Eq => self.eq(),
            OpCode::Inc => self.inc(),
            OpCode::Dec => self.dec(),
            OpCode::Hlt => self.hlt(),
            OpCode::Ige => panic!("unknown opcode"),
        }
    }

    pub fn run(&mut self) {
        while self.running {
            self.run_once();
        }
        // eprintln!("pc: {} program len: {}", &self.pc, self.program.len());
        for regs in self.registers.chunks(8) {
            let r = regs.iter().map(|r| format!("r{r:<4}")).collect::<String>();
            eprintln!("{r}");
        }

        for regs in self.program.chunks(4) {
            let r = regs.iter().map(|r| format!("{:<6}", format!("{r:#04X}"))).collect::<String>();
            eprintln!("{r}");
        }
    }
}

#[test]
fn vm_load() {
    let program = parse("load %0 123");
    let mut vm = Machine::new(program);
    vm.run_once();
    let mut right = [0u32; 32];
    right[0] = 123;
    assert_eq!(&vm.registers, &right);
}

#[test]
fn vm_add() {
    let src = r#"
load %0 123
load %1 321
add %0 %1 %2
"#;
    let program = parse(src);
    let mut vm = Machine::new(program);
    vm.run_once();
    vm.run_once();
    vm.run_once();
    let mut right = [0u32; 32];
    right[0] = 123;
    right[1] = 321;
    right[2] = 444;
    assert_eq!(&vm.registers, &right);
}

#[test]
fn vm_sub() {
    let src = r#"
load %0 321
load %1 123
sub %0 %1 %2
"#;
    let program = parse(src);
    let mut vm = Machine::new(program);
    vm.run_once();
    vm.run_once();
    vm.run_once();
    let mut right = [0u32; 32];
    right[0] = 321;
    right[1] = 123;
    right[2] = 198;
    assert_eq!(&vm.registers, &right);
}

#[test]
fn vm_mul() {
    let src = r#"
load %0 321
load %1 123
mul %0 %1 %2
"#;
    let program = parse(src);
    let mut vm = Machine::new(program);
    vm.run_once();
    vm.run_once();
    vm.run_once();
    let mut right = [0u32; 32];
    right[0] = 321;
    right[1] = 123;
    right[2] = 39483;
    assert_eq!(&vm.registers, &right);
}

#[test]
fn vm_div() {
    let src = r#"
load %0 321
load %1 123
div %0 %1 %2
"#;
    let program = parse(src);
    let mut vm = Machine::new(program);
    vm.run_once();
    vm.run_once();
    vm.run_once();
    let mut right = [0u32; 32];
    right[0] = 321;
    right[1] = 123;
    right[2] = 2;
    assert_eq!(&vm.registers, &right);
}

#[test]
fn vm_jmp() {
    let src = r#"
main:
    jmp main
"#;
    let program = parse(src);
    let mut vm = Machine::new(program);
    assert_eq!(vm.pc, 0, "start");
    vm.run_once();
    assert_eq!(vm.pc, 0, "after ran once");
}

#[test]
fn vm_jeq() {
    let src = r#"
main:
    eq %0 %1
    jeq main
"#;
    let program = parse(src);
    let mut vm = Machine::new(program);
    assert_eq!(vm.pc, 0, "start");
    vm.run_once();
    vm.run_once();
    assert_eq!(vm.pc, 0, "after ran once");
}

#[test]
fn vm_eq() {
    let src = r#"
eq %0 %1
"#;
    let program = parse(src);
    let mut vm = Machine::new(program);
    assert!(!vm.compare);
    vm.run_once();
    assert!(vm.compare);
}

#[test]
fn vm_inc() {
    let src = r#"
inc %0
"#;
    let program = parse(src);
    let mut vm = Machine::new(program);
    vm.run_once();
    let mut right = [0u32; 32];
    right[0] = 1;
    assert_eq!(vm.registers, right);
}

#[test]
fn vm_dec() {
    let src = r#"
dec %0
"#;
    let program = parse(src);
    let mut vm = Machine::new(program);
    vm.registers[0] = 100;
    vm.run_once();
    let mut right = [0u32; 32];
    right[0] = 99;
    assert_eq!(vm.registers, right);
}

#[test]
fn vm_hlt() {
    let src = r#"hlt"#;
    let program = parse(src);
    let mut vm = Machine::new(program);
    assert!(vm.running);
    vm.run_once();
    assert!(!vm.running);
}
