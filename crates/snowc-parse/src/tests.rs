use super::{parser::Parser, precedence::Precedence, Scanner};
use pretty_assertions::assert_eq;
use snowc_errors::CResult;

// macro_rules! setup_test {
//     ($name:ident $(, $input:expr, $output:expr)* $(,)?) => {
//         #[test]
//         fn $name() -> CResult<()> {
//             $(
//                 let s = parse($input)?;
//                 dbg!(&s);
//                 for (i, o) in s.iter().zip($output) {
//                     assert_eq!(i.to_string(), o);
//                 }
//             ) *
//                 Ok(())
//         }
//     };
// }

#[test]
fn expression() -> CResult<()> {
    let lexer = Scanner::new("1").peekable();
    let mut parser = Parser::new(lexer);
    let left = parser.expression(Precedence::None)?.to_string();
    assert_eq!(left, "1");

    let lexer = Scanner::new("1.2").peekable();
    let mut parser = Parser::new(lexer);
    let left = parser.expression(Precedence::None)?.to_string();
    assert_eq!(left, "1.2");

    let lexer = Scanner::new("a").peekable();
    let mut parser = Parser::new(lexer);
    let left = parser.expression(Precedence::None)?.to_string();
    assert_eq!(left, "a");

    Ok(())
}
#[test]
fn unary() -> CResult<()> {
    let lexer = Scanner::new("-1").peekable();
    let mut parser = Parser::new(lexer);
    let left = parser.expression(Precedence::None)?.to_string();
    assert_eq!(left, "(- 1)");

    // let lexer = Scanner::new("--1").peekable();
    // let mut parser = Parser::new(lexer);
    // let left = parser.expression(Precedence::None)?.to_string();
    // assert_eq!(left, "(- (- 1))");

    let lexer = Scanner::new("(- 1.2)").peekable();
    let mut parser = Parser::new(lexer);
    let left = parser.expression(Precedence::None)?.to_string();
    assert_eq!(left, "(- 1.2)");

    let lexer = Scanner::new("-a").peekable();
    let mut parser = Parser::new(lexer);
    let left = parser.expression(Precedence::None)?.to_string();
    assert_eq!(left, "(- a)");

    Ok(())
}

#[test]
fn binary() -> CResult<()> {
    let lexer = Scanner::new("1 + 2 * 3").peekable();
    let mut parser = Parser::new(lexer);
    let left = parser.expression(Precedence::None)?.to_string();
    assert_eq!(left, "(+ 1 (* 2 3))");
    Ok(())
}

#[test]
fn binary_ids() -> CResult<()> {
    let lexer = Scanner::new("a + b * c * d + e").peekable();
    let mut parser = Parser::new(lexer);
    let left = parser.expression(Precedence::None)?.to_string();
    assert_eq!(left, "(+ a (+ (* b (* c d)) e))");

    let lexer = Scanner::new("a + b").peekable();
    let mut parser = Parser::new(lexer);
    let left = parser.expression(Precedence::None)?.to_string();
    assert_eq!(left, "(+ a b)");
    Ok(())
}

#[test]
fn changing_precedence() -> CResult<()> {
    let lexer = Scanner::new("(-1 + 2) * 3 - -4").peekable();
    let mut parser = Parser::new(lexer);
    let left = parser.expression(Precedence::None)?.to_string();
    assert_eq!(left, "(- (* (+ (- 1) 2) 3) (- 4))");

    let lexer = Scanner::new("(((a)))").peekable();
    let mut parser = Parser::new(lexer);
    let left = parser.expression(Precedence::None)?.to_string();
    assert_eq!(left, "a");
    Ok(())
}

#[test]
fn calling_operator() {
    let lexer = Scanner::new("(+) 1 2;").peekable();
    let mut parser = Parser::new(lexer);
    match parser.parse(true) {
        Ok(e) => {
            let mut e = e.iter();
            assert_eq!(
                e.next()
                    .map(ToString::to_string)
                    .unwrap_or("NONE".to_string()),
                "<(+): (1, 2)>"
            );
        }
        Err(e) => {
            dbg!(e);
            assert!(false);
        }
    }
}

#[test]
fn call() -> CResult<()> {
    let lexer = Scanner::new("add 1 2").peekable();
    let mut parser = Parser::new(lexer);
    let left = parser.call(Precedence::None)?.to_string();
    assert_eq!(left, "<add: (1, 2)>");
    Ok(())
}

#[test]
fn pipe_call() -> CResult<()> {
    let lexer = Scanner::new("2 |> add 1").peekable();
    let mut parser = Parser::new(lexer);
    let left = parser.call(Precedence::None)?.to_string();
    assert_eq!(left, "(|> 2 <add: (1)>)");
    Ok(())
}

#[test]
fn conditional() -> CResult<()> {
    let lexer = Scanner::new("if x > y then x else y").peekable();
    let mut parser = Parser::new(lexer);
    let left = parser.conditional()?.to_string();
    assert_eq!(left, "(if ((> x y)) then x else y)");
    Ok(())
}

#[test]
fn function_def() -> CResult<()> {
    let lexer = Scanner::new("fn add x y = x + y;").peekable();
    let mut p = Parser::new(lexer);
    p.lexer.next();
    let left = p.function(0..0)?.to_string();
    assert_eq!(left, "<add: x -> y -> (+ x y)>");
    Ok(())
}

#[test]
fn super_duper_function_def() {
    let src = "fn main = print (max ((add 1 2) + (sub 1 2)) 20);";
    let result = "<main: <print: (<max: ((+ <add: (1, 2)> <sub: (1, 2)>), 20)>)>>";
    let lexer = Scanner::new(src).peekable();
    let mut p = Parser::new(lexer);
    let exprs = p.parse(false).unwrap();
    let mut exprs = exprs.iter();
    assert_eq!(
        exprs
            .next()
            .map(|e| e.to_string())
            .unwrap_or("FAILED".to_string()),
        result
    );
}

#[test]
fn multi_function_def() {
    let lexer = Scanner::new("fn add x y = x + y; fn sub x y = x - y;").peekable();
    let mut parser = Parser::new(lexer);
    let exprs = parser.parse(false).unwrap_or(vec![]);
    let mut e = exprs.iter();
    assert_eq!(e.next().unwrap().to_string(), "<add: x -> y -> (+ x y)>");
    assert_eq!(e.next().unwrap().to_string(), "<sub: x -> y -> (- x y)>");
}

#[test]
fn closures() {
    let lexer = Scanner::new("fn add = (λx -> (λy -> x + y));").peekable();
    let mut parser = Parser::new(lexer);
    match parser.parse(false) {
        Ok(e) => {
            let mut e = e.iter();
            assert_eq!(
                e.next()
                    .map(ToString::to_string)
                    .unwrap_or("NONE".to_string()),
                "<add: x -> y -> (+ x y)>"
            );
        }
        Err(e) => {
            dbg!(e);
            assert!(false);
        }
    }
}

#[test]
fn user_type_def() -> CResult<()> {
    let lexer = Scanner::new("type Option = Some Int | None;").peekable();
    let mut parser = Parser::new(lexer);
    let exprs = parser.parse(false)?;
    let mut e = exprs.iter();
    assert_eq!(
        e.next().unwrap().to_string(),
        "<Option: (Some, [Int]), (None, [])>"
    );

    Ok(())
}

#[test]
fn type_dec() -> CResult<()> {
    let lexer = Scanner::new("add :: Int -> Int -> Int;").peekable();
    let mut parser = Parser::new(lexer);
    let exprs = parser.parse(false)?;
    let mut e = exprs.iter();
    assert_eq!(e.next().unwrap().to_string(), "<add :: Int -> Int -> Int>");

    Ok(())
}
