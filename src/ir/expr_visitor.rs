use crate::front_end::{Atom, Expr, Operator};
use crate::ir::Module;

pub trait ExprVisitor {
    fn visit_atom(&mut self, atom: &Atom);
    fn visit_binary_op(&mut self, left: &Expr, op: &Operator, right: &Expr);
    fn visit_if(
        &mut self,
        condition: &Expr,
        then_branch: &Expr,
        // TODO: convert this Option to Expr because there is no option for an else branch
        // you are required to have an else branch in a if expression
        else_branch: Option<Box<Expr>>,
    );
    fn visit_function(
        &mut self,
        name: &str,
        params: &[String],
        signature: &[String],
        body: &Expr,
    );
    fn visit_expr(&mut self, expr: &Expr);
    fn visit(&mut self, expressions: &[Expr]) -> Module;
}
