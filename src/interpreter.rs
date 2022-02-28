#![allow(unused)]
use super::error::{CResult, Error, ErrorKind};
use super::parser::{Atom, BuiltIn, Expr, FunctionList};
use super::position::{Span, Spanned};
use std::collections::HashMap;
use std::fmt;

pub fn evaluation(expr: &Expr, local: &mut FunctionList, funcs: &FunctionList) -> CResult<Expr> {
    match expr {
        Expr::Constant(contant) => Ok(expr.clone()),
        // func-name args
        Expr::Application(head, tail) => {
            let reduced_head = evaluation(head, local, funcs)?;
            let reduced_tail = tail
                .into_iter()
                .map(|expr| evaluation(&expr.node, local, funcs))
                .collect::<CResult<Vec<Expr>>>()?;
            // dbg!(&tail);
            match reduced_head {
                Expr::Constant(Atom::BuiltIn(bi)) => Ok(Expr::Constant(match bi {
                    BuiltIn::Plus if is_int(reduced_tail.first().clone()) => Atom::Int(
                        reduced_tail
                            .into_iter()
                            .map(get_int_from_expr)
                            .collect::<CResult<Vec<i32>>>()?
                            .into_iter()
                            .sum(),
                    ),
                    BuiltIn::Plus => Atom::Float(
                        reduced_tail
                            .into_iter()
                            .map(get_float_from_expr)
                            .collect::<CResult<Vec<f32>>>()?
                            .into_iter()
                            .sum(),
                    ),
                    BuiltIn::Mult if is_int(reduced_tail.first().clone()) => Atom::Int(
                        reduced_tail
                            .into_iter()
                            .map(get_int_from_expr)
                            .collect::<CResult<Vec<i32>>>()?
                            .into_iter()
                            .product(),
                    ),
                    BuiltIn::Mult => Atom::Float(
                        reduced_tail
                            .into_iter()
                            .map(get_float_from_expr)
                            .collect::<CResult<Vec<f32>>>()?
                            .into_iter()
                            .product(),
                    ),
                    BuiltIn::Mins if is_int(reduced_tail.first().clone()) => {
                        Atom::Int(if let Some(first_elem) = reduced_tail.first().cloned() {
                            let fe = get_int_from_expr(first_elem)?;
                            reduced_tail
                                .into_iter()
                                .map(get_int_from_expr)
                                .collect::<CResult<Vec<i32>>>()?
                                .into_iter()
                                .skip(1)
                                .fold(fe, |a, b| a - b)
                        } else {
                            Default::default()
                        })
                    }
                    BuiltIn::Mins => {
                        Atom::Float(if let Some(first_elem) = reduced_tail.first().cloned() {
                            let fe = get_float_from_expr(first_elem)?;
                            reduced_tail
                                .into_iter()
                                .map(get_float_from_expr)
                                .collect::<CResult<Vec<f32>>>()?
                                .into_iter()
                                .skip(1)
                                .fold(fe, |a, b| a - b)
                        } else {
                            Default::default()
                        })
                    }
                    BuiltIn::Div if is_int(reduced_tail.first().clone()) => {
                        Atom::Int(if let Some(first_elem) = reduced_tail.first().cloned() {
                            let fe = get_int_from_expr(first_elem)?;
                            reduced_tail
                                .into_iter()
                                .map(get_int_from_expr)
                                .collect::<CResult<Vec<i32>>>()?
                                .into_iter()
                                .skip(1)
                                .fold(fe, |a, b| a / b)
                        } else {
                            Default::default()
                        })
                    }
                    BuiltIn::Div => {
                        Atom::Float(if let Some(first_elem) = reduced_tail.first().cloned() {
                            let fe = get_float_from_expr(first_elem)?;
                            reduced_tail
                                .into_iter()
                                .map(get_float_from_expr)
                                .collect::<CResult<Vec<f32>>>()?
                                .into_iter()
                                .skip(1)
                                .fold(fe, |a, b| a / b)
                        } else {
                            Default::default()
                        })
                    }
                    BuiltIn::Eq => Atom::Boolean(
                        reduced_tail
                            .iter()
                            .zip(reduced_tail.iter().skip(1))
                            .all(|(a, b)| a == b),
                    ),
                    BuiltIn::NEq => Atom::Boolean(
                        reduced_tail
                            .iter()
                            .zip(reduced_tail.iter().skip(1))
                            .all(|(a, b)| a != b),
                    ),
                    BuiltIn::Not => {
                        if reduced_tail.len() != 1 {
                            return Err("Nothing on Rhs.".into());
                        } else {
                            Atom::Boolean(!get_bool_from_expr(
                                reduced_tail.first().cloned().unwrap(),
                            )?)
                        }
                    }
                    BuiltIn::Print => {
                        print!("{}", reduced_tail[0]);
                        return Ok(reduced_tail[0].clone());
                    }
                    BuiltIn::PrintLn => {
                        println!("{}", reduced_tail[0]);
                        return Ok(reduced_tail[0].clone());
                    }
                })),
                Expr::Function(_, prams, body) => {
                    let mut local_var = prams
                        .iter()
                        .zip(tail)
                        .map(|(k, v)| (k.node.clone(), reduced_expr(v, local, funcs)))
                        .collect::<HashMap<String, Spanned<Expr>>>();
                    return evaluation(&body.node, &mut local_var, funcs);
                }
                t => Ok(t),
            }
        }
        // func-name prams body
        Expr::Function(name, prams, body) => unreachable!(),
        // func name's or pram name's
        Expr::Local(name) => funcs
            .get(name)
            .map(|s| s.node.clone())
            .or_else(|| local.get(name).map(|s| s.node.clone()))
            .ok_or_else(|| {
                Error::new(
                    &format!("'{}' not found", name),
                    Span::default(),
                    ErrorKind::Undefined,
                )
            }),
        Expr::Do(list_expr) => list_expr
            .iter()
            .map(|expr| evaluation(&expr.node, local, funcs))
            .collect::<CResult<Vec<Expr>>>()?
            .last()
            .map(Clone::clone)
            .ok_or(Error::new(
                "nothing to return from do block",
                Span::default(),
                ErrorKind::EmptyReturn,
            )),
        Expr::Let(name, expr, body) => {
            let expr = reduced_expr(expr, local, funcs);
            local.insert(name.to_string(), expr);
            Ok(evaluation(&body.node, local, funcs)?)
        }
        _ => unimplemented!(),
    }
}

// TODO: should expect a span.
fn get_int_from_expr(e: Expr) -> CResult<i32> {
    if let Expr::Constant(Atom::Int(n)) = e {
        Ok(n)
    } else {
        Err(Error::new(
            &format!("{} is not 'Int'", e),
            Span::default(),
            ErrorKind::TypeError,
        ))
    }
}

fn get_float_from_expr(e: Expr) -> CResult<f32> {
    if let Expr::Constant(Atom::Float(n)) = e {
        Ok(n)
    } else {
        Err(Error::new(
            &format!("{} is not 'Float'", e),
            Span::default(),
            ErrorKind::TypeError,
        ))
    }
}

fn get_bool_from_expr(e: Expr) -> CResult<bool> {
    if let Expr::Constant(Atom::Boolean(b)) = e {
        Ok(b)
    } else {
        Err(Error::new(
            &format!("{} is not 'Boolean'", e),
            Span::default(),
            ErrorKind::TypeError,
        ))
    }
}

fn is_int(oe: Option<&Expr>) -> bool {
    if let Some(v) = oe {
        return get_int_from_expr(v.clone()).is_ok();
    }
    false
}

fn reduced_expr(
    spanned: &Spanned<Expr>,
    local: &mut FunctionList,
    funcs: &FunctionList,
) -> Spanned<Expr> {
    match &spanned.node {
        Expr::Local(n) => funcs
            .get(n)
            .map(Clone::clone)
            .or_else(|| local.get(n).map(Clone::clone))
            .unwrap(),
        Expr::Constant(_) => spanned.clone(),
        _ => (
            evaluation(&spanned.node.clone(), local, funcs).unwrap(),
            spanned.span(),
        )
            .into(),
    }
}
