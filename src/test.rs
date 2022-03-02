use super::parser::Atom;
use super::*;
#[cfg(test)]
use pretty_assertions::assert_eq;
fn from_string(src: &str) -> CResult<Expr> {
    run("testing.snow", src)
}

#[test]
fn test_plus_int() -> CResult<()> {
    let src = "main = + 321 123 444";
    assert_eq!(from_string(src)?, Expr::Constant(Atom::Int(888)));
    Ok(())
}

#[test]
fn test_plus_float() -> CResult<()> {
    let src = "main = + 3.1 1.3";
    assert_eq!(from_string(src)?, Expr::Constant(Atom::Float(4.3999996)));
    Ok(())
}

#[test]
fn test_sub_int() -> CResult<()> {
    let src = "main = - 321 21";
    assert_eq!(from_string(src)?, Expr::Constant(Atom::Int(300)));
    Ok(())
}

#[test]
fn test_sub_float() -> CResult<()> {
    let src = "main = - 3.1 2.1";
    assert_eq!(from_string(src)?, Expr::Constant(Atom::Float(1.)));
    Ok(())
}

#[test]
fn test_mul_int() -> CResult<()> {
    let src = "main = * 321 123";
    assert_eq!(from_string(src)?, Expr::Constant(Atom::Int(39483)));
    Ok(())
}

#[test]
fn test_mul_float() -> CResult<()> {
    let src = "main = * 3.0 (- 0.0 1.0)";
    assert_eq!(from_string(src)?, Expr::Constant(Atom::Float(-3.0)));
    Ok(())
}

#[test]
fn test_div_int() -> CResult<()> {
    let src = "main = / 322 2";
    assert_eq!(from_string(src)?, Expr::Constant(Atom::Int(161)));
    Ok(())
}

#[test]
fn test_div_float() -> CResult<()> {
    let src = "main = / 32.0 2.0";
    assert_eq!(from_string(src)?, Expr::Constant(Atom::Float(16.0)));
    Ok(())
}

#[test]
fn test_eq_int() -> CResult<()> {
    let src = "main = == 322 322";
    assert_eq!(from_string(src)?, Expr::Constant(Atom::Boolean(true)));
    let src = "main = == 3 322";
    assert_eq!(from_string(src)?, Expr::Constant(Atom::Boolean(false)));
    Ok(())
}

#[test]
fn test_eq_float() -> CResult<()> {
    let src = "main = == 3.0 3.0";
    assert_eq!(from_string(src)?, Expr::Constant(Atom::Boolean(true)));
    let src = "main = == 3.0 1.0";
    assert_eq!(from_string(src)?, Expr::Constant(Atom::Boolean(false)));
    Ok(())
}

#[test]
fn test_not_eq_int() -> CResult<()> {
    let src = "main = != 322 322";
    assert_eq!(from_string(src)?, Expr::Constant(Atom::Boolean(false)));
    let src = "main = != 3 322";
    assert_eq!(from_string(src)?, Expr::Constant(Atom::Boolean(true)));
    Ok(())
}

#[test]
fn test_not_eq_float() -> CResult<()> {
    let src = "main = != 3.0 3.0";
    assert_eq!(from_string(src)?, Expr::Constant(Atom::Boolean(false)));
    let src = "main = != 3.0 1.0";
    assert_eq!(from_string(src)?, Expr::Constant(Atom::Boolean(true)));
    Ok(())
}

#[test]
fn test_not() -> CResult<()> {
    let src = "main = !True";
    assert_eq!(from_string(src)?, Expr::Constant(Atom::Boolean(false)));
    let src = "main = !False";
    assert_eq!(from_string(src)?, Expr::Constant(Atom::Boolean(true)));
    Ok(())
}

#[test]
fn test_simple_main() -> CResult<()> {
    let src = "main = let x y = + 1 y in x 1";
    assert_eq!(from_string(src)?, Expr::Constant(Atom::Int(2)));
    Ok(())
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
main =
    do
        println (add 1 (- 100 1))
";
    assert_eq!(from_string(src)?, Expr::Constant(Atom::Int(100)));
    Ok(())
}

#[test]
fn test_do_block_const() -> CResult<()> {
    let src = "
add x y = + x y
main =
    do
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
main = let z = 99, y = 1 in + (z) (y)
";
    eprintln!("{}", src);
    assert_eq!(from_string(src)?, Expr::Constant(Atom::Int(100)));
    Ok(())
}

#[test]
fn test_let_expr_multi_line() -> CResult<()> {
    let src = "
add x y = let a = 1, b = 1 in + (a) (b)

main = add 1 2
";
    eprintln!("{}", src);
    assert_eq!(from_string(src)?, Expr::Constant(Atom::Int(2)));
    Ok(())
}

#[test]
fn test_let_expr_multi_in_new_line() -> CResult<()> {
    let src = "
add x y =
    let
        a = 4,
        b = 3,
        c = 2,
        d = 1,
    in
    - (+ (+ (a) (b)) (c)) (d)

main = add 1 2
";
    eprintln!("{}", src);
    assert_eq!(from_string(src)?, Expr::Constant(Atom::Int(8)));
    Ok(())
}

#[test]
fn test_let_binding_multi_do_expr() -> CResult<()> {
    let src = "
add x y =
    do
        let a = 1
        , b = 3
        , z = 1
        in
        + (+ (a) (b)) (z)

main = add 1 2
";
    eprintln!("{}", src);
    assert_eq!(from_string(src)?, Expr::Constant(Atom::Int(5)));
    Ok(())
}

#[test]
fn test_do_do() -> CResult<()> {
    let src = "
add x y =
    do
        do
            + x y

main = add 1 3
";
    eprintln!("{}", src);
    assert_eq!(from_string(src)?, Expr::Constant(Atom::Int(4)));
    Ok(())
}

#[test]
fn test_do_if() -> CResult<()> {
    let src = r#"
main =
    do
        if True then
            println "If"
        else if False then
            println "Else If"
        else
            println "Else"

"#;
    eprintln!("{}", src);
    assert_eq!(from_string(src)?, Expr::Constant(Atom::String("If".into())));
    Ok(())
}

#[test]
fn test_if_expr() -> CResult<()> {
    let src = "
main = if True then 100
        ";
    eprintln!("{}", src);
    assert_eq!(from_string(src)?, Expr::Constant(Atom::Int(100)));
    Ok(())
}

#[test]
fn test_no_return() {
    let src = "main = if False then 100 ";
    let result = from_string(src)
        .err()
        .map(|c| c.downcast::<crate::error::Error>().ok())
        .flatten()
        .map(|e| e.kind());
    assert_eq!(result, Some(ErrorKind::EmptyReturn));
}
