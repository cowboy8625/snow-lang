mod args;
mod combinators;
mod error;
mod interpreter;
mod parser;
mod position;
mod repl;
mod scanner;
#[cfg(test)]
mod test;

use crate::combinators::Parser;
use crate::error::{CResult, Error, ErrorKind};
use crate::parser::{Expr, FunctionList};
use crate::position::{Span, Spanned};

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
        match interpreter::evaluation(e, &local, &funcs) {
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
    let (tokens, err) = match scanner::scanner(filename, src) {
        Ok(t) => (t, Vec::new()),
        Err((t, e)) => (t, e),
    };
    let (left_over_tokens, funcs) = match parser::parser().parse(&tokens) {
        Ok((t, f)) => (t, f),
        Err(t) => (t, FunctionList::new()),
    };

    if !left_over_tokens.is_empty() || !err.is_empty() {
        for tok in left_over_tokens.iter() {
            println!("{}", tok);
        }
        for e in err.iter() {
            println!("{}", e);
        }
        return Err(Error::new(
            "unable to lex file",
            (left_over_tokens.first(), left_over_tokens.last()).into(),
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
    check_for_missing_output(&scripts, &output);
    for (script, expected) in scripts.iter().zip(output) {
        let right = from_file(&format!("{}/{}", full_path, script))?;
        let left = std::fs::read_to_string(&format!("{}/{}", full_path, expected))?;
        assert_eq!(right.to_string(), left);
    }
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let filename = std::env::args().nth(1).unwrap_or("--shell".into());
    if filename == "--test" {
        test_scripts()?;
    } else if filename == "--shell" {
        repl::run()?;
    } else {
        from_file(&filename)?;
    }
    Ok(())
}
