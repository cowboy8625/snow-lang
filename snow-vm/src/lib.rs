mod assembler;
mod error;
mod machine;
mod parse;
mod debug;
mod opcode;

pub use assembler::{SymbolTable, Assembler};
pub use debug::{debug_opcode, hex_dump, debug_program};

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


