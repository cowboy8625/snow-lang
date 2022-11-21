mod scanner;
#[cfg(test)]
mod test;
mod token;

pub type Span = std::ops::Range<usize>;

pub use crate::scanner::Scanner;
pub use crate::token::Token;
