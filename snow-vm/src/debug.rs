use super::opcode::OpCode;

pub fn debug_opcode(chunk: &[u8]) {
    let &[a, b, c, d] = chunk else {
        eprintln!("{chunk:?}");
        return;
    };
    let opcode = OpCode::from(a);
    let addr = match opcode {
        OpCode::Jmp | OpCode::Jeq | OpCode::Jne | OpCode::Prts => u32::from_be_bytes([0, b, c, d]),
        OpCode::Load => u32::from_be_bytes([0, 0, c, d]),
        _ => 0,
    };
    match opcode {
        OpCode::Load => eprintln!("load %{b} {addr}"),
        OpCode::Add => eprintln!("add %{b} %{c} %{d}"),
        OpCode::Sub => eprintln!("sub %{b} %{c} %{d}"),
        OpCode::Div => eprintln!("div %{b} %{c} %{d}"),
        OpCode::Mul => eprintln!("mul %{b} %{c} %{d}"),
        OpCode::Jmp => eprintln!("jmp {addr}"),
        OpCode::Jeq => eprintln!("jeq {addr}"),
        OpCode::Jne => eprintln!("jne {addr}"),
        OpCode::Eq => eprintln!("eq %{b} {c}"),
        OpCode::Inc => eprintln!("inc %{b}"),
        OpCode::Dec => eprintln!("dec %{b}"),
        OpCode::Hlt => eprintln!("hlt"),
        OpCode::Prts => eprintln!("prts {addr}"),
        OpCode::Ige => eprintln!("ige {a} {c} {c} {d}"),
    }
}

pub fn hex_dump(program: &[u8]) {
    for (i, regs) in program.chunks(4).enumerate() {
        if i * 4 == 64 {
            eprintln!("----------");
        }
        let r = regs
            .iter()
            .map(|r| format!("{:<6}", format!("{r:#04X}")))
            .collect::<String>();
        let line_num = i * 4;
        eprintln!("{line_num} {line_num:#04X}: {r}");
    }
}

pub fn debug_program(program: &[u8]) {
    hex_dump(program);
    let mut chunks = program[..64].chunks(4);
    // Magin Number
    let Some(&[0x7F, 0x6e, 0x6f, 0x77]) = chunks.next()else  {
        panic!("invalid magic number");
    };
    // start of .text
    let Some(&[a, b, c, d]) = chunks.next() else {
        // little endien
        panic!("invalid text offset");
    };
    let text_start = u32::from_le_bytes([a, b, c, d]) as usize;

    // entry point
    let Some(&[a, b, c, d]) = chunks.next() else {
        panic!("invalid entry offset");
    };
    let entry_point = u32::from_le_bytes([a, b, c, d]) as usize;

    for chunk in program[text_start..entry_point].chunks(4) {
        debug_opcode(chunk);
    }
    for chunk in program[entry_point..].chunks(4) {
        debug_opcode(chunk);
    }
}
