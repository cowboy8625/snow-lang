mod args;
use vm::{Machine, parse, debug_program};


fn main() {
    let settings = args::cargs();
    let Some(filename) = settings.filename else {
        eprintln!("expected a filename");
        return;
    };
    let src = std::fs::read_to_string(filename).expect(&format!("failed to open file '{}'", filename));
    let program = parse(&src);
    debug_program(&program);

    let mut vm = Machine::new(program);

    vm.run();
}
