use super::scanner;
#[cfg(test)]
use pretty_assertions::assert_eq;

const FILENAME: &str = "scan_test.snow";

#[test]
fn scanner_let_block_one_line() {
    use scanner::KeyWord::*;
    use scanner::Token::{self, *};
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
        DeDent,
        Id("main".into()),
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
    ];
    assert_eq!(left, right);
}

#[test]
fn scanner_let_block_multi_lines() {
    use scanner::KeyWord::*;
    use scanner::Token::{self, *};
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
        DeDent,
        Id("main".into()),
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
    use scanner::KeyWord::*;
    use scanner::Token::{self, *};
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
        DeDent,
        Id("add".into()),
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
        Id("main".into()),
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
    ];
    let left = tokens
        .iter()
        .map(|s| s.node.clone())
        .collect::<Vec<Token>>();
    assert_eq!(left, right);
}

#[test]
fn scanner_if_else_if_else() {
    use scanner::KeyWord::*;
    use scanner::Token::{self, *};
    let src = "
main = if True then println \"If\" else if False then println \"Else If\" else println \"Else\"";
    let tokens = match scanner::scanner(FILENAME, src) {
        Ok(t) => t,
        Err(e) => {
            eprintln!("{}", e);
            Vec::new()
        }
    };
    let right = vec![
        DeDent,
        Id("main".into()),
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
    ];
    let left = tokens
        .iter()
        .map(|s| s.node.clone())
        .collect::<Vec<Token>>();
    assert_eq!(left, right);
}

#[test]
fn scanner_do_if_else_if_else() {
    use scanner::KeyWord::*;
    use scanner::Token::{self, *};
    let src = r#"main = do
    if True then
        println "If"
    else if False then
        println "Else If"
    else
        println "Else""#;
    let tokens = match scanner::scanner(FILENAME, src) {
        Ok(t) => t,
        Err(e) => {
            eprintln!("{}", e);
            Vec::new()
        }
    };
    let right = vec![
        DeDent,
        Id("main".into()),
        Op("="),
        KeyWord(Do),
        InDent,
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
