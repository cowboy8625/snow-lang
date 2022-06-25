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

    pub fn bind_arg(&mut self, mut arg: Expr, local: &mut FunctionList) -> bool {
        if let Some(bind) = self.prams.first() {
            if let Expr::Local(name) = &arg {
                if let Some(func) = local.get(&name.node) {
                    arg = func.body();
                }
            }
            self.bound_args.push((bind.clone(), arg));
            self.prams.remove(0);
            return true;
        }
        false
    }

    pub fn local(&self, var: &mut FunctionList) {
        for (p, a) in self.bound_args.iter() {
            let span = p.span();
            let func = Self::new(&p.node, &self.prams, a.clone(), span);
            // var.insert(format!("{}.{}", self.name, p.node.clone()), func);
            var.insert(p.node.clone(), func);
        }
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
