use super::{haskell_code_gen, parser, scanner, CResult, FunctionList, Parser};

#[cfg(test)]
use pretty_assertions::assert_eq;

const FILENAME: &str = "code_gen_test.snow";

fn code_gen_from_string(src: &str) -> CResult<String> {
    let tokens = scanner::scanner(FILENAME, src)?;
    dbg!(&tokens);
    let (_, funcs) = match parser::parser().parse(&tokens) {
        Ok((t, f)) => (t, f),
        Err(t) => (t, FunctionList::new()),
    };
    Ok(haskell_code_gen(funcs, FILENAME))
}

#[test]
fn hello_world() -> CResult<()> {
    let src = r#"main = println "Hello World""#;
    let left = code_gen_from_string(src)?;
    let right = r#"main = putStr "Hello World" "#;
    assert_eq!(left, right);
    Ok(())
}

#[test]
fn hello_world_with_do_block() -> CResult<()> {
    let src = r#"main = do
    println "Hello World"
"#;
    let left = code_gen_from_string(src)?;
    let right = r#"main = do
    putStr "Hello World" 
"#;
    assert_eq!(left, right);
    Ok(())
}

#[test]
fn hello_world_with_do_block_x_2() -> CResult<()> {
    let src = r#"main = do
    println "Hello World"
    println "Hello World"
"#;
    let left = code_gen_from_string(src)?;
    let right = r#"main = do
    putStr "Hello World" 
    putStr "Hello World" 
"#;
    assert_eq!(left, right);
    Ok(())
}
