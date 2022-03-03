// use super::parser::{self, Atom, BuiltIn, Expr, FunctionList, Parser};
// use super::position::Span;
// use super::scanner;
// #[cfg(test)]
// use pretty_assertions::assert_eq;
// // use scanner::KeyWord::*;
// // use scanner::Token::{self, *};
//
// const FILENAME: &str = "par_test.snow";
// #[test]
// fn parser_curry_app() {
//     let src = "main = ((+ 1) 2)";
//     let tokens = match scanner::scanner(FILENAME, src) {
//         Ok(t) => t,
//         Err(e) => {
//             eprintln!("{}", e);
//             Vec::new()
//         }
//     };
//     let (t, left) = match parser::parser().parse(&tokens) {
//         Ok((t, f)) => (t, f),
//         Err(t) => (t, FunctionList::new()),
//     };
//     dbg!(&left);
//     assert_eq!(t, vec![]);
//     let mut right = FunctionList::new();
//     // Application(Box<Spanned<Self>>, Vec<Spanned<Self>>),
//     // Function(Spanned<String>, Vec<Spanned<String>>, Box<Spanned<Self>>),
//     right.insert(
//         "main".into(),
//         (
//             Expr::Function(
//                 ("main".into(), Span::default()).into(),
//                 Vec::new(),
//                 Box::new(
//                     (
//                         Expr::Application(
//                             Box::new(
//                                     (
//                                         Expr::Application(
//                                             Box::new(
//                                                 (
//                                                     Expr::Constant(Atom::BuiltIn(BuiltIn::Plus)),
//                                                     Span::default(),
//                                                 )
//                                                     .into(),
//                                             ),
//                                             vec![(Expr::Constant(Atom::Int(1)), Span::default())
//                                                 .into()],
//                                         ),
//                                         Span::default(),
//                                     )
//                                         .into(),
//                             ),
//                             vec![(Expr::Constant(Atom::Int(2)), Span::default()).into()],
//                         ),
//                         Span::default(),
//                     )
//                         .into(),
//                 ),
//             ),
//             Span::default(),
//         )
//             .into(),
//     );
//     dbg!(&right);
//     assert_eq!(left, right);
// }
