// use super::{error::Error, parse, Scanner, parse_expr};
//
// use pretty_assertions::assert_eq;
//
// macro_rules! testme {
//     ($name:ident, $src:expr, $expected:expr $(,)?) => {
//         #[test]
//         fn $name() -> Result<(), Vec<Error>> {
//             let ast = match parse(Scanner::new($src)) {
//                 Ok(ok) => ok,
//                 Err(errors) => {
//                     for err in errors.iter() {
//                         let msg = err.report("nothing", $src);
//                         eprintln!("{msg}");
//                     }
//                     return Err(errors);
//                 }
//             };
//             for node in ast.iter() {
//                 eprintln!("{node}");
//             }
//             let left = ast.iter().map(ToString::to_string).collect::<Vec<_>>();
//             assert_eq!(left, $expected);
//             Ok(())
//         }
//     };
// }
//
// #[test]
// fn is_error() {
//     let ast = parse_expr(Scanner::new("1 +"));
//     assert!(ast.iter().any(|x| x.is_error()));
// }
//
// testme!(
//     expression_int,
//     "main :: Int; main = 1;",
//     vec!["<main :: Int>", "<main: 1>",],
// );
//
// testme!(
//     expression_float,
//     "main :: Int; main = 1.2;",
//     vec!["<main :: Int>", "<main: 1.2>",],
// );
//
// testme!(
//     expression_ident,
//     "main :: Int; main = a;",
//     vec!["<main :: Int>", "<main: a>",],
// );
//
// testme!(
//     expression_unary_neg_int,
//     "main :: Int; main = -1;",
//     vec!["<main :: Int>", "<main: (- 1)>",],
// );
//
// testme!(
//     expression_unary_neg_float,
//     "main :: Int; main = -1.223;",
//     vec!["<main :: Int>", "<main: (- 1.223)>",],
// );
//
// testme!(
//     expression_unary_neg_ident,
//     "main :: Int; main = -a;",
//     vec!["<main :: Int>", "<main: (- a)>",],
// );
//
// testme!(
//     expression_unary_not_true,
//     "main = !true;",
//     vec!["<main: (! true)>",],
// );
//
// testme!(
//     expression_unary_not_false,
//     "main = !false;",
//     vec!["<main: (! false)>",],
// );
//
// testme!(
//     expression_binary,
//     "main = 1 + 2 * 3;",
//     vec!["<main: (+ 1 (* 2 3))>"],
// );
//
// testme!(
//     expression_binary_ident_1,
//     "main = a + b * c * d + e;",
//     vec!["<main: (+ (+ a (* (* b c) d)) e)>"],
// );
//
// testme!(
//     expression_binary_ident_2,
//     "main = a + b;",
//     vec!["<main: (+ a b)>"],
// );
//
// testme!(changing_precedence_1, "main = (((a)));", vec!["<main: a>"],);
//
// testme!(
//     changing_precedence_2,
//     "main = (-1 + 2) * 3 - -4;",
//     vec!["<main: (- (* (+ (- 1) 2) 3) (- 4))>"],
// );
//
// testme!(
//     calling_operator,
//     "main = (+) 1 2;",
//     vec!["<main: <(+): (1, 2)>>"],
// );
//
// testme!(call, "main = add 1 2;", vec!["<main: <add: (1, 2)>>"],);
//
// testme!(
//     pipe_call_right_to_left,
//     "main = (add 1) <| 2;",
//     vec!["<main: <add: (1, 2)>>"],
// );
//
// testme!(
//     pipe_call_left_to_right,
//     "main = 2 |> (add 1);",
//     vec!["<main: <add: (1, 2)>>"],
// );
//
// testme!(
//     pipe_call,
//     "main = 2 |> add <| 1;",
//     vec!["<main: <add: (1, 2)>>"],
// );
//
// testme!(
//     conditional,
//     "main = if x > y then x else y;",
//     vec!["<main: (if ((> x y)) then x else y)>"],
// );
//
// testme!(
//     function_def_from_parse_funtion,
//     "add x y = x + y; main = add 1 2;",
//     vec![
//         r#"<add: (\x -> (\y -> (+ x y)))>"#,
//         r#"<main: <add: (1, 2)>>"#
//     ],
// );
//
// testme!(
//     function_def,
//     "add x y = x + y; main = add 1 2;",
//     vec![
//         r#"<add: (\x -> (\y -> (+ x y)))>"#,
//         r#"<main: <add: (1, 2)>>"#
//     ],
// );
//
// testme!(
//     super_duper_function_def,
//     "main = print (max ((add 1 2) + (sub 1 2)) 20);",
//     vec![r#"<main: <print: (<max: ((+ <add: (1, 2)> <sub: (1, 2)>), 20)>)>>"#],
// );
//
// testme!(
//     multi_function_def,
//     "add x y = x + y; sub x y = x - y; main = sub (add 1 2) 3;",
//     vec![
//         r#"<add: (\x -> (\y -> (+ x y)))>"#,
//         r#"<sub: (\x -> (\y -> (- x y)))>"#,
//         r#"<main: <sub: (<add: (1, 2)>, 3)>>"#,
//     ],
// );
//
// testme!(
//     closures_lambda_sign,
//     "add = (λx -> (λy -> x + y)); main = add 1 2;",
//     vec![
//         r#"<add: (\x -> (\y -> (+ x y)))>"#,
//         r#"<main: <add: (1, 2)>>"#,
//     ],
// );
//
// testme!(
//     closures,
//     r#"add = (\x -> (\y -> x + y)); main = add 1 2;"#,
//     vec![
//         r#"<add: (\x -> (\y -> (+ x y)))>"#,
//         r#"<main: <add: (1, 2)>>"#,
//     ],
// );
//
// testme!(
//     enum_def,
//     r#"enum Option = Some Int | None; main = 1;"#,
//     vec![r#"<Option: (Some, [Int]), (None, [])>"#, r#"<main: 1>"#,],
// );
//
// testme!(
//     type_dec,
//     r#"add :: Int -> Int -> Int; main = 1;"#,
//     vec![r#"<add :: Int -> Int -> Int>"#, r#"<main: 1>"#,],
// );
//
// testme!(
//     array,
//     r#"main = [1, 2, 3, 4];"#,
//     vec![r#"<main: [1, 2, 3, 4]>"#,],
// );
//
// testme!(
//     expression_mod,
//     r#"main = 1 mod 10;"#,
//     vec![r#"<main: (mod 1 10)>"#,],
// );
