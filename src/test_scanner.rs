use super::scanner;

const FILENAME: &str = "scan_test.snow";

#[test]
fn scanner_let_block_one_line() {
    use scanner::KeyWord::*;
    use scanner::Token::{self, *};
    let src = "
main = let a = 10, b = 12 in a + b
";
    let (tokens, err) = match scanner::scanner(FILENAME, src) {
        Ok(t) => (t, Vec::new()),
        Err((t, e)) => (t, e),
    };
    let tokens = tokens
        .iter()
        .map(|s| s.node.clone())
        .collect::<Vec<Token>>();
    assert!(err.is_empty());
    assert_eq!(
        tokens,
        vec![
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
        ]
    );
}

#[test]
fn scanner_let_block_multi_lines() {
    use scanner::KeyWord::*;
    use scanner::Token::{self, *};
    let src = "
main =
    let a = 10
    ,   b = 12
    in  a + b
";
    let (tokens, err) = match scanner::scanner(FILENAME, src) {
        Ok(t) => (t, Vec::new()),
        Err((t, e)) => (t, e),
    };
    let tokens = tokens
        .iter()
        .map(|s| s.node.clone())
        .collect::<Vec<Token>>();
    assert!(err.is_empty());
    assert_eq!(
        tokens,
        vec![
            DeDent,
            Id("main".into()),
            Op("="),
            InDent(4),
            KeyWord(Let),
            Id("a".into()),
            Op("="),
            Int("10".into()),
            InDent(4),
            Ctrl(','),
            Id("b".into()),
            Op("="),
            Int("12".into()),
            InDent(4),
            KeyWord(In),
            Id("a".into()),
            Op("+"),
            Id("b".into()),
        ]
    );
}

#[test]
fn scanner_let_block_single_line() {
    use scanner::KeyWord::*;
    use scanner::Token::{self, *};
    let src = "
add x y = let a = x, b = y in + a b
main x y = let a = x, b = y in + a b
";
    let (tokens, err) = match scanner::scanner(FILENAME, src) {
        Ok(t) => (t, Vec::new()),
        Err((t, e)) => (t, e),
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
    assert!(err.is_empty());
    for (l, r) in left.iter().zip(right.clone()) {
        eprintln!("{} == {} = {}", l, r, l == &r);
        assert!(l == &r);
    }
    assert_eq!(left, right);
}
