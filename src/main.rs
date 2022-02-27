mod args;
mod combinators;
mod error;
mod interpreter;
mod parser;
mod position;
mod scanner;
#[cfg(test)]
mod test;

use crate::combinators::Parser;
use crate::error::{CResult, Error, ErrorKind};
use crate::parser::{Expr, FunctionList};
use crate::position::{Span, Spanned};

fn from_file(filename: &str) -> CResult<Expr> {
    let src = args::snow_source_file(&filename)?;
    run(filename, &src)
}

fn run(filename: &str, src: &str) -> CResult<Expr> {
    let tokens = scanner::scanner(filename, src).unwrap();
    let (t, funcs) = match parser::parser().parse(&tokens) {
        Ok((t, f)) => (t, f),
        Err(t) => (t, FunctionList::new()),
    };

    if t.len() != 0 {
        return Err(Error::new(
            "unable to lex file",
            (t.first(), t.last()).into(),
            ErrorKind::LexeringFailer,
        ));
    }
    match &funcs.get("main") {
        Some(Spanned {
            node: Expr::Lambda(_, _, body),
            ..
        }) => Ok(interpreter::evaluation(
            &body.node,
            &FunctionList::new(),
            &funcs,
        )?),
        _ => Err(Error::new(
            "you must provide a 'main' entry point",
            Span::default(),
            ErrorKind::NoMain,
        )),
    }
}

fn check_for_missing_output(scripts: &[String], output: &[String]) {
    if scripts.len() != output.len() {
        let max = scripts.iter().map(|x| x.len()).max().unwrap_or(0);
        for script in scripts.iter() {
            let outfile = format!("{}.out", script.split(".").nth(0).unwrap_or(""));
            let right = format!(" {}", outfile);
            let left = format!("❌ ❄ {} is missing", script);
            let space = left.len() + max - script.len();
            if !output.contains(&outfile) {
                eprintln!("{:<space$}{}", left, right);
            }
        }
        std::process::exit(60);
    }
}

fn test_scripts() -> CResult<()> {
    let full_path = "/home/cowboy/Documents/Rust/languages/snow/example_scripts";
    let files = std::fs::read_dir(full_path)?;
    let (scripts, output): (Vec<_>, Vec<_>) = files
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
