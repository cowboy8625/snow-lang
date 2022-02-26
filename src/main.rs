mod args;
mod combinators;
mod error;
mod interpreter;
mod parser;
mod position;
mod scanner;

use crate::combinators::Parser;
use crate::interpreter::EvalResult;
use crate::parser::{Expr, FunctionList};

fn run(filename: &str) -> EvalResult<()> {
    let src = args::snow_source_file(&filename)?;
    let tokens = scanner::scanner(filename, &src).expect("Failed to Scan in run");
    let (t, funcs) = match parser::parser().parse(&tokens) {
        Ok((t, f)) => (t, f),
        Err(t) => (t, FunctionList::new()),
    };

    assert!(t.len() == 0);
    match &funcs.get("main").expect("Failed to get main in run").node {
        Expr::Lambda(_, _, body) => {
            let output = interpreter::evaluation(&body.node, &FunctionList::new(), &funcs)?;
            println!("Return from main: {}", output);
        }
        _ => {
            println!("No 'main' entry provided.");
        }
    };
    Ok(())
}

fn test_scripts() -> EvalResult<()> {
    let full_path = "/home/cowboy/Documents/Rust/languages/snow/example_scripts";
    let files = std::fs::read_dir(full_path)?;
    let (scripts, _output): (Vec<_>, Vec<_>) = files
        .map(|f| {
            f.unwrap()
                .file_name()
                .into_string()
                .expect("Failed to unwrap OsString in Test Scripts")
        })
        .partition(|f| f.ends_with(".snow"));
    dbg!(&scripts, _output);
    for script in scripts.iter() {
        run(&format!("{}/{}", full_path, script))?;
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let filename = std::env::args().nth(1).unwrap();
    if filename == "--test" {
        println!("Test");
        test_scripts()?;
    } else {
        println!("Run");
        run(&filename)?;
    }
    Ok(())
}

#[test]
fn test_main() -> EvalResult<()> {
    use crate::parser::Atom;
    let src = "
add x y = + x y

main = print (add 1 2)
";
    let tokens = scanner::scanner("test.snow", src).expect("Failed to Scan in test");
    let (tokens, funcs) = parser::parser()
        .parse(&tokens)
        .expect("Failed to Parse in test");
    match &funcs.get("main").expect("Failed to get main in test").node {
        Expr::Lambda(_, _, body) => {
            let left = interpreter::evaluation(&body.node, &FunctionList::new(), &funcs)?;
            assert_eq!(left, Expr::Constant(Atom::Int(3)));
            assert_eq!(tokens, vec![]);
        }
        _ => assert!(false),
    };
    Ok(())
}
