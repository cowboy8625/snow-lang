pub mod error;
pub mod expr;
pub mod op;
pub mod parser;
pub use expr::{Atom, Expr};
pub use op::Op;

// mod precedence;
#[cfg(test)]
mod tests;
pub use snowc_lexer::{Scanner, Span, Token, TokenPosition};

pub use parser::parse;

// pub fn parse(scanner: Scanner) -> Result<Vec<Expr>, Vec<error::Error>> {
//     let stream = scanner.peekable();
//     Parser::new(stream).parse()
// }
//
// pub fn parse_expr(scanner: Scanner) -> Result<Expr, Vec<error::Error>> {
//     let stream = scanner.peekable();
//     Ok(Parser::new(stream).conditional())
// }
//
// #[macro_export]
// macro_rules! bail {
//     ($span:expr $(, $arg:expr)* $(,)?) => {{
//         let msg = format!($($arg, )*);
//         return Err(Box::new($crate::error::ParserError::new(msg, $span)));
//     }};
// }
