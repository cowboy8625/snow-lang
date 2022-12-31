mod args;
use vm::{Assembler, Machine};

fn main() {
    let settings = args::cargs();
    let Some(filename) = &settings.filename else {
        eprintln!("expected a filename");
        return;
    };
    let src = std::fs::read_to_string(filename)
        .unwrap_or_else(|_| panic!("failed to open file '{}'", filename));
    match Assembler::new(&src).assemble() {
        Ok(program) => {
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
