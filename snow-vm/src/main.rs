mod args;
use vm::{Assembler, Machine};
use std::io::Read;

fn remove_she_bang_bin(program: &mut Vec<u8>) {
    let [a, b] = program[0..2] else {
        panic!("program file is to short");
    };
    if a == b'#' && b == b'!' {
        let Some(idx) = program.iter().position(|i| i == &b'\n') else {
            panic!("she bang mest up");
        };
        *program = program[idx+1..].to_vec();
    }
}

fn remove_she_bang_src(src: &mut String) {
    if src.starts_with("#!/") {
        let (_head, tail) = src.split_once('\n').unwrap_or_default();
        println!("{_head:?}");
        println!("{tail:?}");
        *src = tail.to_string();
    }
}

fn main() {
    let settings = args::cargs();
    let Some(filename) = &settings.filename else {
        eprintln!("expected a filename");
        return;
    };
    if settings.bin_file {
        let mut file = std::fs::OpenOptions::new()
            .read(true)
            .open(filename).expect("failed to open file");
        let mut program = vec![];
        file.read_to_end(&mut program).expect("filed to read bin file");
        remove_she_bang_bin(&mut program);
        let mut vm = Machine::new(program, settings.debug);
        vm.run();
        return;
    }
    let mut src = std::fs::read_to_string(filename)
        .unwrap_or_else(|_| panic!("failed to open file '{}'", filename));
    remove_she_bang_src(&mut src);
    match Assembler::new(&src).assemble() {
        Ok(program) => {
            use std::io::Write;
            let mut file = std::fs::OpenOptions::new()
                .create(true)
                .write(true)
                .truncate(true)
                .open("helloworld").expect("failed to open file");
            file.write_all(&program).expect("failed to write to file");

            let mut vm = Machine::new(program, settings.debug);
            vm.run();
        }
        Err(errors) => {
            for e in errors.iter() {
                eprintln!("{e:?}");
            }
        }
    }
}
