#![allow(dead_code)]
#![allow(warnings)]

// mod wasm_runtime;
// fn main() -> anyhow::Result<()> {
//     let Some(filename) = std::env::args().nth(1) else {
//         println!("Usage: {} <filename>", std::env::args().next().unwrap());
//         return Ok(());
//     };
//     let bytes = std::fs::read_to_string(&filename)?;
//
//     wasm_runtime::run(&bytes.into_bytes())?;
//
//     Ok(())
// }

// use anyhow::Result;
// use logos::Logos;
// mod front_end;
// mod ir;
// mod wasm;
// mod wasm_runtime;
// use std::io::Write;
// use wasm::module::Module;
// use wasm::opcode::Instruction;
// use wasm::section::{
//     code::{Block, Code},
//     data::{Data, Segment},
//     export::{Export, ExportEntry, ExportType},
//     function::Function,
//     header::Header,
//     import::{Import, ImportEntry, ImportType},
//     memory::{Memory, Page},
//     start::Start,
//     DataType, Section,
//     _type::{FunctionType, Type, ValueType},
// };
//
// fn main() -> Result<()> {
//     let mut module = Module::default();
//
//     module.add_memory(Page::WithNoMinimun(1));
//
//     let export_entry = ExportEntry::new("memory", ExportType::Memory, 0);
//     module.export(export_entry);
//
//     module.import(
//         "core",
//         "write",
//         FunctionType::default()
//             .with_param(ValueType::Data(DataType::I32))
//             .with_result(DataType::I32),
//     );
//
//     let block = Block::default()
//         .with(Instruction::I32Const(0))
//         .with(Instruction::I32Const(8))
//         .with(Instruction::I32Store)
//         .with(Instruction::I32Const(4))
//         .with(Instruction::I32Const(14))
//         .with(Instruction::I32Store)
//         .with(Instruction::I32Const(0))
//         .with(Instruction::Call(0))
//         .with(Instruction::Drop);
//
//     module.add_function(FunctionType::default(), block);
//     let segment = Segment::default()
//         .with_instruction(Instruction::I32Const(8))
//         .with_data("Hello, World!\n".as_bytes().to_vec());
//     module.add_data(segment);
//
//     // TODO: implement
//     // let main_func_id = module.get("main");
//     // module.set_start(main_func_id);
//     module.set_start(1);
//
//     let bytes = module.to_bytes().unwrap();
//     // println!(
//     //     "{:#?}",
//     //     bytes
//     //         .iter()
//     //         .map(|b| format!("{:02x}", b))
//     //         .collect::<Vec<String>>()
//     // );
//
//     std::fs::OpenOptions::new()
//         .write(true)
//         .create(true)
//         .open("test.wasm")
//         .unwrap()
//         .write_all(&bytes)
//         .unwrap();
//
//     wasm_runtime::run(&bytes)?;
//     Ok(())
// }

mod front_end;
mod ir;
mod wasm;
use ir::ExprVisitor;
use logos::Logos;

fn main() {
    let input = r#"
main := 0

max x y
    : Int->Int->Int
    = if x > y then x else y

-- min x y
--     : Int -> Int -> Int
--     = if x < y then x else y

-- enum Option a
--     = Some a
--     | None

-- enum Result a b
--     = OK a
--     | Error b
    "#;

    use std::io::Write;
    let lexer = front_end::Token::lexer(input);
    let mut parser = front_end::Parser::new(lexer.peekable());

    match parser.parse() {
        Ok(ast) => {
            let mut emitter = ir::Emitter::new();
            let ir_module = emitter.visit(&ast);
            let wasm_emitter = wasm::Emitter::new(ir_module);
            let wasm_module = wasm_emitter.emit();
            println!("{:#?}", wasm_module);

            let bytes = wasm_module.to_bytes().unwrap();
            println!(
                "{:#?}",
                bytes
                    .iter()
                    .map(|b| format!("{:02x}", b))
                    .collect::<Vec<String>>()
            );

            std::fs::OpenOptions::new()
                .write(true)
                .create(true)
                .open("test.wasm")
                .unwrap()
                .write_all(&bytes)
                .unwrap();
        }
        Err(errors) => {
            for e in errors {
                eprintln!("Error: {}", e);
            }
        }
    }
}
