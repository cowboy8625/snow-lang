use super::parser::Atom;
use super::*;
fn from_string(src: &str) -> CResult<Expr> {
    run("testing.snow", src)
}

#[test]
fn test_no_main() {
    let src = "add x y = + x y";
    let result = from_string(src)
        .err()
        .map(|c| c.downcast::<crate::error::Error>().ok())
        .flatten()
        .map(|e| e.kind());
    assert_eq!(result, Some(ErrorKind::NoMain));
}
#[test]
fn test_add_function() -> CResult<()> {
    let src = "
add x y = + x y
main = print (add 1 2)
";
    assert_eq!(from_string(src)?, Expr::Constant(Atom::Int(3)));
    Ok(())
}

#[test]
fn test_passing_function_as_arg() -> CResult<()> {
    let src = "
add x y = + x y
apply a b c = c a b
main = print (apply 1 2 add)
";
    assert_eq!(from_string(src)?, Expr::Constant(Atom::Int(3)));
    Ok(())
}

#[test]
fn test_passing_app_and_func_as_arg() -> CResult<()> {
    let src = "
add x y = + x y
apply a b c = c a b
main = println (apply (println (- 20 10)) (println (+ 1 9)) add)
";
    assert_eq!(from_string(src)?, Expr::Constant(Atom::Int(20)));
    Ok(())
}

#[test]
fn test_line_comment_before_and_after_func_dec() -> CResult<()> {
    let src = "
-- line comment before func dec check
add x y = + x y
-- line comment after func dec check
apply a b c = c a b
main = println (apply (println (- 20 10)) (println (+ 1 9)) add)
";
    assert_eq!(from_string(src)?, Expr::Constant(Atom::Int(20)));
    Ok(())
}

#[test]
fn test_block_comment_before_and_after_func_dec() -> CResult<()> {
    let src = "
{- line comment before func dec check 
-}
add x y = + x y
{- line comment after func dec check
-}
apply a b c = c a b
main = println (apply (println (- 20 10)) (println (+ 1 9)) add)
";
    assert_eq!(from_string(src)?, Expr::Constant(Atom::Int(20)));
    Ok(())
}

#[test]
fn test_do_block_app() -> CResult<()> {
    let src = "
add x y = + x y
main = do
    println 2
    println (add 1 (- 100 1))
";
    assert_eq!(from_string(src)?, Expr::Constant(Atom::Int(100)));
    Ok(())
}

// TODO: Update this test when custom parser errors are implemented
// #[test]
// fn test_do_block_empty_return() { let src = "
// add x y = + x y
// main = do
//
// ";
//     let result = from_string(src)
//         .err()
//         .map(|c| c.downcast::<crate::error::Error>().ok())
//         .flatten()
//         .map(|e| e.kind());
//     assert_eq!(result, Some(ErrorKind::EmptyReturn));
// }

#[test]
fn test_do_block_const() -> CResult<()> {
    let src = "
add x y = + x y
main = do
    1
    2
    3
";
    assert_eq!(from_string(src)?, Expr::Constant(Atom::Int(3)));
    Ok(())
}

// TODO: for nested do blocks to work we need a way to
// dynamically check indents.  Currently is set to 4.
// #[test]
// fn test_nested_do_block() -> CResult<()> {
//     let src = "
// add x y = + x y
// main = do
//     println 4
//     do
//         println 8
//     println (add 1 (- 100 1))
// ";
//     assert_eq!(from_string(src)?, Expr::Constant(Atom::Int(3)));
//     Ok(())
// }
