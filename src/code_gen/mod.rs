mod haskell;
#[cfg(test)]
mod test_haskell;
use super::parser::Expr;
use super::FunctionList;
#[cfg(test)]
use super::{
    error::CResult,
    parser::{self, Parser},
    scanner,
};
pub use haskell::haskell_code_gen;
