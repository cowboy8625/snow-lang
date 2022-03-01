use super::error::{CResult, Error, ErrorKind};
use super::parser::{Atom, BuiltIn, Expr, FunctionList};
use super::position::Spanned;
use std::collections::HashMap;

pub fn evaluation(
    expr: &Expr,
    args: &[Spanned<Expr>],
    local: &mut FunctionList,
    funcs: &FunctionList,
) -> CResult<Expr> {
    match expr {
        Expr::Constant(_) => Ok(expr.clone()),
        Expr::Application(head, tail) => {
            let reduced_head = evaluation(head, &[], local, funcs)?;
            let reduced_tail = tail
                .into_iter()
                .map(|expr| Ok((evaluation(&expr.node, &[], local, funcs)?, expr.span()).into()))
                .collect::<CResult<Vec<Spanned<Expr>>>>()?;
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
                    BuiltIn::Eq if is_int(reduced_tail.first().clone()) => {
                        Atom::Boolean(if let Some(first_elem) = reduced_tail.first().cloned() {
                            let fe = get_int_from_expr(first_elem)?;
                            reduced_tail
                                .into_iter()
                                .map(get_int_from_expr)
                                .collect::<CResult<Vec<i32>>>()?
                                .into_iter()
                                .skip(1)
                                .fold((fe, true), |(a, c), b| (b, c && a == b))
                                .1
                        } else {
                            Default::default()
                        })
                    }
                    BuiltIn::Eq => {
                        Atom::Boolean(if let Some(first_elem) = reduced_tail.first().cloned() {
                            let fe = get_float_from_expr(first_elem)?;
                            reduced_tail
                                .into_iter()
                                .map(get_float_from_expr)
                                .collect::<CResult<Vec<f32>>>()?
                                .into_iter()
                                .skip(1)
                                .fold((fe, true), |(a, c), b| (b, c && a == b))
                                .1
                        } else {
                            Default::default()
                        })
                    }
                    BuiltIn::NEq if is_int(reduced_tail.first().clone()) => {
                        Atom::Boolean(if let Some(first_elem) = reduced_tail.first().cloned() {
                            let fe = get_int_from_expr(first_elem)?;
                            reduced_tail
                                .into_iter()
                                .map(get_int_from_expr)
                                .collect::<CResult<Vec<i32>>>()?
                                .into_iter()
                                .skip(1)
                                .fold((fe, true), |(a, c), b| (b, c && a != b))
                                .1
                        } else {
                            Default::default()
                        })
                    }
                    BuiltIn::NEq => {
                        Atom::Boolean(if let Some(first_elem) = reduced_tail.first().cloned() {
                            let fe = get_float_from_expr(first_elem)?;
                            reduced_tail
                                .into_iter()
                                .map(get_float_from_expr)
                                .collect::<CResult<Vec<f32>>>()?
                                .into_iter()
                                .skip(1)
                                .fold((fe, true), |(a, c), b| (b, c && a != b))
                                .1
                        } else {
                            Default::default()
                        })
                    }
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
                        print!("{}", reduced_tail[0].node);
                        return Ok(reduced_tail[0].node.clone());
                    }
                    BuiltIn::PrintLn => {
                        println!("{}", reduced_tail[0].node);
                        return Ok(reduced_tail[0].node.clone());
                    }
                })),
                t => evaluation(&t, &tail, local, funcs),
            }
        }
        Expr::Function(_, prams, body) => {
            let mut local_var = prams
                .iter()
                .zip(args)
                .map(|(k, v)| (k.node.clone(), reduced_expr(v, local, funcs)))
                .collect::<HashMap<String, Spanned<Expr>>>();
            return evaluation(&body.node, &[], &mut local_var, funcs);
        }
        Expr::Local(name) => funcs
            .get(&name.node)
            .map(|s| s.node.clone())
            .or_else(|| local.get(&name.node).map(|s| s.node.clone()))
            .ok_or_else(|| {
                Error::new(
                    &format!("'{}' not found", name),
                    name.span(),
                    ErrorKind::Undefined,
                )
            }),
        Expr::Do(list_expr) => list_expr
            .clone()
            .iter()
            .map(|expr| evaluation(&expr.node, &[], local, funcs))
            .collect::<CResult<Vec<Expr>>>()?
            .last()
            .map(Clone::clone)
            .ok_or(Error::new(
                "nothing to return from do block",
                (list_expr.first(), list_expr.last()).into(),
                ErrorKind::EmptyReturn,
            )),
        Expr::Let(expr, body) => {
            for func in expr.iter() {
                match &func.node {
                    Expr::Function(name, ..) => local.insert(name.node.clone(), func.clone()),
                    x => unreachable!(x),
                };
            }
            Ok(evaluation(&body.node, &[], local, funcs)?)
        }
        Expr::If(condition, body) => {
            let reduced_condition = evaluation(&condition.node, &[], local, funcs)?;
            let cond = get_bool_from_expr((reduced_condition, condition.span()).into())?;
            if cond {
                return evaluation(&body.node, &[], local, funcs);
            }
            Err(Error::new(
                "conditional may need a else statement",
                condition.span(),
                ErrorKind::EmptyReturn,
            ))
        }
        Expr::IfElse(condition, body, r#else) => {
            let reduced_condition = evaluation(&condition.node, &[], local, funcs)?;
            let cond = get_bool_from_expr((reduced_condition, condition.span()).into())?;
            if cond {
                return evaluation(&body.node, &[], local, funcs);
            } else {
                return evaluation(&r#else.node, &[], local, funcs);
            }
        } // _ => unimplemented!(),
    }
}

fn get_int_from_expr(e: Spanned<Expr>) -> CResult<i32> {
    if let Expr::Constant(Atom::Int(n)) = e.node {
        Ok(n)
    } else {
        Err(Error::new(
            &format!("{} is not 'Int'", e),
            e.span(),
            ErrorKind::TypeError,
        ))
    }
}

fn get_float_from_expr(e: Spanned<Expr>) -> CResult<f32> {
    if let Expr::Constant(Atom::Float(n)) = e.node {
        Ok(n)
    } else {
        Err(Error::new(
            &format!("{} is not 'Float'", e),
            e.span(),
            ErrorKind::TypeError,
        ))
    }
}

fn get_bool_from_expr(e: Spanned<Expr>) -> CResult<bool> {
    if let Expr::Constant(Atom::Boolean(b)) = e.node {
        Ok(b)
    } else {
        Err(Error::new(
            &format!("{} is not 'Boolean'", e),
            e.span(),
            ErrorKind::TypeError,
        ))
    }
}

fn is_int(oe: Option<&Spanned<Expr>>) -> bool {
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
        Expr::Local(name) => funcs
            .get(&name.node)
            .map(Clone::clone)
            .or_else(|| local.get(&name.node).map(Clone::clone))
            .unwrap_or(spanned.clone()),
        Expr::Constant(_) => spanned.clone(),
        _ => (
            evaluation(&spanned.node.clone(), &[], local, funcs).unwrap_or(spanned.node.clone()),
            spanned.span(),
        )
            .into(),
    }
}
