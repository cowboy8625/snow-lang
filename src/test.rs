use super::parser::Atom;
use super::*;
#[cfg(test)]
use pretty_assertions::assert_eq;
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
    println (add 1 (- 100 1))
";
    assert_eq!(from_string(src)?, Expr::Constant(Atom::Int(100)));
    Ok(())
}

#[test]
fn test_do_block_const() -> CResult<()> {
    let src = "
add x y = + x y
main = do
    3
";
    assert_eq!(from_string(src)?, Expr::Constant(Atom::Int(3)));
    Ok(())
}

#[test]
fn test_let_expr_one() -> CResult<()> {
    let src = "
main = let z = 99 in z
";
    eprintln!("{}", src);
    assert_eq!(from_string(src)?, Expr::Constant(Atom::Int(99)));
    Ok(())
}

#[test]
fn test_let_expr_two() -> CResult<()> {
    let src = "
main = let z = 99, y = 1 in + z y
";
    eprintln!("{}", src);
    assert_eq!(from_string(src)?, Expr::Constant(Atom::Int(100)));
    Ok(())
}

#[test]
fn test_let_expr_multi_line() -> CResult<()> {
    let src = "
add x y =
    let a = x
    , b = y
    in + a b

main = add 1 2
";
    eprintln!("{}", src);
    assert_eq!(from_string(src)?, Expr::Constant(Atom::Int(3)));
    Ok(())
}

#[test]
fn test_let_expr_multi_in_new_line() -> CResult<()> {
    let src = "
add x y =
    let a = x
    , b = y
    ,
    c = y, d = x
    in
    - (+ (+ a b) c) d

main = add 1 2
";
    eprintln!("{}", src);
    assert_eq!(from_string(src)?, Expr::Constant(Atom::Int(4)));
    Ok(())
}

#[test]
fn test_let_binding_multi_do_expr() -> CResult<()> {
    let src = "
add x y = do
    let a = x
    , b = y
    , z = 1
    + a b z

main = add 1 2
";
    eprintln!("{}", src);
    assert_eq!(from_string(src)?, Expr::Constant(Atom::Int(4)));
    Ok(())
}

#[test]
fn test_do_do() -> CResult<()> {
    let src = "
add x y = do
            do
                + x y

main = add 1 3
";
    eprintln!("{}", src);
    assert_eq!(from_string(src)?, Expr::Constant(Atom::Int(4)));
    Ok(())
}

    Ok(())
}
