use pretty_assertions::assert_eq;

use super::*;
use snowc_parse::{parse, Scanner};

macro_rules! build_test {
    ($name:ident, $src:expr, $expected:expr $(,)?) => {
        #[test]
        fn $name() {
            let scanner = Scanner::new($src);
            let ast = parse(scanner).unwrap();
            let v = match walk(&ast) {
                Ok(v) => v,
                Err(err) => {
                    for e in err.iter() {
                        e.report("yo momma", $src);
                    }
                    eprintln!("{err:?}");
                    None
                }
            };
            assert_eq!(v, $expected);
        }
    };
}

build_test! {
    unary,
    "main = -1;",
    Some(Value::Int(-1, Span::new(0, 0, 0, 0, 7, 9)))
}

#[test]
fn hhhhhh() {
    let src = r#"
create_grid size char = if (0 < (size - 1)) then (push (create_grid (size - 1) char) char) else ([char]);
main = create_grid 9 " ";
        "#;
    let scanner = Scanner::new(src);
    let ast = parse(scanner).unwrap();
    let value = walk(&ast).unwrap();
    assert_eq!(
        Some(Value::Array(
            vec![
                Value::String(String::from(" "), Span::default()),
                Value::String(String::from(" "), Span::default()),
                Value::String(String::from(" "), Span::default()),
                Value::String(String::from(" "), Span::default()),
            ],
            Span::default()
        )),
        value
    );
}
