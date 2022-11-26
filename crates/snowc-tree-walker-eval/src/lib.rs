use snowc_parse::{Atom, Expr, Op};
use std::collections::HashMap;
pub type FuncMap = HashMap<String, Expr>;

pub fn eval(expr: Expr, stack: &mut Vec<Atom>, funcs: &mut FuncMap) -> Option<Atom> {
    match expr {
        Expr::Atom(a) => Some(a),
        Expr::Unary(op, atom) => match op {
            Op::Minus => Some(match eval(*atom, stack, funcs)? {
                Atom::Int(i) => Atom::Int(-i),
                Atom::Float(i) => Atom::Float(-i),
                _ => unimplemented!(),
            }),
            Op::Not => Some(match eval(*atom, stack, funcs)? {
                Atom::Bool(i) => Atom::Bool(!i),
                _ => unimplemented!(),
            }),
            _ => unimplemented!(),
        },
        Expr::Binary(op, lhs, rhs) => match op {
            Op::Minus => Some(
                match (eval(*lhs, stack, funcs)?, eval(*rhs, stack, funcs)?) {
                    (Atom::Int(l), Atom::Int(r)) => Atom::Int(l - r),
                    (Atom::Float(l), Atom::Float(r)) => Atom::Float(l - r),
                    _ => unimplemented!(),
                },
            ),
            Op::Plus => Some(
                match (eval(*lhs, stack, funcs)?, eval(*rhs, stack, funcs)?) {
                    (Atom::Int(l), Atom::Int(r)) => Atom::Int(l + r),
                    (Atom::Float(l), Atom::Float(r)) => Atom::Float(l + r),
                    _ => unimplemented!(),
                },
            ),
            Op::Div => Some(
                match (eval(*lhs, stack, funcs)?, eval(*rhs, stack, funcs)?) {
                    (Atom::Int(l), Atom::Int(r)) => Atom::Int(l / r),
                    (Atom::Float(l), Atom::Float(r)) => Atom::Float(l / r),
                    _ => unimplemented!(),
                },
            ),
            Op::Mult => Some(
                match (eval(*lhs, stack, funcs)?, eval(*rhs, stack, funcs)?) {
                    (Atom::Int(l), Atom::Int(r)) => Atom::Int(l * r),
                    (Atom::Float(l), Atom::Float(r)) => Atom::Float(l * r),
                    _ => unimplemented!(),
                },
            ),
            Op::Grt => Some(
                match (eval(*lhs, stack, funcs)?, eval(*rhs, stack, funcs)?) {
                    (Atom::Int(l), Atom::Int(r)) => Atom::Bool(l > r),
                    (Atom::Float(l), Atom::Float(r)) => Atom::Bool(l > r),
                    _ => unimplemented!(),
                },
            ),
            Op::Les => Some(
                match (eval(*lhs, stack, funcs)?, eval(*rhs, stack, funcs)?) {
                    (Atom::Int(l), Atom::Int(r)) => Atom::Bool(l < r),
                    (Atom::Float(l), Atom::Float(r)) => Atom::Bool(l < r),
                    _ => unimplemented!(),
                },
            ),
            Op::GrtEq => Some(
                match (eval(*lhs, stack, funcs)?, eval(*rhs, stack, funcs)?) {
                    (Atom::Int(l), Atom::Int(r)) => Atom::Bool(l >= r),
                    (Atom::Float(l), Atom::Float(r)) => Atom::Bool(l >= r),
                    _ => unimplemented!(),
                },
            ),
            Op::LesEq => Some(
                match (eval(*lhs, stack, funcs)?, eval(*rhs, stack, funcs)?) {
                    (Atom::Int(l), Atom::Int(r)) => Atom::Bool(l <= r),
                    (Atom::Float(l), Atom::Float(r)) => Atom::Bool(l <= r),
                    _ => unimplemented!(),
                },
            ),
            Op::Eq => Some(
                match (eval(*lhs, stack, funcs)?, eval(*rhs, stack, funcs)?) {
                    (Atom::Int(l), Atom::Int(r)) => Atom::Bool(l == r),
                    (Atom::Float(l), Atom::Float(r)) => Atom::Bool(l == r),
                    _ => unimplemented!(),
                },
            ),
            _ => unimplemented!(),
        },
        Expr::IfElse(condition, branch1, branch2) => {
            Some(match eval(*condition, stack, funcs)? {
                Atom::Bool(true) => eval(*branch1, stack, funcs)?,
                Atom::Bool(false) => eval(*branch2, stack, funcs)?,
                _ => unimplemented!(),
            })
        }
        Expr::App(_head, _tail) => {
            todo!()
        }
        // \x -> \y -> x + Y
        Expr::Clouser(_head, _tail) => {
            todo!()
        }

        Expr::Func(name, body) => {
            funcs.insert(name, *body);
            None
        }
        // Self::Type(String, Vec<(String, Vec<String>)>),
        // Self::TypeDec(String, Vec<String>),
        _ => unimplemented!(),
    }
}
