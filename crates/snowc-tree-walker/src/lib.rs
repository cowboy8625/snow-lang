mod error;
#[cfg(test)]
mod tests;
mod value;
pub use error::RuntimeError;
use snowc_parse::{
    expr::{App, Binary},
    Atom, Expr, Op, Span, TokenPosition, Unary,
};
use std::collections::HashMap;
pub use value::Value;

type Env = HashMap<String, Expr>;
type Result<T> = std::result::Result<T, RuntimeError>;

fn builtin(op: Op) -> Expr {
    let pos = TokenPosition::Middle;
    let span = Span::default();
    let left = Box::new(Expr::Atom(Atom::Id("x".into(), pos, span)));
    let right = Box::new(Expr::Atom(Atom::Id("y".into(), pos, span)));
    let binary = Binary {
        op,
        left: left.clone(),
        right: right.clone(),
        pos: TokenPosition::End,
        span,
    };
    let body = Expr::Binary(binary);
    let x = Expr::Closure(left, Box::new(body), span);
    Expr::Closure(right, Box::new(x), span)
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

fn expr_unary(unary: &Unary, scope: &Scope) -> Result<Value> {
    let Unary { op, expr, .. } = unary;
    let atom = walk_expr(expr, scope)?;
    match (op, atom) {
        (Op::Minus, Value::Int(int, span)) => Ok(Value::Int(-int, span)),
        (Op::Not, Value::Bool(b, span)) => Ok(Value::Bool(!b, span)),
        _ => unimplemented!("for operator '{op:?}'"),
    }
}

fn expr_binary(binary: &Binary, scope: &Scope) -> Result<Value> {
    let Binary {
        op,
        left,
        right,
        span,
        ..
    } = binary;
    let lhs_atom = walk_expr(left, scope)?;
    let rhs_atom = walk_expr(right, scope)?;
    match (op, lhs_atom, rhs_atom) {
        (Op::Plus, Value::Int(lhs, ..), Value::Int(rhs, ..)) => {
            Ok(Value::Int(lhs + rhs, *span))
        }
        (Op::Plus, Value::String(lhs, ..), Value::String(rhs, ..)) => {
            Ok(Value::String(format!("{lhs}{rhs}"), *span))
        }
        (Op::Plus, Value::Array(lhs, ..), Value::Array(rhs, ..)) => Ok(Value::Array(
            lhs.iter().cloned().chain(rhs.iter().cloned()).collect(),
            *span,
        )),
        (Op::Minus, Value::Int(lhs, ..), Value::Int(rhs, ..)) => {
            Ok(Value::Int(lhs - rhs, *span))
        }
        (Op::Mult, Value::Int(lhs, ..), Value::Int(rhs, ..)) => {
            Ok(Value::Int(lhs * rhs, *span))
        }
        (Op::Div, Value::Int(lhs, ..), Value::Int(rhs, ..)) => {
            Ok(Value::Int(lhs / rhs, *span))
        }
        (Op::Mod, Value::Int(lhs, ..), Value::Int(rhs, ..)) => {
            Ok(Value::Int(lhs % rhs, *span))
        }
        (Op::Grt, Value::Int(lhs, ..), Value::Int(rhs, ..)) => {
            Ok(Value::Bool(lhs > rhs, *span))
        }
        (Op::GrtEq, Value::Int(lhs, ..), Value::Int(rhs, ..)) => {
            Ok(Value::Bool(lhs >= rhs, *span))
        }
        (Op::Les, Value::Int(lhs, ..), Value::Int(rhs, ..)) => {
            Ok(Value::Bool(lhs < rhs, *span))
        }
        (Op::LesEq, Value::Int(lhs, ..), Value::Int(rhs, ..)) => {
            Ok(Value::Bool(lhs <= rhs, *span))
        }
        (Op::Eq, Value::Int(lhs, ..), Value::Int(rhs, ..)) => {
            Ok(Value::Bool(lhs == rhs, *span))
        }
        (Op::Neq, Value::Int(lhs, ..), Value::Int(rhs, ..)) => {
            Ok(Value::Bool(lhs != rhs, *span))
        }
        (Op::Eq, Value::String(lhs, ..), Value::String(rhs, ..)) => {
            Ok(Value::Bool(lhs == rhs, *span))
        }
        (Op::Neq, Value::String(lhs, ..), Value::String(rhs, ..)) => {
            Ok(Value::Bool(lhs != rhs, *span))
        }
        (Op::And, Value::Bool(lhs, ..), Value::Bool(rhs, ..)) => {
            Ok(Value::Bool(lhs && rhs, *span))
        }
        (Op::Or, Value::Bool(lhs, ..), Value::Bool(rhs, ..)) => {
            Ok(Value::Bool(lhs || rhs, *span))
        }
        (_, _, _) => Err(RuntimeError::InvalidBinaryOp(*span)),
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
    let Expr::Atom(Atom::Id(name, _, span)) = head else {
        unimplemented!("shouldnt get here i think?");
    };
    let mut scope = scope.clone();
    scope.insert_local(name.clone(), arg.clone());
    if tail.is_app() {
        return expr_app(tail, &args[1..], *span, &scope);
    }
    walk_expr(tail, &scope)
}

fn expr_app(expr: &Expr, args: &[Expr], span: Span, scope: &Scope) -> Result<Value> {
    let Expr::Atom(Atom::Id(name, _, span)) = expr else {
        let (Some(head), Some(tail)) = (expr.get_head(), expr.get_tail()) else {
            let Expr::App(App { name, args, span, .. }) = expr else {
                return Err(RuntimeError::InvalidArguments(span));
            };
            return expr_app(name, args, *span, scope);
        };
        return expr_closure_with_args(head, tail, args, scope);
    };
    match name.as_str() {
        // Prints any item to console
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
                            Value::Array(array, ..) => format!(
                                "{}]",
                                array.iter().fold("[".into(), |acc, item| {
                                    if acc == "[" {
                                        return format!("{acc}{item}");
                                    }
                                    format!("{acc}, {item}")
                                })
                            ),
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
        // use this function to index into an array
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
        // use this function to get the length of an array
        "length" => {
            let Value::Array(array, span) = walk_expr(&args[0], scope)? else {
                return Ok(Value::Int(0, *span))
            };
            let len = array.len();
            Ok(Value::Int(len as i32, span))
        }
        // use this function to push to the end of an array
        "push" => {
            let lhs = walk_expr(&args[0], scope)?;
            let rhs = walk_expr(&args[1], scope)?;
            match (lhs, rhs) {
                (Value::String(mut string, span), Value::String(value, ..)) => {
                    string.push_str(&value);
                    Ok(Value::String(string, span))
                }
                (Value::Array(mut array, span), value) => {
                    array.push(value);
                    Ok(Value::Array(array, span))
                }
                _ => Err(RuntimeError::InvalidArguments(*span)),
            }
        }
        "tail" => {
            let iter = walk_expr(&args[0], scope)?;
            match iter {
                Value::String(string, span) => {
                    if string.is_empty() {
                        return Ok(Value::String(String::new(), span));
                    }
                    Ok(Value::String(string[1..].to_string(), span))
                }
                Value::Array(array, span) => {
                    if array.is_empty() {
                        return Ok(Value::Array(vec![], span));
                    }
                    Ok(Value::Array(array[1..].to_vec(), span))
                }
                _ => Err(RuntimeError::InvalidArguments(*span)),
            }
        }
        "head" => {
            let iter = walk_expr(&args[0], scope)?;
            match iter {
                Value::String(string, span) => {
                    if string.is_empty() {
                        return Err(RuntimeError::EmptyArray(span));
                    }
                    Ok(Value::String(string[0..1].to_string(), span))
                }
                Value::Array(array, span) => {
                    if array.is_empty() {
                        return Err(RuntimeError::EmptyArray(span));
                    }
                    Ok(array[0].clone())
                }
                _ => Err(RuntimeError::InvalidArguments(*span)),
            }
        }
        _ => {
            let Some(mut func) = scope.get(name).cloned() else {
                return walk_expr(expr, scope);
            };
            let mut scope = scope.clone();
            for arg in args.iter() {
                match func.get_head() {
                    Some(Expr::Atom(Atom::Id(name, pos, ..))) => {
                        let value = match walk_expr(&arg.clone(), &scope) {
                            Ok(v) => into_expr(&v, *pos),
                            Err(_) => arg.clone(),
                        };
                        scope.insert_local(name.clone(), value);
                        let Some(t) = func.get_tail() else {
                            unreachable!()
                        };
                        func = t.clone();
                    }
                    _ => {
                        let value = match walk_expr(&func, &scope) {
                            Ok(v) => into_expr(&v, arg.position()),
                            Err(_) => arg.clone(),
                        };
                        func = value;
                    }
                }
                // let Some(Expr::Atom(Atom::Id(name, pos, ..))) = func.get_head() else {
                //     return walk_expr(expr, &scope);
                //     // eprintln!("func: {} | type: {}", func, _typeofexpr(func));
                //     // eprintln!("name: {} | type: {}", name, "id");
                //     // let Some(add_one) = scope.get(&func.to_string()) else {
                //     //     continue;
                //     // };
                //     // eprintln!("addOne: {} | type: {}", add_one, _typeofexpr(add_one));
                //     // eprintln!("arg: {} | type: {}", arg, _typeofexpr(arg));
                //     // let value = match walk_expr(&arg.clone(), &scope) {
                //     //     Ok(v) => into_expr(&v, arg.position()),
                //     //     Err(_) => arg.clone(),
                //     // };
                //     // eprintln!("value: {} | type: {}", value, _typeofexpr(&value));
                //     // continue;
                // };
                // let value = match walk_expr(&arg.clone(), &scope) {
                //     Ok(v) => into_expr(&v, *pos),
                //     Err(_) => arg.clone(),
                // };
                // scope.insert_local(name.clone(), value);
                // let Some(t) = func.get_tail() else {
                //     unreachable!()
                // };
                // func = t;
            }

            walk_expr(&func, &scope)
        }
    }
}
fn _typeofexpr(expr: &Expr) -> String {
    match expr {
        Expr::Atom(atom) => match atom {
            Atom::Id(..) => "id".to_string(),
            Atom::Int(..) => "int".to_string(),
            Atom::Float(..) => "float".to_string(),
            Atom::Bool(..) => "bool".to_string(),
            Atom::String(..) => "string".to_string(),
            Atom::Char(..) => "char".to_string(),
        },
        Expr::Array(..) => "array".to_string(),
        Expr::Closure(..) => "closure".to_string(),
        Expr::Func(..) => "function".to_string(),
        Expr::Error(..) => "error".to_string(),
        Expr::App(..) => "app".to_string(),
        Expr::Unary(..) => "unary".to_string(),
        Expr::Binary(..) => "binary".to_string(),
        Expr::IfElse(..) => "if".to_string(),
        Expr::Enum(..) => "enum".to_string(),
    }
}
fn into_expr(v: &Value, pos: TokenPosition) -> Expr {
    match v {
        Value::Int(i, span) => Expr::Atom(Atom::Int(*i, pos, *span)),
        Value::Float(i, span) => Expr::Atom(Atom::Float(i.clone(), pos, *span)),
        Value::Bool(i, span) => Expr::Atom(Atom::Bool(*i, pos, *span)),
        Value::String(i, span) => Expr::Atom(Atom::String(i.clone(), pos, *span)),
        Value::Char(i, span) => Expr::Atom(Atom::Char(*i, pos, *span)),
        Value::Array(array, span) => {
            let array = array.iter().map(|v| into_expr(v, pos)).collect();
            Expr::Array(array, pos, *span)
        }
        _ => unreachable!("{v}"),
    }
}

fn expr_closure(head: &Expr, tail: &Expr, scope: &Scope) -> Result<Value> {
    walk_expr(head, scope)?;
    walk_expr(tail, scope)
}

fn walk_atom(atom: &Atom, scope: &Scope) -> Result<Value> {
    match atom {
        Atom::Id(name, _, span) => {
            let Some(expr) = scope.get(name) else {
                return Err(RuntimeError::Undefined(name.into(), *span));
            };
            walk_expr(expr, scope)
        }
        Atom::Int(i, _, span) => Ok(Value::Int(*i, *span)),
        Atom::Float(f, _, span) => Ok(Value::Float(f.clone(), *span)),
        Atom::Bool(b, _, span) => Ok(Value::Bool(*b, *span)),
        Atom::String(string, _, span) => Ok(Value::String(string.clone(), *span)),
        Atom::Char(c, _, span) => Ok(Value::Char(*c, *span)),
    }
}

fn walk_expr(expr: &Expr, scope: &Scope) -> Result<Value> {
    match expr {
        Expr::Atom(atom) => walk_atom(atom, scope),
        Expr::Unary(unary) => expr_unary(unary, scope),
        Expr::Binary(binary) => expr_binary(binary, scope),
        Expr::IfElse(condition, then, r#else, ..) => {
            expr_conditional(condition, then, r#else, scope)
        }
        Expr::Closure(head, tail, ..) => expr_closure(head, tail, scope),
        Expr::App(App {
            name, args, span, ..
        }) => expr_app(name, args, *span, scope),
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
            Expr::Func(name, _, closure, ..) => {
                scope.insert_global(name.to_string(), *closure.clone());
            }
            _ => unreachable!("{:?}", expr),
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
    let Expr::Func(_, _, closure, ..) = main_function else {
        panic!("maybe you added a new prameter to Expr::Func?");
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
        Expr::Func(name, _, closure, ..) => {
            scope.insert_global(name.to_string(), *closure.clone());
            Ok(None)
        }
        _ => walk_expr(expr, scope).map(Some),
    }
}

#[test]
fn test_walk() {
    use pretty_assertions::assert_eq;
    use snowc_parse::parse;
    let src = include_str!("./../../../samples/other.snow");
    let ast = parse(src);
    let result = walk(&ast.unwrap()).unwrap();
    assert_eq!(result, None);
}
