use crate::combinators::Parser;

mod combinators;
mod interpreter;
mod parser;
mod position;
mod scanner;

fn main() -> () {
    let src = "
add x y = + 2 2
add1 y = add y 1
main = print (+ 1 (add1 100))
";

    let filename = "s.snow";
    println!("{}", src);
    let tokens = scanner::scanner(filename, src);
    for tok in tokens.iter() {
        println!("{}", tok);
    }
    match parser::parser().parse(&tokens) {
        Ok((i, r)) => {
            println!("Wooooo");
            dbg!(r);
            // for (name, func) in r.iter() {
            //     println!("name: {}, func: {}", name, func);
            // }
            for t in i.iter() {
                println!("{}", t);
            }
        }
        Err(e) => {
            println!("ERROR");
            for t in e.iter() {
                println!("{}", t);
            }
        }
    };
}
