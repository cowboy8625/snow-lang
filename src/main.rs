mod args;
mod error;
mod interpreter;
mod parser;
mod position;
mod repl;
mod scanner;
#[cfg(test)]
mod test;
#[cfg(test)]
mod test_parser;
#[cfg(test)]
mod test_scanner;

use crate::error::{CResult, Error, ErrorKind};
use crate::interpreter::FunctionList;
use crate::position::{Pos, Span, Spanned};
use parser::{Expr, Parser};

fn excute_with_env_of<'a>(src: &str, local: &mut FunctionList, funcs: &'a mut FunctionList) {
    let tokens = scanner::scanner("shell.snow", src).unwrap_or(Vec::new());
    // Try and parse all functions
    let (t, expr): (Vec<_>, Option<_>) = match parser::parser().parse(&tokens) {
        // If all was paresed look into current functions and replace old functions
        // with new def.
        Ok((t, f)) => {
            for (k, v) in f.iter() {
                // Old functions is thrown away if present.
                let _ = funcs.insert(k.to_string(), v.clone());
            }
            (t.to_vec(), None)
        }
        // If failed to parse try and parse Atoms
        Err(t) => match t.first().map(|s| s.node.clone()) {
            Some(scanner::Token::DeDent) => match parser::app().parse(&t[1..]) {
                Ok((t, expr)) => (t.to_vec(), Some(expr.node)),
                Err(t) => (t.to_vec(), None),
            },
            _ => match parser::app().parse(&t) {
                Ok((t, expr)) => (t.to_vec(), Some(expr.node)),
                Err(t) => (t.to_vec(), None),
            },
        },
    };

    if !t.is_empty() {
        eprintln!("{}", ErrorKind::LexeringFailer);
        for s in t.iter() {
            eprintln!("{}", s);
        }
    }
    if let Some(e) = &expr {
        match interpreter::evaluation(e, &[], local, &funcs) {
            Ok(out) => eprintln!("[OUT]: {}", out),
            Err(e) => eprintln!("{}", e),
        }
    }
}

fn from_file(filename: &str) -> CResult<Expr> {
    let src = args::snow_source_file(&filename)?;
    run(filename, &src)
}

fn run(filename: &str, src: &str) -> CResult<Expr> {
    let tokens = scanner::scanner(filename, src)?;
    let (t, mut funcs) = match parser::parser().parse(&tokens) {
        Ok((t, f)) => (t, f),
        Err(t) => (t, FunctionList::new()),
    };

    // let f: Spanned<Expr> = (
    //     Expr::Constant(Atom::String(filename.into())),
    //     Span::default(),
    // )
    //     .into();
    let args: Vec<Spanned<Expr>> = Vec::new();
    // let mut args = if let Some(idx) = std::env::args().into_iter().position(|x| x == filename) {
    //     std::env::args()
    //         .into_iter()
    //         .enumerate()
    //         .rev()
    //         .take_while(|(i, _)| i > &idx)
    //         .map(|(_, i)| (Expr::Constant(Atom::String(i.into())), Span::default()).into())
    //         .collect::<Vec<Spanned<Expr>>>()
    // } else {
    //     Vec::new()
    // };
    // args.insert(0, f);
    let mut local = FunctionList::new();
    match funcs.get_mut("main") {
        Some(func) => {
            let mut idx = 0;
            for (i, arg) in args.iter().enumerate() {
                if func.bind_arg(arg.node.clone(), &mut local) {
                    break;
                }
                idx = i;
            }
            let left_of_args = args[idx..].to_vec();
            func.local(&mut local);
            Ok(interpreter::evaluation(
                &func.body(),
                &left_of_args,
                &mut local,
                &funcs,
            )?)
        }
        _ => {
            dbg!(t);
            dbg!(funcs);
            Err(Error::new(
                "you must provide a 'main' entry point",
                Span::new(Pos::default(), Pos::default(), filename),
                ErrorKind::NoMain,
            ))
        }
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
    check_for_missing_output(&scripts, &output);
    for (script, expected) in scripts.iter().zip(output) {
        let right = from_file(&format!("{}/{}", full_path, script))?;
        let left = std::fs::read_to_string(&format!("{}/{}", full_path, expected))?;
        assert_eq!(right.to_string(), left);
    }
    Ok(())
}

fn run_file(filename: &str) {
    let src = match args::snow_source_file(&filename) {
        Ok(f) => f,
        Err(e) => {
            println!("{}", e);
            std::process::exit(1);
        }
    };
    match run(filename, &src) {
        Ok(_) => {}
        Err(e) => println!("{}", e),
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let filename = std::env::args().nth(1).unwrap_or("--shell".into());
    if filename == "--test" {
        test_scripts()?;
    } else if filename == "--shell" {
        repl::run()?;
    } else if filename != "--shell" && filename != "--help" {
        run_file(&filename);
    } else {
        println!("snowc [version 0.0.0]");
        println!("<file>            run <file>");
        println!("--shell           repl - default");
        println!("--test            run custom test");
        println!("--help            this message");
    }
    Ok(())
}
