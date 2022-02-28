use std::collections::HashMap;

use super::{
    position::{Span, Spanned},
    scanner::{KeyWord, Token},
};
mod atoms;
mod builtins;
mod expr;
mod mini_parse;
mod parser;

pub use mini_parse::{ParseResult, Parser};
pub use parser::parser;

pub type FunctionList = HashMap<String, Spanned<Expr>>;

pub use atoms::Atom;
use atoms::{boolean, keyword, number, string};
use builtins::builtin;
pub use builtins::BuiltIn;
use expr::function;
pub use expr::{app, Expr};
