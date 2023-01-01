use super::{debug_program, opcode::OpCode};

pub struct Machine {
    registers: [u32; 32],
    program: Vec<u8>,
    pc: usize,
    running: bool,
    compare: bool,
    remainder: u32,
    debug: bool,
}

impl Machine {
    pub fn new(program: Vec<u8>, debug: bool) -> Self {
        Self {
            program,
            registers: [0; 32],
            pc: 0,
            compare: false,
            running: true,
            remainder: 0,
            debug,
        }
    }

    fn read_header(&mut self) {
        let Self { program, pc, .. } = self;
        let mut chunks = program[..64].chunks(4);
        // Magin Number
        let Some(&[0x7F, 0x6e, 0x6f, 0x77]) = chunks.next()else  {
            panic!("invalid magic number");
        };
        // start of .text
        let Some(&[_a, _b, _c, _d]) = chunks.next() else {
            // little endien
            panic!("invalid text offset");
        };

        // entry point
        let Some(&[a, b, c, d]) = chunks.next() else {
            panic!("invalid entry offset");
        };
        *pc = u32::from_le_bytes([a, b, c, d]) as usize;
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
        self.remainder = lhs % rhs;
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

    fn neq(&mut self) {
        let lhs = self.registers[self.get_next_u8() as usize];
        let rhs = self.registers[self.get_next_u8() as usize];
        self.get_next_u8();
        self.compare = lhs != rhs;
    }

    fn gt(&mut self) {
        let lhs = self.registers[self.get_next_u8() as usize];
        let rhs = self.registers[self.get_next_u8() as usize];
        self.get_next_u8();
        self.compare = lhs > rhs;
    }

    fn inc(&mut self) {
        let des = self.get_next_u8() as usize;
        self.registers[des] += 1;
        self.get_next_u8();
        self.get_next_u8();
    }

    fn dec(&mut self) {
        let des = self.get_next_u8() as usize;
        self.registers[des] -= 1;
        self.get_next_u8();
        self.get_next_u8();
    }

    fn prts(&mut self) {
        use std::io::Write;
        let v0 = self.get_next_u8() as u32;
        let v1 = self.get_next_u8() as u32;
        let v2 = self.get_next_u8() as u32;
        let ptr = ((v0 << 8) | (v1 << 4) | v2) as usize;
        let byte_string = self.program[ptr..]
            .iter()
            .take_while(|i| **i != 0)
            .copied()
            .collect::<Vec<u8>>();
        match String::from_utf8(byte_string) {
            Ok(s) => {
                print!("{s}");
                std::io::stdout().flush().expect("failed to flush");
            }
            Err(e) => println!("{e:?}"),
        }
    }

    fn hlt(&mut self) {
        self.running = false;
    }

    fn debug(&self) {
        debug_program(&self.program);
        eprintln!("--- reg ---");
        for (x, regs) in self.registers.chunks(8).enumerate() {
            let r = regs
                .iter()
                .enumerate()
                .map(|(i, r)| {
                    let num = (x * 8) + i;
                    let reg = format!("%{num}");
                    format!("{reg:>3}<-{r:<2}  ")
                })
                .collect::<String>();
            eprintln!("{r}");
        }
    }

    fn get_opcode(&mut self) -> OpCode {
        let opcode = OpCode::from(self.program[self.pc]);
        self.pc += 1;
        opcode
    }

    pub fn run_once(&mut self) {
        let Self { program, .. } = self;
        if program.is_empty() || !self.running {
            eprintln!("nothing to run");
            return;
        }
        match self.get_opcode() {
            OpCode::Load => self.load(),
            OpCode::Add => self.add(),
            OpCode::Sub => self.sub(),
            OpCode::Div => self.div(),
            OpCode::Mul => self.mult(),
            OpCode::Jmp => self.jmp(),
            OpCode::Jeq => self.jeq(),
            OpCode::Jne => self.jne(),
            OpCode::Eq => self.eq(),
            OpCode::Neq => self.neq(),
            OpCode::Gt => self.gt(),
            OpCode::Inc => self.inc(),
            OpCode::Dec => self.dec(),
            OpCode::Prts => self.prts(),
            OpCode::Hlt => self.hlt(),
            OpCode::Nop => {}
            OpCode::Ige => panic!("unknown opcode"),
        }
    }

    pub fn run(&mut self) {
        self.read_header();
        while self.running {
            self.run_once();
            // let opcode = OpCode::from(self.program[self.pc]);
            // let b = self.program[self.pc+1];
            // let c = self.program[self.pc+2];
            // let d = self.program[self.pc+3];
            // eprintln!("{opcode:?} {b} {c} {d}");
            // std::io::stdin().read_line(&mut "".into()).expect("");
        }
        if self.debug {
            self.debug();
        }
    }
}

#[cfg(test)]
mod test {
    use super::Machine;
    use crate::assembler::Assembler;
    #[test]
    fn vm_load() {
        let src = r#"
.entry main
.text
main:
    load %0 123
"#;
        let program = Assembler::new(src).assemble().unwrap();
        let mut vm = Machine::new(program, true);
        vm.read_header();
        vm.run_once();
        let mut right = [0u32; 32];
        right[0] = 123;
        assert_eq!(&vm.registers, &right);
    }

    #[test]
    fn vm_add() {
        let src = r#"
.entry main
.text
main:
    load %0 123
    load %1 321
    add %0 %1 %2
    "#;
        let program = Assembler::new(src).assemble().unwrap();
        let mut vm = Machine::new(program, true);
        vm.read_header();
        vm.run_once();
        vm.run_once();
        vm.run_once();
        let mut right = [0u32; 32];
        right[0] = 123;
        right[1] = 321;
        right[2] = 444;
        assert_eq!(&vm.registers, &right);
    }
    //
    // #[test]
    // fn vm_sub() {
    //     let src = r#"
    // load %0 321
    // load %1 123
    // sub %0 %1 %2
    // "#;
    //     let program = assemble(src);
    //     let mut vm = Machine::new(program, true);
    //     vm.run_once();
    //     vm.run_once();
    //     vm.run_once();
    //     let mut right = [0u32; 32];
    //     right[0] = 321;
    //     right[1] = 123;
    //     right[2] = 198;
    //     assert_eq!(&vm.registers, &right);
    // }
    //
    // #[test]
    // fn vm_mul() {
    //     let src = r#"
    // load %0 321
    // load %1 123
    // mul %0 %1 %2
    // "#;
    //     let program = assemble(src);
    //     let mut vm = Machine::new(program, true);
    //     vm.run_once();
    //     vm.run_once();
    //     vm.run_once();
    //     let mut right = [0u32; 32];
    //     right[0] = 321;
    //     right[1] = 123;
    //     right[2] = 39483;
    //     assert_eq!(&vm.registers, &right);
    // }
    //
    // #[test]
    // fn vm_div() {
    //     let src = r#"
    // load %0 321
    // load %1 123
    // div %0 %1 %2
    // "#;
    //     let program = assemble(src);
    //     let mut vm = Machine::new(program, true);
    //     vm.run_once();
    //     vm.run_once();
    //     vm.run_once();
    //     let mut right = [0u32; 32];
    //     right[0] = 321;
    //     right[1] = 123;
    //     right[2] = 2;
    //     assert_eq!(&vm.registers, &right);
    // }
    //
    // #[test]
    // fn vm_jmp() {
    //     let src = r#"
    // main:
    //     jmp main
    // "#;
    //     let program = assemble(src);
    //     let mut vm = Machine::new(program, true);
    //     assert_eq!(vm.pc, 0, "start");
    //     vm.run_once();
    //     assert_eq!(vm.pc, 0, "after ran once");
    // }
    //
    // #[test]
    // fn vm_jeq() {
    //     let src = r#"
    // main:
    //     eq %0 %1
    //     jeq main
    // "#;
    //     let program = assemble(src);
    //     let mut vm = Machine::new(program, true);
    //     assert_eq!(vm.pc, 0, "start");
    //     vm.run_once();
    //     vm.run_once();
    //     assert_eq!(vm.pc, 0, "after ran once");
    // }
    //
    // #[test]
    // fn vm_eq() {
    //     let src = r#"
    // eq %0 %1
    // "#;
    //     let program = assemble(src);
    //     let mut vm = Machine::new(program, true);
    //     assert!(!vm.compare);
    //     vm.run_once();
    //     assert!(vm.compare);
    // }
    //
    // #[test]
    // fn vm_inc() {
    //     let src = r#"
    // inc %0
    // "#;
    //     let program = assemble(src);
    //     let mut vm = Machine::new(program, true);
    //     vm.run_once();
    //     let mut right = [0u32; 32];
    //     right[0] = 1;
    //     assert_eq!(vm.registers, right);
    // }
    //
    // #[test]
    // fn vm_dec() {
    //     let src = r#"
    // dec %0
    // "#;
    //     let program = assemble(src);
    //     let mut vm = Machine::new(program, true);
    //     vm.registers[0] = 100;
    //     vm.run_once();
    //     let mut right = [0u32; 32];
    //     right[0] = 99;
    //     assert_eq!(vm.registers, right);
    // }
    //
    #[test]
    fn vm_hlt() {
        let src = r#"
.entry main
.text
main:
    hlt
"#;
        let program = Assembler::new(src).assemble().unwrap();
        let mut vm = Machine::new(program, true);
        assert!(vm.running);
        vm.read_header();
        vm.run_once();
        vm.run_once();
        vm.run_once();
        vm.run_once();
        vm.run_once();
        vm.run_once();
        assert!(!vm.running);
    }
}
