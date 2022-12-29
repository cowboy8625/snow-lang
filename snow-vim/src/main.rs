use vm::{Machine, parse, debug_program};


fn main() {
    let src = r#"
main:
        load %1 1
        load %2 10
loop:
        add %0 %1 %0
        eq %0 %2
        jne loop
        hlt
"#;
    println!("{src}\n");
    let program = parse(src);
    debug_program(&program);

    let mut vm = Machine::new(program);

    vm.run();
}
