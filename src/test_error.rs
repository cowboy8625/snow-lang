// NOTE:
//      Should never get a UnknownChar error on its own.

use super::*;
#[cfg(test)]
use pretty_assertions::assert_eq;
fn from_string(src: &str) -> CResult<Expr> {
    run("testing.snow", src)
}

#[test]
fn error_no_main() {
    let src = "add x y = + x y";
    let result = from_string(src)
        .err()
        .map(|c| c.downcast::<crate::error::Error>().ok())
        .flatten()
        .map(|e| e.kind());
    assert_eq!(result, Some(ErrorKind::NoMain));
}

#[test]
fn error_no_return() {
    let src = "main = if False then 100 ";
    let result = from_string(src)
        .err()
        .map(|c| c.downcast::<crate::error::Error>().ok())
        .flatten()
        .map(|e| e.kind());
    assert_eq!(result, Some(ErrorKind::EmptyReturn));
}

// #[test]
// fn error_undefined() {
//     let src = "
// main = add 1 2
// ";
//     let result = from_string(src)
//         .err()
//         .map(|c| c.downcast::<crate::error::MulitError>().ok())
//         .map(|me| {
//             let mut errors = Vec::new();
//             if let Some(multi_error) = me {
//                 for error in multi_error.errors.iter() {
//                     errors.push(error.downcast::<crate::error::Error>().ok());
//                 }
//             }
//             errors
//         })
//         .map(|vec_errors| vec_errors.iter().cloned().map(|e| e.map(|e| e.kind())))
//         .collect::<Option<Vec<Option<Box<crate::error::Error>>>>>();
//     assert_eq!(result, vec![Some(ErrorKind::Undefined)]);
// }

// #[test]
// fn error_unclosed_delimiter() {
//     let src = "
// main = + (* 1 2 2
// ";
//     let result = from_string(src)
//         .err()
//         .map(|c| c.downcast::<crate::error::Error>().ok())
//         .flatten()
//         .map(|e| e.kind());
//     assert_eq!(result, Some(ErrorKind::UnclosedDelimiter));
// }
// UnclosedDelimiter,
// InvalidIndentation,
// ReserverdWord,
