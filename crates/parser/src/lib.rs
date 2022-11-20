pub mod error;
pub mod expr;
mod op;
pub mod parser;
mod precedence;
#[cfg(test)]
mod tests;
pub use crate::parser::parse;
use op::Op;
use scanner::{Scanner, Token};
use snowc_errors::CResult;

type Span = std::ops::Range<usize>;

#[macro_export]
macro_rules! bail {
    ($span:expr $(, $arg:expr)* $(,)?) => {
        return Err(Box::new(crate::error::ParserError::new(
                    format!($($arg,) *),
                    $span
        )))
    };
}
