mod assembler;
mod debug;
mod machine;
mod opcode;
mod parse;

pub use assembler::{assembler, SymbolTable};
pub use debug::{debug_opcode, debug_program, hex_dump};
pub use machine::Machine;

// #[test]
// fn parse_test() {
//     let src = r#"
// start:
//     load %0 100
//     add %0 %1 %0
//     jmp start
//     "#;
//     let bytes = assemble(src);
//     assert_eq!(bytes, vec![0, 0, 0, 100, 1, 0, 1, 0, 5, 0, 0, 0,]);
// }
