#![allow(dead_code)]
#![allow(warnings)]

use logos::Logos;
mod front_end;
mod wasm;
// mod ir;
// mod ir_emitter;

fn main() {
    let input = r#"
fn max x y
    : Int -> Int -> Int
    = if x > y then x else y

fn min x y
    : Int -> Int -> Int
    = if x < y then x else y

enum Option a
    = Some a
    | None

enum Result a b
    = OK a
    | Error b
    "#;

    let lexer = front_end::Token::lexer(input);
    let mut parser = front_end::Parser::new(lexer.peekable());

    match parser.parse() {
        Ok(ast) => {
            println!("{:#?}", ast.len());
            for expr in ast {
                println!("{:#?}", expr);
            }
            // let mut emitter = ir_emitter::IrEmitter::new();
            // let ir = emitter.visit(&ast);
            // println!("{:#?}", ir);
        }
        Err(errors) => {
            for e in errors {
                eprintln!("Error: {}", e);
            }
        }
    }
}
