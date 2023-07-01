mod error;
#[cfg(test)]
mod tests;
mod value;
pub use error::RuntimeError;
use snowc_parse::{Atom, Expr, Op, Span};
use std::collections::HashMap;
use value::Value;

type Env = HashMap<String, Expr>;
type Result<T> = std::result::Result<T, RuntimeError>;

fn builtin(op: Op) -> Expr {
    let span = Span::default();
    let lhs = Expr::Atom(Atom::Id("x".into()), span);
    let rhs = Expr::Atom(Atom::Id("y".into()), span);
    let body = Expr::Binary(op, Box::new(lhs.clone()), Box::new(rhs.clone()), span);
    let x = Expr::Closure(Box::new(lhs), Box::new(body), span);
    Expr::Closure(Box::new(rhs), Box::new(x), span)
}

#[derive(Debug, Clone)]
pub struct Scope {
    pub local: Env,
    pub global: Env,
}

impl Scope {
    pub fn get(&self, name: &str) -> Option<&Expr> {
        self.local.get(name).or(self.global.get(name))
    }

    fn insert_global(&mut self, k: String, v: Expr) {
        self.global.insert(k, v);
    }
    fn insert_local(&mut self, k: String, v: Expr) {
        self.local.insert(k, v);
    }
}
impl Default for Scope {
    fn default() -> Self {
        let mut scope = Self {
            local: Env::default(),
            global: Env::default(),
        };
        scope.insert_global("(+)".into(), builtin(Op::Plus));
        scope.insert_global("(-)".into(), builtin(Op::Minus));
        scope.insert_global("(*)".into(), builtin(Op::Mult));
        scope.insert_global("(/)".into(), builtin(Op::Div));
        scope
    }
}

fn expr_unary(op: &Op, rhs: &Expr, _span: Span, scope: &Scope) -> Result<Value> {
    let atom = walk_expr(rhs, scope)?;
    match (op, atom) {
        (Op::Minus, Value::Int(int, span)) => Ok(Value::Int(-int, span)),
        (Op::Not, Value::Bool(b, span)) => Ok(Value::Bool(!b, span)),
        _ => unimplemented!("for operator '{op:?}'"),
    }
}

fn expr_binary(
    op: &Op,
    lhs: &Expr,
    rhs: &Expr,
    span: Span,
    scope: &Scope,
) -> Result<Value> {
    let lhs_atom = walk_expr(lhs, scope)?;
    let rhs_atom = walk_expr(rhs, scope)?;
    match (op, lhs_atom, rhs_atom) {
        (Op::Plus, Value::Int(lhs, ..), Value::Int(rhs, ..)) => {
            Ok(Value::Int(lhs + rhs, span))
        }
        (Op::Minus, Value::Int(lhs, ..), Value::Int(rhs, ..)) => {
            Ok(Value::Int(lhs - rhs, span))
        }
        (Op::Mult, Value::Int(lhs, ..), Value::Int(rhs, ..)) => {
            Ok(Value::Int(lhs * rhs, span))
        }
        (Op::Div, Value::Int(lhs, ..), Value::Int(rhs, ..)) => {
            Ok(Value::Int(lhs / rhs, span))
        }
        (Op::Mod, Value::Int(lhs, ..), Value::Int(rhs, ..)) => {
            Ok(Value::Int(lhs % rhs, span))
        }
        (Op::Grt, Value::Int(lhs, ..), Value::Int(rhs, ..)) => {
            Ok(Value::Bool(lhs > rhs, span))
        }
        (Op::GrtEq, Value::Int(lhs, ..), Value::Int(rhs, ..)) => {
            Ok(Value::Bool(lhs >= rhs, span))
        }
        (Op::Les, Value::Int(lhs, ..), Value::Int(rhs, ..)) => {
            Ok(Value::Bool(lhs < rhs, span))
        }
        (Op::LesEq, Value::Int(lhs, ..), Value::Int(rhs, ..)) => {
            Ok(Value::Bool(lhs <= rhs, span))
        }
        (Op::Eq, Value::Int(lhs, ..), Value::Int(rhs, ..)) => {
            Ok(Value::Bool(lhs == rhs, span))
        }
        (Op::Neq, Value::Int(lhs, ..), Value::Int(rhs, ..)) => {
            Ok(Value::Bool(lhs != rhs, span))
        }
        (Op::Eq, Value::String(lhs, ..), Value::String(rhs, ..)) => {
            Ok(Value::Bool(lhs == rhs, span))
        }
        (Op::Neq, Value::String(lhs, ..), Value::String(rhs, ..)) => {
            Ok(Value::Bool(lhs != rhs, span))
        }
        (Op::And, Value::Bool(lhs, ..), Value::Bool(rhs, ..)) => {
            Ok(Value::Bool(lhs && rhs, span))
        }
        (Op::Or, Value::Bool(lhs, ..), Value::Bool(rhs, ..)) => {
            Ok(Value::Bool(lhs || rhs, span))
        }
        (op, r, l) => unimplemented!("{l:?} {op:?} {r:?}"),
    }
}

fn expr_conditional(
    condition: &Expr,
    then: &Expr,
    r#else: &Expr,
    scope: &Scope,
) -> Result<Value> {
    match walk_expr(condition, scope)? {
        Value::Bool(true, _) => walk_expr(then, scope),
        Value::Bool(false, _) => walk_expr(r#else, scope),
        _ => unreachable!(),
    }
}

fn expr_closure_with_args(
    head: &Expr,
    tail: &Expr,
    args: &[Expr],
    scope: &Scope,
) -> Result<Value> {
    let Some(arg) = args.first() else {
        unimplemented!("not sure what to do here");
    };
    let Expr::Atom(Atom::Id(name), span) = head else {
        unimplemented!("shouldnt get here i think?");
    };
    let mut scope = scope.clone();
    scope.insert_local(name.clone(), arg.clone());
    if tail.is_app() {
        return expr_app(tail, &args[1..], *span, &scope);
    }
    walk_expr(tail, &scope)
}

fn expr_app(name: &Expr, args: &[Expr], _span: Span, scope: &Scope) -> Result<Value> {
    let Expr::Atom(Atom::Id(name), span) = name else {
        let (Some(head), Some(tail)) = (name.get_head(), name.get_tail()) else {
            let Expr::App(name, args, span) = name else {
                unimplemented!("return a run time error");
            };
            return expr_app(name, args, *span, scope);
        };
        return expr_closure_with_args(head, tail, args, scope);
    };
    match name.as_str() {
        "print" => {
            let mut eval_args = vec![];
            for expr in args.iter() {
                let value = walk_expr(expr, scope)?;
                eval_args.push(value);
            }
            let formated =
                eval_args
                    .iter()
                    .enumerate()
                    .fold("".into(), |acc, (idx, item)| {
                        let item = match item {
                            Value::Array(array, ..) => {
                                array.iter().map(ToString::to_string).collect::<String>()
                            }
                            _ => item.to_string(),
                        };
                        if idx == 0 {
                            return item;
                        }
                        format!("{acc} {item}")
                    });
            print!("{formated}");
            Ok(eval_args[0].clone())
        }
        "nth" => {
            let atom = walk_expr(&args[0], scope)?;
            let Value::Array(array, span) = atom else {
                return Err(RuntimeError::InvalidArguments(*span));
            };
            let atom = walk_expr(&args[1], scope)?;
            let Value::Int(idx, span) = &atom else {
                return Err(RuntimeError::InvalidArguments(span));
            };
            let idx = *idx as usize;
            if idx >= array.len() {
                return Err(RuntimeError::IdxOutOfBounds(*span));
            }
            let atom = &array[idx];
            Ok(atom.clone())
        }
        "length" => {
            let Value::Array(array, span) = walk_expr(&args[0], scope)? else {
                return Err(RuntimeError::InvalidArguments(*span));
            };
            let len = array.len();
            Ok(Value::Int(len as i32, span))
        }
        "push" => {
            let Value::Array(mut array, span) = walk_expr(&args[0], scope)? else {
                return Err(RuntimeError::InvalidArguments(*span));
            };
            let value = walk_expr(&args[1], scope)?;
            array.push(value);
            Ok(Value::Array(array, span))
        }
        _ => {
            let Some(mut func) = scope.get(name) else {
                return Err(RuntimeError::Undefined(name.into(), *span));
            };
            let mut scope = scope.clone();
            for arg in args.iter() {
                let Some(Expr::Atom(Atom::Id(name), ..)) = func.get_head() else {
                    continue;
                };
                let value = walk_expr(&arg.clone(), &scope)?;
                fn into_expr(v: &Value) -> Expr {
                    match v {
                        Value::Int(i, span) => Expr::Atom(Atom::Int(*i), *span),
                        Value::Float(i, span) => {
                            Expr::Atom(Atom::Float(i.clone()), *span)
                        }
                        Value::Bool(i, span) => Expr::Atom(Atom::Bool(*i), *span),
                        Value::String(i, span) => {
                            Expr::Atom(Atom::String(i.clone()), *span)
                        }
                        Value::Char(i, span) => Expr::Atom(Atom::Char(*i), *span),
                        Value::Array(array, span) => {
                            let array = array.iter().map(into_expr).collect();
                            Expr::Array(array, *span)
                        }
                        _ => unreachable!("{v}"),
                    }
                }
                scope.insert_local(name.clone(), into_expr(&value));
                let Some(t) = func.get_tail() else {
                    unreachable!()
                };
                func = t;
            }
            walk_expr(func, &scope)
        }
    }
}

fn expr_closure(head: &Expr, tail: &Expr, scope: &Scope) -> Result<Value> {
    walk_expr(head, scope)?;
    walk_expr(tail, scope)
}

fn walk_atom(atom: &Atom, span: Span, scope: &Scope) -> Result<Value> {
    match atom {
        Atom::Id(name) => {
            let Some(expr) = scope.get(name) else {
                return Err(RuntimeError::Undefined(name.into(), span));
            };
            walk_expr(expr, scope)
        }
        Atom::Int(i) => Ok(Value::Int(*i, span)),
        Atom::Float(f) => Ok(Value::Float(f.clone(), span)),
        Atom::Bool(b) => Ok(Value::Bool(*b, span)),
        Atom::String(string) => Ok(Value::String(string.clone(), span)),
        Atom::Char(c) => Ok(Value::Char(*c, span)),
    }
}

fn walk_expr(expr: &Expr, scope: &Scope) -> Result<Value> {
    match expr {
        Expr::Atom(atom, span) => walk_atom(atom, *span, scope),
        Expr::Unary(op, rhs, span) => expr_unary(op, rhs, *span, scope),
        Expr::Binary(op, lhs, rhs, span) => expr_binary(op, lhs, rhs, *span, scope),
        Expr::IfElse(condition, then, r#else, ..) => {
            expr_conditional(condition, then, r#else, scope)
        }
        Expr::Closure(head, tail, ..) => expr_closure(head, tail, scope),
        Expr::App(name, args, span) => expr_app(name, args, *span, scope),
        Expr::Array(array, ..) => {
            let mut result = vec![];
            let start_span = array.first().map(|e| e.span()).unwrap_or_default();
            let end_span = array.last().map(|e| e.span()).unwrap_or_default();
            let span = Span::from((start_span, end_span));
            for e in array.iter() {
                let expr = walk_expr(e, scope)?;
                result.push(expr);
            }
            Ok(Value::Array(result, span))
        }
        Expr::Enum(..) => unimplemented!("enum"),

        // this should only be for type checker.
        Expr::TypeDec(..) => unreachable!("type dec"),
        // should never get to theres
        Expr::Func(..) => unreachable!("func"),
        Expr::Error(..) => unreachable!("error"),
    }
}

pub fn walk(ast: &[Expr]) -> std::result::Result<Option<Value>, Vec<RuntimeError>> {
    let mut scope = Scope::default();
    let mut main_idx: Option<usize> = None;
    for (idx, expr) in ast.iter().enumerate() {
        match expr {
            Expr::Func(name, ..) if name == "main" => {
                main_idx = Some(idx);
            }
            Expr::Func(name, closure, ..) => {
                scope.insert_global(name.to_string(), *closure.clone());
            }
            Expr::TypeDec(..) => {}
            _ => unreachable!(),
        }
    }

    let Some(idx) = main_idx else {
        return Err(vec![RuntimeError::MissingMainFunction]);
    };

    let mut errors: Vec<RuntimeError> = Vec::new();
    if !errors.is_empty() {
        return Err(errors);
    }
    let main_function = &ast[idx];
    let Expr::Func(_, closure, ..) = main_function else {
        panic!("really bad things are happening");
    };
    match walk_expr(closure, &scope) {
        Ok(v) => Ok(Some(v)),
        Err(err) => {
            errors.push(err);
            Err(errors)
        }
    }
}

pub fn eval_expr_with_scope(
    expr: &Expr,
    scope: &mut Scope,
) -> std::result::Result<Option<Value>, RuntimeError> {
    match expr {
        Expr::Func(name, closure, ..) => {
            scope.insert_global(name.to_string(), *closure.clone());
            Ok(None)
        }
        Expr::TypeDec(..) => Ok(None),
        _ => walk_expr(expr, scope).map(Some),
    }
}
