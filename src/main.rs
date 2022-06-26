mod args;
mod code_gen;
mod error;
mod function;
mod parser;
mod position;
mod scanner;
#[cfg(test)]
mod test_scanner;

use std::fs::OpenOptions;
use std::io::Write;

use crate::error::Result;
use crate::function::FunctionList;
use error::Error;
use parser::Parser;
use position::Span;

fn pop_extesion<'a>(filename: &'a str) -> &'a str {
    filename.split('.').collect::<Vec<&'a str>>()[0]
}

fn haskell_code_gen(filename: &str, src: &str, show_tokens: bool, show_ast: bool) -> Result<()> {
    let tokens = scanner::scanner(filename, &src)?;
    if show_tokens {
        dbg!(&tokens);
    }
    let (_t, funcs) = match parser::parser().parse(&tokens) {
        Ok((t, f)) => (t, f),
        Err(t) => (t, FunctionList::new()),
    };

    if show_ast {
        dbg!(&funcs);
    }

    let hcode = code_gen::haskell_code_gen(funcs, filename);
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(format!("{}.hs", pop_extesion(filename)))
        .map_err(|e| Error {
            last: None,
            msg: e.to_string(),
            span: Span::default(),
        })?;
    file.write_all(hcode.as_bytes()).map_err(|e| Error {
        last: None,
        msg: e.to_string(),
        span: Span::default(),
    })?;
    Ok(())
}

fn report_error_to_user(src: &str, error: &Error) {
    if let Some(error) = &error.last {
        report_error_to_user(src, &error);
    }

    println!("{}", error.span);
    let start = &src[error.span.to_range()]
        .chars()
        .rev()
        .enumerate()
        .find(|(_, i)| i == &'\n')
        .map(|(i, _)| i)
        .unwrap_or(error.span.start.idx);
    let end = *&src[error.span.to_range()]
        .chars()
        .enumerate()
        .find(|(_, i)| i == &'\n')
        .map(|(i, _)| error.span.start.idx + i)
        .unwrap_or(src.len().saturating_sub(1));
    let source_line = format!(
        "{}{}{}",
        &src[(error.span.start.idx - start)..error.span.start.idx],
        &src[error.span.to_range()],
        &src[error.span.start.idx + 1..end],
    );
    println!("|   {}", source_line);
    let spacing = &src[(error.span.start.idx - start)..error.span.start.idx]
        .chars()
        .map(|_| ' ')
        .collect::<String>();
    println!("|   {}^", spacing);
    println!("|   {}{}", spacing, error.msg);
    let line = (0..source_line.len()).map(|_| '-').collect::<String>();
    println!("|---{}", line);
}

fn main() -> Result<()> {
    let settings = args::cargs();
    for filename in settings.filenames.iter() {
        let src = args::snow_source_file(filename)?;
        match haskell_code_gen(filename, &src, settings.tokens, settings.ast) {
            Err(e) => report_error_to_user(&src, &e),
            _ => {}
        }
    }
    Ok(())
}
