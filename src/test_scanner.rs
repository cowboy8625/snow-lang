use super::scanner;
#[cfg(test)]
use pretty_assertions::assert_eq;
use scanner::KeyWord::*;
use scanner::Token::{self, *};

const FILENAME: &str = "scan_test.snow";
#[test]
fn scanner_fn_token() {
    use scanner::Token::{self, *};
    let src = "foo x = x
main = foo 1
";
    let tokens = match scanner::scanner(FILENAME, src) {
        Ok(t) => t,
        Err(e) => {
            eprintln!("{}", e);
            Vec::new()
        }
    };
    let left = tokens
        .iter()
        .map(|s| s.node.clone())
        .collect::<Vec<Token>>();
    let right = vec![
        Fn("foo".into()),
        Id("x".into()),
        Op("="),
        Id("x".into()),
        Fn("main".into()),
        Op("="),
        Id("foo".into()),
        Int("1".into()),
        DeDent,
    ];
    assert_eq!(left, right);
}

#[test]
fn scanner_let_block_one_line() {
    let src = "
main = let a = 10, b = 12 in a + b
";
    let tokens = match scanner::scanner(FILENAME, src) {
        Ok(t) => t,
        Err(e) => {
            eprintln!("{}", e);
            Vec::new()
        }
    };
    let left = tokens
        .iter()
        .map(|s| s.node.clone())
        .collect::<Vec<Token>>();
    let right = vec![
        Fn("main".into()),
        Op("="),
        KeyWord(Let),
        Id("a".into()),
        Op("="),
        Int("10".into()),
        Ctrl(','),
        Id("b".into()),
        Op("="),
        Int("12".into()),
        KeyWord(In),
        Id("a".into()),
        Op("+"),
        Id("b".into()),
        DeDent,
    ];
    assert_eq!(left, right);
}

#[test]
fn scanner_let_block_multi_lines() {
    let src = "main =
    let a = 10
    ,   b = 12
    in  a + b
    ";
    let tokens = match scanner::scanner(FILENAME, src) {
        Ok(t) => t,
        Err(e) => {
            eprintln!("{}", e);
            Vec::new()
        }
    };
    let left = tokens
        .iter()
        .map(|s| s.node.clone())
        .collect::<Vec<Token>>();
    let right = vec![
        Fn("main".into()),
        Op("="),
        InDent,
        KeyWord(Let),
        Id("a".into()),
        Op("="),
        Int("10".into()),
        Ctrl(','),
        Id("b".into()),
        Op("="),
        Int("12".into()),
        KeyWord(In),
        Id("a".into()),
        Op("+"),
        Id("b".into()),
        DeDent,
    ];
    assert_eq!(left, right);
}

#[test]
fn scanner_let_block_single_line() {
    let src = "
add x y = let a = x, b = y in + a b
main x y = let a = x, b = y in + a b
";
    let tokens = match scanner::scanner(FILENAME, src) {
        Ok(t) => t,
        Err(e) => {
            eprintln!("{}", e);
            Vec::new()
        }
    };
    let right = vec![
        Fn("add".into()),
        Id("x".into()),
        Id("y".into()),
        Op("="),
        KeyWord(Let),
        Id("a".into()),
        Op("="),
        Id("x".into()),
        Ctrl(','),
        Id("b".into()),
        Op("="),
        Id("y".into()),
        KeyWord(In),
        Op("+"),
        Id("a".into()),
        Id("b".into()),
        Fn("main".into()),
        Id("x".into()),
        Id("y".into()),
        Op("="),
        KeyWord(Let),
        Id("a".into()),
        Op("="),
        Id("x".into()),
        Ctrl(','),
        Id("b".into()),
        Op("="),
        Id("y".into()),
        KeyWord(In),
        Op("+"),
        Id("a".into()),
        Id("b".into()),
        DeDent,
    ];
    let left = tokens
        .iter()
        .map(|s| s.node.clone())
        .collect::<Vec<Token>>();
    assert_eq!(left, right);
}

#[test]
fn scanner_if_else_if_else() {
    let src = r#"
main = if True then println "If" else if False then println "Else If" else println "Else""#;
    let tokens = match scanner::scanner(FILENAME, src) {
        Ok(t) => t,
        Err(e) => {
            eprintln!("{}", e);
            Vec::new()
        }
    };
    let right = vec![
        Fn("main".into()),
        Op("="),
        KeyWord(If),
        KeyWord(True),
        KeyWord(Then),
        KeyWord(PrintLn),
        String("If".into()),
        KeyWord(Else),
        KeyWord(If),
        KeyWord(False),
        KeyWord(Then),
        KeyWord(PrintLn),
        String("Else If".into()),
        KeyWord(Else),
        KeyWord(PrintLn),
        String("Else".into()),
        DeDent,
    ];
    let left = tokens
        .iter()
        .map(|s| s.node.clone())
        .collect::<Vec<Token>>();
    assert_eq!(left, right);
}

#[test]
fn scanner_do_if_else_if_else() {
    let src = r#"
main = do
    if True then
        println "If"
    else if False then
        println "Else If"
    else
        println "Else"
"#;
    eprintln!("{}", src);
    let tokens = match scanner::scanner(FILENAME, src) {
        Ok(t) => t,
        Err(e) => {
            eprintln!("{}", e);
            Vec::new()
        }
    };
    let right = vec![
        Fn("main".into()),
        Op("="),
        KeyWord(Do),
        InDent,
        KeyWord(If),
        KeyWord(True),
        KeyWord(Then),
        InDent,
        KeyWord(PrintLn),
        String("If".into()),
        DeDent,
        KeyWord(Else),
        KeyWord(If),
        KeyWord(False),
        KeyWord(Then),
        InDent,
        KeyWord(PrintLn),
        String("Else If".into()),
        DeDent,
        KeyWord(Else),
        InDent,
        KeyWord(PrintLn),
        String("Else".into()),
        DeDent,
        DeDent,
    ];
    let left = tokens
        .iter()
        .map(|s| s.node.clone())
        .collect::<Vec<Token>>();
    assert_eq!(left, right);
}

#[test]
fn scanner_main_one_line_if() {
    use scanner::Token::{self, *};
    let src = "
main = if True then 100

";
    let tokens = match scanner::scanner(FILENAME, src) {
        Ok(t) => t,
        Err(e) => {
            eprintln!("{}", e);
            Vec::new()
        }
    };
    let left = tokens
        .iter()
        .map(|s| s.node.clone())
        .collect::<Vec<Token>>();
    let right = vec![
        Fn("main".into()),
        Op("="),
        KeyWord(If),
        KeyWord(True),
        KeyWord(Then),
        Int("100".into()),
        DeDent,
    ];
    assert_eq!(left, right);
}

#[test]
fn scanner_curry_app() {
    use scanner::Token::{self, *};
    let src = "main = (+ 1) 2";
    let tokens = match scanner::scanner(FILENAME, src) {
        Ok(t) => t,
        Err(e) => {
            eprintln!("{}", e);
            Vec::new()
        }
    };
    let left = tokens
        .iter()
        .map(|s| s.node.clone())
        .collect::<Vec<Token>>();
    let right = vec![
        Fn("main".into()),
        Op("="),
        Ctrl('('),
        Op("+"),
        Int("1".into()),
        Ctrl(')'),
        Int("2".into()),
        DeDent,
    ];
    assert_eq!(left, right);
}
