use super::{precedence::Precedence, ParserBuilder};

#[test]
fn expression() {
    let src = "1";
    let left = ParserBuilder::default()
        .build(src)
        .expression(Precedence::None)
        .to_string();
    assert_eq!(left, src);

    let src = "1.2";
    let left = ParserBuilder::default()
        .build(src)
        .expression(Precedence::None)
        .to_string();
    assert_eq!(left, src);

    let src = "a";
    let left = ParserBuilder::default()
        .build(src)
        .expression(Precedence::None)
        .to_string();
    assert_eq!(left, src);
}
#[test]
fn unary() {
    let src = "-1";
    let left = ParserBuilder::default()
        .build(src)
        .expression(Precedence::None)
        .to_string();
    assert_eq!(left, "(- 1)");

    let src = "(- 1.2)";
    let left = ParserBuilder::default()
        .build(src)
        .expression(Precedence::None)
        .to_string();
    assert_eq!(left, "(- 1.2)");

    let src = "-a";
    let left = ParserBuilder::default()
        .build(src)
        .expression(Precedence::None)
        .to_string();
    assert_eq!(left, "(- a)");
}

#[test]
fn binary() {
    let src = "1 + 2 * 3";
    let left = ParserBuilder::default()
        .build(src)
        .expression(Precedence::None)
        .to_string();
    assert_eq!(left, "(+ 1 (* 2 3))");
}

#[test]
fn binary_ids() {
    let src = "a + b * c * d + e";
    let left = ParserBuilder::default()
        .build(src)
        .expression(Precedence::None)
        .to_string();
    assert_eq!(left, "(+ (+ a (* (* b c) d)) e)");

    let src = "a + b";
    let left = ParserBuilder::default()
        .build(src)
        .expression(Precedence::None)
        .to_string();
    assert_eq!(left, "(+ a b)");
}

#[test]
fn changing_precedence() {
    let src = "(-1 + 2) * 3 - -4";
    let left = ParserBuilder::default()
        .build(src)
        .expression(Precedence::None)
        .to_string();
    assert_eq!(left, "(- (* (+ (- 1) 2) 3) (- 4))");

    let src = "(((a)))";
    let left = ParserBuilder::default()
        .build(src)
        .expression(Precedence::None)
        .to_string();
    assert_eq!(left, "a");
}

#[test]
fn calling_operator() {
    let src = "(+) 1 2";
    let left = ParserBuilder::default()
        .build(src)
        .call(Precedence::None)
        .to_string();
    assert_eq!(left, "<(+): (1, 2)>");
}

#[test]
fn call() {
    let src = "add 1 2";
    let left = ParserBuilder::default()
        .build(src)
        .call(Precedence::None)
        .to_string();
    assert_eq!(left, "<add: (1, 2)>");
}

#[test]
fn pipe_call() {
    let src = "2 |> add 1";
    let left = ParserBuilder::default()
        .build(src)
        .call(Precedence::None)
        .to_string();
    assert_eq!(left, "(|> 2 <add: (1)>)");
}

#[test]
fn conditional() {
    let src = "if x > y then x else y;";
    let left = ParserBuilder::default()
        .build(src)
        .conditional()
        .to_string();
    assert_eq!(left, "(if ((> x y)) then x else y)");
}

#[test]
fn function_def() {
    let src = "add x y = x + y;";
    let left = ParserBuilder::default()
        .build(src)
        .function(0..0)
        .to_string();
    assert_eq!(left, r#"<add: (\x -> (\y -> (+ x y)))>"#);
}

#[test]
fn super_duper_function_def() {
    let src = "fn main = print (max ((add 1 2) + (sub 1 2)) 20);";
    let right = "<main: <print: (<max: ((+ <add: (1, 2)> <sub: (1, 2)>), 20)>)>>";
    let left = ParserBuilder::default().build(src).parse().unwrap()[0].to_string();
    assert_eq!(left, right);
}

#[test]
fn multi_function_def() {
    let src = "fn add x y = x + y; fn sub x y = x - y;";
    let left = ParserBuilder::default().build(src).parse().unwrap();
    let mut e = left.iter();
    assert_eq!(
        e.next().unwrap().to_string(),
        r#"<add: (\x -> (\y -> (+ x y)))>"#
    );
    assert_eq!(
        e.next().unwrap().to_string(),
        r#"<sub: (\x -> (\y -> (- x y)))>"#
    );
}

#[test]
fn closures() {
    let src = "fn add = (λx -> (λy -> x + y));";
    let right = r#"<add: (\x -> (\y -> (+ x y)))>"#;
    let left = ParserBuilder::default().build(src).parse().unwrap()[0].to_string();
    assert_eq!(left, right);

    let src = r#"fn add = (\x -> (\y -> x + y));"#;
    let right = r#"<add: (\x -> (\y -> (+ x y)))>"#;
    let left = ParserBuilder::default().build(src).parse().unwrap()[0].to_string();
    assert_eq!(left, right);
}

#[test]
fn user_type_def() {
    let src = r#"type Option = Some Int | None;"#;
    let right = r#"<Option: (Some, [Int]), (None, [])>"#;
    let left = ParserBuilder::default().build(src).parse().unwrap()[0].to_string();
    assert_eq!(left, right);
}

#[test]
fn type_dec() {
    let src = r#"add :: Int -> Int -> Int;"#;
    let right = r#"<add :: Int -> Int -> Int>"#;
    let left = ParserBuilder::default().build(src).parse().unwrap()[0].to_string();
    assert_eq!(left, right);
}
