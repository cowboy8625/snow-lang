use snowc_lexer::{Scanner, Token};

#[cfg(test)]
use pretty_assertions::assert_eq;

macro_rules! setup_test {
    ($name:ident, $input:expr $(, ($token:ident, $output:expr))* $(,)?) => {
        #[test]
        fn $name() {
            use Token::*;
            let src = $input;
            let mut scanner = Scanner::new(src);
            $(
                let (tok, found) = scanner.next().map(|t| {
                    let span = t.span().range();
                    (t, &src[span])
                }).unwrap();
                let span = tok.span();
                let value = tok.value().to_string();
                assert_eq!($token(value, span), tok, "token vs. token");
                assert_eq!(tok.value(), $output, "captured in token");
                match tok {
                    Token::String(..) => assert_eq!(found, format!("{:?}", $output), "span in source"),
                    _ => assert_eq!(found, $output, "span in source"),
                }
            )*
        }
    };
}

#[test]
fn failed() {
    let mut scanner = Scanner::new("");
    scanner.next();
    scanner.next();
    scanner.next();
    scanner.next();
    scanner.next();
    scanner.next();
    assert_eq!(scanner.next(), None);
}
setup_test!(token_error, "😅", (Error, "😅"));
setup_test!(symbol_scan, "λλλ", (Op, "λ"), (Op, "λ"), (Op, "λ"),);

setup_test!(
    scanner_main,
    r#"main = print "Hello World!";"#,
    (Id, "main"),
    (Op, "="),
    (Id, "print"),
    (String, "Hello World!"),
    (Op, ";"),
);

setup_test!(
    lambdas,
    r#"main = \x -> x;"#,
    (Id, "main"),
    (Op, "="),
    (Op, "\\"),
    (Id, "x"),
    (Op, "->"),
    (Id, "x"),
    (Op, ";")
);

setup_test!(
    lambda_symbol,
    r#"fn add = (λx -> (λy -> x + y));"#,
    (KeyWord, "fn"),
    (Id, "add"),
    (Op, "="),
    (Op, "("),
    (Op, "λ"),
    (Id, "x"),
    (Op, "->"),
    (Op, "("),
    (Op, "λ"),
    (Id, "y"),
    (Op, "->"),
    (Id, "x"),
    (Op, "+"),
    (Id, "y"),
    (Op, ")"),
    (Op, ")"),
    (Op, ";")
);

setup_test!(
    scanner_add_func,
    r#"



add x y = x + y;
main = add 123 321;
"#,
    (Id, "add"),
    (Id, "x"),
    (Id, "y"),
    (Op, "="),
    (Id, "x"),
    (Op, "+"),
    (Id, "y"),
    (Op, ";"),
    (Id, "main"),
    (Op, "="),
    (Id, "add"),
    (Int, "123"),
    (Int, "321"),
    (Op, ";"),
);

// #[test]
// fn check_string() {
//     use Token::*;
//     let src = r#"""#;
//     let output = r#"""#;
//     let mut scanner = Scanner::new(src);
//     let (tok, found) = scanner.next().map(|t| {
//         let span = t.span().range();
//         (t, &src[span])
//     }).unwrap();
//
//     let span = tok.span();
//     let value = tok.value().to_string();
//     assert_eq!(String(value, span), tok, "token vs. token");
//     assert_eq!(tok.value(), output, "captured in token");
//     assert_eq!(found, output, "span in source");
// }

// fn test_tokens(src: &str, scanner: &mut Scanner) {
//     let (tok, found, rspan) = scanner.next().map(|t| {
//         let span = t.span();
//         let range = span.range();
//         (t, &src[range], span)
//     }).unwrap();
//
//     let tok_span = tok.span();
//     let value = tok.value().to_string();
//     eprintln!("{tok:?}, {found:?}, {tok_span:?}, {rspan:?}, {value:?}");
// }
//
// #[test]
// fn check_span() {
//     let src = r#"let main =
//     println "hello";"#;
//     let mut scanner = Scanner::new(src);
//     test_tokens(src, &mut scanner);
//     test_tokens(src, &mut scanner);
//     test_tokens(src, &mut scanner);
//     test_tokens(src, &mut scanner);
//     assert!(false);
// }
