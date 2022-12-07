use snowc_parse::{Atom, Expr, Op};
use std::collections::HashMap;
pub type FuncMap = HashMap<String, Function>;

#[derive(Debug, Clone, Hash)]
pub struct Function {
    pub params: Vec<String>,
    pub body: Expr,
}

// FIXME: Option needs to be a result cause eval can fail
pub fn eval(expr: Expr, funcs: &mut FuncMap) -> Option<Atom> {
    match expr {
        Expr::Atom(Atom::Id(name)) => atom_look_up(&name, funcs),
        Expr::Atom(a) => atom_unwrap(a),
        Expr::Unary(op, rhs) => unary(op, *rhs, funcs),
        Expr::Binary(op, lhs, rhs) => binary(op, *lhs, *rhs, funcs),
        Expr::IfElse(cond, b1, b2) => if_else(*cond, *b1, *b2, funcs),
        Expr::App(head, args) => app(*head, &args, funcs),
        Expr::Closure(head, tail) => closure(*head, *tail, funcs),
        Expr::Func(name, body) => function(&name, *body, funcs),
        // Self::Type(String, Vec<(String, Vec<String>)>),
        // Self::TypeDec(String, Vec<String>),
        _ => unimplemented!(),
    }
}

pub fn seperate_args_from_body(expr: Expr) -> (Vec<String>, Expr) {
    let Expr::Closure(head, tail) = expr else {
        return (vec![], expr);
    };
    let mut args = vec![head.to_string()];
    let mut tail: Expr = Clone::clone(&tail);
    while let Some((name, next_tail)) = get_closure_arg_body(tail.clone()) {
        tail = next_tail;
        args.push(name);
    }
    (args, tail)
}

fn get_closure_arg_body(expr: Expr) -> Option<(String, Expr)> {
    let Expr::Closure(head, tail) = expr else {
        return None;
    };
    Some((head.to_string(), *tail))
}

fn atom_look_up(name: &str, funcs: &mut FuncMap) -> Option<Atom> {
    let Some(Function { body, .. }) = funcs.get(name) else {
        eprintln!("can not find '{name}' in scope\r");
        return None;
    };
    eval(body.clone(), funcs)
}

fn atom_unwrap(a: Atom) -> Option<Atom> {
    Some(a)
}

fn unary(op: Op, rhs: Expr, funcs: &mut FuncMap) -> Option<Atom> {
    match op {
        Op::Minus => unary_minus(rhs, funcs),
        Op::Not => unary_not(rhs, funcs),
        _ => unimplemented!(),
    }
}

fn unary_minus(rhs: Expr, funcs: &mut FuncMap) -> Option<Atom> {
    Some(match eval(rhs, funcs)? {
        Atom::Int(i) => Atom::Int(-i),
        Atom::Float(i) => Atom::Float((-i.parse::<f32>().unwrap()).to_string()),
        _ => unimplemented!(),
    })
}

fn unary_not(rhs: Expr, funcs: &mut FuncMap) -> Option<Atom> {
    Some(match eval(rhs, funcs)? {
        Atom::Bool(i) => Atom::Bool(!i),
        _ => unimplemented!(),
    })
}

fn binary(op: Op, lhs: Expr, rhs: Expr, funcs: &mut FuncMap) -> Option<Atom> {
    match op {
        Op::Minus => binary_minus(lhs, rhs, funcs),
        Op::Plus => binary_plus(lhs, rhs, funcs),
        Op::Div => binary_div(lhs, rhs, funcs),
        Op::Mult => binary_mult(lhs, rhs, funcs),
        Op::Grt => binary_grt(lhs, rhs, funcs),
        Op::Les => binary_less(lhs, rhs, funcs),
        Op::GrtEq => binary_grt_eq(lhs, rhs, funcs),
        Op::LesEq => binary_les_eq(lhs, rhs, funcs),
        Op::Eq => binary_eq(lhs, rhs, funcs),
        a => unimplemented!("'{a:?}'"),
    }
}

fn binary_minus(lhs: Expr, rhs: Expr, funcs: &mut FuncMap) -> Option<Atom> {
    Some(match (eval(lhs, funcs)?, eval(rhs, funcs)?) {
        (Atom::Int(l), Atom::Int(r)) => Atom::Int(l - r),
        (Atom::Float(l), Atom::Float(r)) => Atom::Float(
            (l.parse::<f32>().unwrap() - r.parse::<f32>().unwrap()).to_string(),
        ),
        _ => unimplemented!(),
    })
}

fn binary_plus(lhs: Expr, rhs: Expr, funcs: &mut FuncMap) -> Option<Atom> {
    Some(match (eval(lhs, funcs)?, eval(rhs, funcs)?) {
        (Atom::Int(l), Atom::Int(r)) => Atom::Int(l + r),
        (Atom::Float(l), Atom::Float(r)) => Atom::Float(
            (l.parse::<f32>().unwrap() + r.parse::<f32>().unwrap()).to_string(),
        ),
        a => unimplemented!("missing implementation of '{:?}'", a),
    })
}

fn binary_div(lhs: Expr, rhs: Expr, funcs: &mut FuncMap) -> Option<Atom> {
    Some(match (eval(lhs, funcs)?, eval(rhs, funcs)?) {
        (Atom::Int(l), Atom::Int(r)) => Atom::Int(l / r),
        (Atom::Float(l), Atom::Float(r)) => Atom::Float(
            (l.parse::<f32>().unwrap() / r.parse::<f32>().unwrap()).to_string(),
        ),
        _ => unimplemented!(),
    })
}

fn binary_mult(lhs: Expr, rhs: Expr, funcs: &mut FuncMap) -> Option<Atom> {
    Some(match (eval(lhs, funcs)?, eval(rhs, funcs)?) {
        (Atom::Int(l), Atom::Int(r)) => Atom::Int(l * r),
        (Atom::Float(l), Atom::Float(r)) => Atom::Float(
            (l.parse::<f32>().unwrap() / r.parse::<f32>().unwrap()).to_string(),
        ),
        _ => unimplemented!(),
    })
}

fn binary_grt(lhs: Expr, rhs: Expr, funcs: &mut FuncMap) -> Option<Atom> {
    Some(match (eval(lhs, funcs)?, eval(rhs, funcs)?) {
        (Atom::Int(l), Atom::Int(r)) => Atom::Bool(l > r),
        (Atom::Float(l), Atom::Float(r)) => {
            Atom::Bool(l.parse::<f32>().unwrap() > r.parse::<f32>().unwrap())
        }
        _ => unimplemented!(),
    })
}

fn binary_less(lhs: Expr, rhs: Expr, funcs: &mut FuncMap) -> Option<Atom> {
    Some(match (eval(lhs, funcs)?, eval(rhs, funcs)?) {
        (Atom::Int(l), Atom::Int(r)) => Atom::Bool(l < r),
        (Atom::Float(l), Atom::Float(r)) => {
            Atom::Bool(l.parse::<f32>().unwrap() < r.parse::<f32>().unwrap())
        }
        _ => unimplemented!(),
    })
}

fn binary_grt_eq(lhs: Expr, rhs: Expr, funcs: &mut FuncMap) -> Option<Atom> {
    Some(match (eval(lhs, funcs)?, eval(rhs, funcs)?) {
        (Atom::Int(l), Atom::Int(r)) => Atom::Bool(l >= r),
        (Atom::Float(l), Atom::Float(r)) => {
            Atom::Bool(l.parse::<f32>().unwrap() >= r.parse::<f32>().unwrap())
        }
        _ => unimplemented!(),
    })
}

fn binary_les_eq(lhs: Expr, rhs: Expr, funcs: &mut FuncMap) -> Option<Atom> {
    Some(match (eval(lhs, funcs)?, eval(rhs, funcs)?) {
        (Atom::Int(l), Atom::Int(r)) => Atom::Bool(l <= r),
        (Atom::Float(l), Atom::Float(r)) => {
            Atom::Bool(l.parse::<f32>().unwrap() <= r.parse::<f32>().unwrap())
        }
        _ => unimplemented!(),
    })
}

fn binary_eq(lhs: Expr, rhs: Expr, funcs: &mut FuncMap) -> Option<Atom> {
    Some(match (eval(lhs, funcs)?, eval(rhs, funcs)?) {
        (Atom::Int(l), Atom::Int(r)) => Atom::Bool(l == r),
        (Atom::Float(l), Atom::Float(r)) => {
            Atom::Bool(l.parse::<f32>().unwrap() == r.parse::<f32>().unwrap())
        }
        a => unimplemented!("'{a:?}'"),
    })
}

fn app_print(args: &[Expr], funcs: &mut FuncMap) -> Option<Atom> {
    let items = args
        .iter()
        .filter_map(|i| eval(i.clone(), funcs))
        .collect::<Vec<Atom>>()
        .iter()
        .map(ToString::to_string)
        .collect::<String>();
    println!("{items}");
    None
}

fn app_call(name: &str, args: &[Expr], funcs: &mut FuncMap) -> Option<Atom> {
    let Some(Function { params, body }) = funcs.get(name) else {
        panic!("this should return an error");
    };
    let mut local = funcs.clone();
    for (param, arg) in params.iter().zip(args) {
        local.insert(
            format!("{param}"),
            Function {
                params: vec![],
                body: arg.clone(),
            },
        );
    }
    eval(body.clone(), &mut local)
}

fn app(head: Expr, args: &[Expr], funcs: &mut FuncMap) -> Option<Atom> {
    match head {
        Expr::Atom(Atom::Id(ref name)) if name == "print" => app_print(args, funcs),
        Expr::Atom(Atom::Id(name)) => app_call(&name, args, funcs),
        Expr::Closure(_head, _tail) => todo!("closure calls in app"),
        _ => {
            let mut local = funcs.clone();
            let (params, body) = seperate_args_from_body(head);
            for (param, arg) in params.iter().zip(args) {
                local.insert(
                    format!("{param}"),
                    Function {
                        params: vec![],
                        body: arg.clone(),
                    },
                );
            }
            return eval(body, &mut local);
        }
    }
}

fn if_else(cond: Expr, b1: Expr, b2: Expr, funcs: &mut FuncMap) -> Option<Atom> {
    Some(match eval(cond, funcs)? {
        Atom::Bool(true) => eval(b1, funcs)?,
        Atom::Bool(false) => eval(b2, funcs)?,
        a => unimplemented!("'{a:?}'"),
    })
}

fn closure(_head: Expr, _tail: Expr, _funcs: &mut FuncMap) -> Option<Atom> {
    todo!()
}

fn function(name: &str, body: Expr, funcs: &mut FuncMap) -> Option<Atom> {
    let (params, body) = seperate_args_from_body(body);
    let func = Function { params, body };
    funcs.insert(name.into(), func);
    None
}
