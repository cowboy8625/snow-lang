use snowc_lexer::{LexerDebug, Scanner, Token};

#[cfg(test)]
use pretty_assertions::assert_eq;

macro_rules! setup_test {
    ($name:ident, $input:expr $(, ($token:ident, $output:expr))* $(,)?) => {
        #[test]
        fn $name() {
            use Token::*;
            let src = $input;
            let mut scanner = Scanner::new(src, LexerDebug::Off);
            $(
                let (tok, found) = scanner.next().map(|t| {
                    let span = t.span().range();
                    (t, &src[span])
                }).unwrap();

                let span = tok.span();
                let value = tok.value().to_string();
                assert_eq!($token(value, span), tok, "token vs. token");
                assert_eq!(tok.value(), $output, "captured in token");
                assert_eq!(found, $output, "span in source");
            )*
        }
    };
}

setup_test!(token_error, "😅", (Error, "😅"));
setup_test!(symbol_scan, "λλλ", (Op, "λ"), (Op, "λ"), (Op, "λ"),);

setup_test!(
    scanner_main,
    r#"fn main() {
    return 0;
}
"#,
    (KeyWord, "fn"),
    (Id, "main"),
    (Op, "("),
    (Op, ")"),
    (Op, "{"),
    (KeyWord, "return"),
    (Int, "0"),
    (Op, ";"),
    (Op, "}")
);
setup_test!(
    lambdas,
    r#"\x -> x;"#,
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
    r#"fn add(x u64, y u64) u64 {
    x + y
}
fn main() {
    let x = add(123, 321);
}
"#,
    (KeyWord, "fn"),
    (Id, "add"),
    (Op, "("),
    (Id, "x"),
    (Id, "u64"),
    (Op, ","),
    (Id, "y"),
    (Id, "u64"),
    (Op, ")"),
    (Id, "u64"),
    (Op, "{"),
    (Id, "x"),
    (Op, "+"),
    (Id, "y"),
    (Op, "}"),
    (KeyWord, "fn"),
    (Id, "main"),
    (Op, "("),
    (Op, ")"),
    (Op, "{"),
    (KeyWord, "let"),
    (Id, "x"),
    (Op, "="),
    (Id, "add"),
    (Op, "("),
    (Int, "123"),
    (Op, ","),
    (Int, "321"),
    (Op, ")"),
    (Op, ";"),
    (Op, "}")
);
