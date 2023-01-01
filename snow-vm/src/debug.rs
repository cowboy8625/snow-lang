use super::opcode::OpCode;

pub fn debug_opcode(chunk: &[u8]) -> String {
    let &[a, b, c, d] = chunk else {
        return format!("{chunk:?}");
    };
    let opcode = OpCode::from(a);
    let addr = match opcode {
        OpCode::Jmp | OpCode::Jeq | OpCode::Jne | OpCode::Prts => {
            u32::from_be_bytes([0, b, c, d])
        }
        OpCode::Load => u32::from_be_bytes([0, 0, c, d]),
        _ => 0,
    };
    match opcode {
        OpCode::Load => format!("load %{b} {addr}"),
        OpCode::Add => format!("add %{b} %{c} %{d}"),
        OpCode::Sub => format!("sub %{b} %{c} %{d}"),
        OpCode::Div => format!("div %{b} %{c} %{d}"),
        OpCode::Mul => format!("mul %{b} %{c} %{d}"),
        OpCode::Jmp => format!("jmp {addr}"),
        OpCode::Jeq => format!("jeq {addr}"),
        OpCode::Jne => format!("jne {addr}"),
        OpCode::Eq => format!("eq %{b} {c}"),
        OpCode::Inc => format!("inc %{b}"),
        OpCode::Dec => format!("dec %{b}"),
        OpCode::Hlt => format!("hlt"),
        OpCode::Prts => format!("prts {addr}"),
        OpCode::Nop => format!("nop"),
        OpCode::Ige => format!("ige {a} {c} {c} {d}"),
    }
}

pub fn hex_dump(i: usize, chunk: &[u8]) -> String {
        let c = chunk
            .iter()
            .map(|r| format!("{:<6}", format!("{r:#04X}")))
            .collect::<String>();
        let line_num = i * 4;
        format!("{line_num:>3} {line_num:#04X}: {c}")
}
pub fn hex_dump_chunks(program: &[u8]) {
    for (i, chunk) in program.chunks(4).enumerate() {
        eprintln!("{}", hex_dump(i, chunk));
    }
}

pub fn debug_program(program: &[u8]) {
    hex_dump_chunks(program);
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
    eprintln!("--- header ---");
    hex_dump_chunks(&program[64..text_start]);
    eprintln!("--- .data  ---");
    hex_dump_chunks(&program[64..text_start]);
    eprintln!("--- .text  ---");
    for (i, chunk) in program[text_start..].chunks(4).enumerate() {
        let bytes = hex_dump(i, chunk);
        let opcode = debug_opcode(chunk);
        eprintln!("{:<10} | {}", opcode, bytes);
    }
}
