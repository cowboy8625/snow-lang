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

pub type FunctionList = HashMap<String, Function>;

pub use atoms::Atom;
use atoms::{boolean, number, string};
use builtins::builtin;
pub use builtins::BuiltIn;
use expr::function;
pub use expr::{app, Expr};

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

    pub fn bind_arg(&mut self, arg: Expr) -> bool {
        if let Some(name) = self.prams.pop() {
            self.bound_args.push((name, arg));
            return true;
        }
        false
    }

    pub fn local(&self, var: &mut FunctionList) {
        for (p, a) in self.bound_args.iter() {
            let span = p.span();
            let func = Self::new(&p.node, &self.prams, a.clone(), span);
            var.insert(p.node.clone(), func);
        }
    }

    pub fn body(&self) -> Expr {
        self.body.clone()
    }

    pub fn reduce<'a>(&mut self, args: &[Spanned<Expr>]) -> Vec<Spanned<Expr>> {
        match self.body.clone() {
            Expr::Application(lhs, mut rhs) => {
                rhs.extend_from_slice(args);
                self.body = lhs.node.clone();
                self.reduce(&rhs.to_vec())
            }
            _ => args.to_vec(),
        }
    }

    pub fn into_app(&mut self, args: &[Spanned<Expr>]) -> Expr {
        let args = self.reduce(args);
        Expr::Application(
            Box::new((self.body.clone(), self.span.clone()).into()),
            args.into(),
        )
    }
}
