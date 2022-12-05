use snowc_parse::{Atom, Expr, Op};
use std::collections::HashMap;
pub type FuncMap = HashMap<String, Function>;

#[derive(Debug, Clone, Hash)]
pub struct Function {
    pub params: Vec<String>,
    pub body: Expr,
}

pub fn eval(expr: Expr, funcs: &mut FuncMap) -> Option<Atom> {
    match expr {
        Expr::Atom(Atom::Id(name)) => {
            let Some(Function { body, .. }) = funcs.get(&name) else {
                println!("NONE\r");
                return None;
            };
            eval(body.clone(), funcs)
        }
        Expr::Atom(a) => Some(a),
        Expr::Unary(op, atom) => match op {
            Op::Minus => Some(match eval(*atom, funcs)? {
                Atom::Int(i) => Atom::Int(-i),
                Atom::Float(i) => Atom::Float((-i.parse::<f32>().unwrap()).to_string()),
                _ => unimplemented!(),
            }),
            Op::Not => Some(match eval(*atom, funcs)? {
                Atom::Bool(i) => Atom::Bool(!i),
                _ => unimplemented!(),
            }),
            _ => unimplemented!(),
        },
        Expr::Binary(op, lhs, rhs) => match op {
            Op::Minus => Some(match (eval(*lhs, funcs)?, eval(*rhs, funcs)?) {
                (Atom::Int(l), Atom::Int(r)) => Atom::Int(l - r),
                (Atom::Float(l), Atom::Float(r)) => Atom::Float(
                    (l.parse::<f32>().unwrap() - r.parse::<f32>().unwrap()).to_string(),
                ),
                _ => unimplemented!(),
            }),
            Op::Plus => Some(match (eval(*lhs, funcs)?, eval(*rhs, funcs)?) {
                (Atom::Int(l), Atom::Int(r)) => Atom::Int(l + r),
                (Atom::Float(l), Atom::Float(r)) => Atom::Float(
                    (l.parse::<f32>().unwrap() + r.parse::<f32>().unwrap()).to_string(),
                ),
                a => unimplemented!("missing implementation of '{:?}'", a),
            }),
            Op::Div => Some(match (eval(*lhs, funcs)?, eval(*rhs, funcs)?) {
                (Atom::Int(l), Atom::Int(r)) => Atom::Int(l / r),
                (Atom::Float(l), Atom::Float(r)) => Atom::Float(
                    (l.parse::<f32>().unwrap() / r.parse::<f32>().unwrap()).to_string(),
                ),
                _ => unimplemented!(),
            }),
            Op::Mult => Some(match (eval(*lhs, funcs)?, eval(*rhs, funcs)?) {
                (Atom::Int(l), Atom::Int(r)) => Atom::Int(l * r),
                (Atom::Float(l), Atom::Float(r)) => Atom::Float(
                    (l.parse::<f32>().unwrap() / r.parse::<f32>().unwrap()).to_string(),
                ),
                _ => unimplemented!(),
            }),
            Op::Grt => Some(match (eval(*lhs, funcs)?, eval(*rhs, funcs)?) {
                (Atom::Int(l), Atom::Int(r)) => Atom::Bool(l > r),
                (Atom::Float(l), Atom::Float(r)) => {
                    Atom::Bool(l.parse::<f32>().unwrap() > r.parse::<f32>().unwrap())
                }
                _ => unimplemented!(),
            }),
            Op::Les => Some(match (eval(*lhs, funcs)?, eval(*rhs, funcs)?) {
                (Atom::Int(l), Atom::Int(r)) => Atom::Bool(l < r),
                (Atom::Float(l), Atom::Float(r)) => {
                    Atom::Bool(l.parse::<f32>().unwrap() < r.parse::<f32>().unwrap())
                }
                _ => unimplemented!(),
            }),
            Op::GrtEq => Some(match (eval(*lhs, funcs)?, eval(*rhs, funcs)?) {
                (Atom::Int(l), Atom::Int(r)) => Atom::Bool(l >= r),
                (Atom::Float(l), Atom::Float(r)) => {
                    Atom::Bool(l.parse::<f32>().unwrap() >= r.parse::<f32>().unwrap())
                }
                _ => unimplemented!(),
            }),
            Op::LesEq => Some(match (eval(*lhs, funcs)?, eval(*rhs, funcs)?) {
                (Atom::Int(l), Atom::Int(r)) => Atom::Bool(l <= r),
                (Atom::Float(l), Atom::Float(r)) => {
                    Atom::Bool(l.parse::<f32>().unwrap() <= r.parse::<f32>().unwrap())
                }
                _ => unimplemented!(),
            }),
            Op::Eq => Some(match (eval(*lhs, funcs)?, eval(*rhs, funcs)?) {
                (Atom::Int(l), Atom::Int(r)) => Atom::Bool(l == r),
                (Atom::Float(l), Atom::Float(r)) => {
                    Atom::Bool(l.parse::<f32>().unwrap() == r.parse::<f32>().unwrap())
                }
                _ => unimplemented!(),
            }),
            _ => unimplemented!(),
        },
        Expr::IfElse(condition, branch1, branch2) => {
            Some(match eval(*condition, funcs)? {
                Atom::Bool(true) => eval(*branch1, funcs)?,
                Atom::Bool(false) => eval(*branch2, funcs)?,
                _ => unimplemented!(),
            })
        }
        Expr::App(head, args) => {
            let Expr::Atom(Atom::Id(name)) = *head else {
                let mut local = funcs.clone();
                let (params, body) = seperate_args_from_body(*head);
                for (param, arg) in params.iter().zip(args) {
                    local.insert(
                        format!("{param}"),
                        Function {
                            params: vec![],
                            body: arg,
                        },
                    );
                }
                return eval(body, &mut local);
            };
            if let Some(Function { params, body }) = funcs.get(&name) {
                let mut local = funcs.clone();
                for (param, arg) in params.iter().zip(args) {
                    local.insert(
                        format!("{param}"),
                        Function {
                            params: vec![],
                            body: arg,
                        },
                    );
                }
                return eval(body.clone(), &mut local);
            }
            todo!()
        }
        // \x -> \y -> x + Y
        Expr::Clouser(_head, _tail) => {
            println!("CLOUSER");
            todo!()
        }

        Expr::Func(name, body) => {
            let (params, body) = seperate_args_from_body(*body);
            let func = Function { params, body };
            funcs.insert(name, func);
            None
        }
        // Self::Type(String, Vec<(String, Vec<String>)>),
        // Self::TypeDec(String, Vec<String>),
        _ => unimplemented!(),
    }
}

pub fn seperate_args_from_body(expr: Expr) -> (Vec<String>, Expr) {
    if let Expr::Clouser(head, tail) = expr {
        let mut args = vec![head.to_string()];
        let mut tail: Expr = Clone::clone(&tail);
        while let Some((name, next_tail)) = closure(tail.clone()) {
            tail = next_tail;
            args.push(name);
        }
        return (args, tail);
    }
    (vec![], expr)
}

fn closure(expr: Expr) -> Option<(String, Expr)> {
    let Expr::Clouser(head, tail) = expr else {
        return None;
    };
    Some((head.to_string(), *tail))
}
