pub mod error;
pub mod expr;
pub mod op;
pub mod parser;
pub use expr::{App, Atom, Binary, Expr, TypeInfo, Unary};
pub use op::Op;

#[cfg(test)]
mod tests;
pub use snowc_lexer::{Ident, Scanner, Span, Token, TokenPosition};

use error::Error;
pub use parser::parse;
type Result<T> = std::result::Result<T, Error>;
type ParserResult = std::result::Result<Vec<Expr>, Vec<Error>>;

pub fn expression(src: &str) -> Result<Expr> {
    let mut tokens: Vec<Token> = Scanner::new(src).collect();
    parser::expression(&mut tokens)
}
