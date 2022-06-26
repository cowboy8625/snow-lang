use super::parser::Expr;
use super::position::{Span, Spanned};
use std::collections::HashMap;

pub type FunctionList = HashMap<String, Function>;

#[derive(Debug, Clone, PartialEq)]
pub struct Function {
    name: String,
    prams: Vec<Spanned<String>>,
    bound_args: Vec<(Spanned<String>, Expr)>,
    body: Expr,
    span: Span,
}

impl Function {
    pub fn new(name: &str, prams: &[Spanned<String>], body: Expr, span: Span) -> Self {
        Self {
            name: name.into(),
            prams: prams.to_vec(),
            bound_args: Vec::new(),
            body,
            span,
        }
    }

    pub fn prams(&self) -> &[Spanned<String>] {
        &self.prams
    }

    pub fn body(&self) -> Expr {
        self.body.clone()
    }
}
