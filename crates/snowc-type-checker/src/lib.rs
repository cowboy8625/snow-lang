use snowc_errors::{bail, CResult, CompilerError};
use snowc_parse::{Atom, Expr, Op};

type Types = std::collections::HashMap<String, TypedFunc>;
type Span = std::ops::Range<usize>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Type {
    Int,
    Float,
    Bool,
    String,
    Char,
    IO,
}

impl TryFrom<&String> for Type {
    type Error = Box<dyn std::error::Error>;
    fn try_from(t: &String) -> Result<Self, Self::Error> {
        match t.as_str() {
            "Int" => Ok(Self::Int),
            "Float" => Ok(Self::Float),
            "Bool" => Ok(Self::Bool),
            "String" => Ok(Self::String),
            "Char" => Ok(Self::Char),
            "IO" => Ok(Self::IO),
            _ => bail!(0..0, "unknown type '{t}'"),
        }
    }
}

#[derive(Debug, Clone, Hash)]
struct TypedFunc {
    return_type: Type,
    args: Vec<(Option<String>, Type)>,
}

impl TypedFunc {
    // fn new(return_type: Type) -> Self {
    //     Self::new_with_args(return_type, vec![])
    // }
    //
    fn new_with_args(return_type: Type, args: Vec<(Option<String>, Type)>) -> Self {
        Self { return_type, args }
    }

    fn push_arg(&mut self, name: impl Into<String>) {
        for (param, _type) in self.args.iter_mut() {
            if param.is_none() {
                *param = Some(name.into());
                break;
            }
        }
    }

    fn lookup(&self, name: impl Into<String>) -> Option<Type> {
        let name = name.into();
        self.args
            .iter()
            .find(|(arg, _)| arg.as_ref().map(|i| i == &name).unwrap_or(false))
            .map(|(_, t)| t.clone())
    }
}

fn lookup(func_name: &str, env: &Types, id: &str) -> CResult<Type> {
    match env.get(id) {
        Some(type_func) => Ok(type_func.return_type),
        None => match env.get(func_name).map(|i| i.lookup(id)).flatten() {
            Some(t) => Ok(t),
            None => bail!(0..0, "unbound error '{id}' has never been created"),
        },
    }
}

fn type_check_binary<'a>(
    func_name: &str,
    env: &Types,
    op: &'a Op,
    lhs: &'a Expr,
    rhs: &'a Expr,
) -> CResult<Type> {
    let t1 = type_of(func_name, env, lhs)?;
    let t2 = type_of(func_name, env, rhs)?;
    if t1 != t2 {
        bail!(
            0..0,
            "type miss matched '{op:?}' lhs: '{t1:?}', rhs: '{t2:?}'"
        );
    }
    match op {
        Op::Plus | Op::Minus | Op::Mult | Op::Div => Ok(t1),
        Op::Grt | Op::Les | Op::GrtEq | Op::LesEq | Op::Eq | Op::Neq | Op::Not => {
            Ok(Type::Bool)
        }
        Op::Equals | Op::Pipe => {
            let span = lhs.span().start..rhs.span().end;
            bail!(span, "not yet implemented for assignment or pipe")
        }
    }
}

fn type_check_if_else<'a>(
    func_name: &str,
    env: &Types,
    c: &'a Expr,
    b1: &'a Expr,
    b2: &'a Expr,
) -> CResult<Type> {
    let tc = type_of(func_name, env, c)?;
    let Type::Bool = tc else {
        bail!(c.span(),  "if condition found '{tc:?}' but expected 'Bool' Type");
    };
    let t1 = type_of(func_name, env, b1)?;
    let t2 = type_of(func_name, env, b2)?;
    if t1 != t2 {
        bail!(
            b1.span().start..b2.span().end,
            "branch types do not match, expected '{t1:?}' but found '{t2:?}'"
        );
    }
    Ok(t1)
}

fn type_check_app<'a>(
    func_name: &str,
    env: &Types,
    name: &'a Expr,
    args: &[Expr],
    span: &Span,
) -> CResult<Type> {
    let t = type_of(func_name, env, name)?;
    let Expr::Atom(Atom::Id(name), _) = name else {
        return Ok(t);
    };
    let Some(tfunc) = env.get(name) else {
        bail!(span.clone(),  "undefined '{name}'");
    };
    if args.len() > tfunc.args.len() {
        bail!(span.clone(), "to many args given to '{name}'");
    }
    for (idx, arg) in args.iter().enumerate() {
        let t = type_of(func_name, env, arg)?;
        let (arg_name, pt) = &tfunc.args[idx];
        if pt != &t {
            bail!(
                arg.span(),
                "expected '{pt:?}' for {} but found '{t:?}'",
                arg_name.clone().unwrap_or("<name>".to_string())
            );
        }
    }
    Ok(t)
}

fn type_of<'a>(func_name: &str, env: &Types, e: &'a Expr) -> CResult<Type> {
    match e {
        Expr::Atom(Atom::Int(_), _) => Ok(Type::Int),
        Expr::Atom(Atom::Float(_), _) => Ok(Type::Float),
        Expr::Atom(Atom::Bool(_), _) => Ok(Type::Bool),
        Expr::Atom(Atom::String(_), _) => Ok(Type::String),
        Expr::Atom(Atom::Char(_), _) => Ok(Type::Char),
        Expr::Atom(Atom::Id(id), _) => lookup(func_name, env, id),
        Expr::Unary(_, rhs, _) => type_of(func_name, env, rhs),
        Expr::Binary(op, lhs, rhs, _) => type_check_binary(func_name, env, op, lhs, rhs),
        Expr::IfElse(c, b1, b2, _) => type_check_if_else(func_name, env, c, b1, b2),
        Expr::App(name, args, span) => type_check_app(func_name, env, name, args, span),
        _ => bail!(e.span(), "not implemented yet for expr: '{e:?}'"),
    }
}

fn pair_up_params<'a>(
    func_name: String,
    type_func: &mut TypedFunc,
    expr: &'a Expr,
) -> CResult<&'a Expr> {
    if !expr.is_clouser() {
        return Ok(expr);
    }
    let Expr::Closure(head, tail, _) = expr else {
        bail!(expr.span(), "unimplemented yet for '{expr}'");
    };
    let Expr::Atom(Atom::Id(name), _) = &**head else {
        bail!(head.span(), "unimplemented yet for '{expr}'");
    };
    type_func.push_arg(name);
    pair_up_params(func_name, type_func, &tail)
}

fn default_types() -> Types {
    let mut env = Types::new();
    let func = TypedFunc::new_with_args(Type::IO, vec![(Some("x".into()), Type::String)]);
    env.insert("print_str".into(), func);
    let func = TypedFunc::new_with_args(Type::IO, vec![(Some("x".into()), Type::Int)]);
    env.insert("print_int".into(), func);
    let func = TypedFunc::new_with_args(Type::IO, vec![(Some("x".into()), Type::Float)]);
    env.insert("print_float".into(), func);
    let func = TypedFunc::new_with_args(Type::IO, vec![(Some("x".into()), Type::Char)]);
    env.insert("print_char".into(), func);
    let func = TypedFunc::new_with_args(Type::IO, vec![(Some("x".into()), Type::Bool)]);
    env.insert("print_bool".into(), func);
    env
}

pub fn type_check<'a>(ast: &'a [Expr]) -> CResult<&'a [Expr]> {
    let mut env = default_types();
    for def in ast.iter() {
        match def {
            Expr::Func(name, body, span) => {
                let Some(type_func) = env.get_mut(name) else {
                    bail!(
                        span.clone(),
                        "function '{name}' missing type declaration"
                    );
                };
                let body = pair_up_params(name.into(), type_func, body)?;
                let dec_return_type = type_func.return_type.clone();
                let return_type = type_of(&name, &env, body)?;
                if return_type != dec_return_type {
                    bail!(
                        body.span(),
                        "miss matched return types: found '{return_type:?}' \
                        but expected '{dec_return_type:?}'"
                    );
                }
            }
            Expr::TypeDec(name, body, span) => {
                let mut body = body.clone();
                let Some(t) = body.pop() else {
                    bail!(span.clone(), "missing return type from type declaration '{name}'");
                };
                let mut args = vec![];
                for string_type in body.iter() {
                    args.push((None, Type::try_from(string_type)?));
                }
                let return_type = Type::try_from(&t)?;
                let typed_func = TypedFunc::new_with_args(return_type, args);
                env.insert(name.into(), typed_func);
            }
            _ => unimplemented!("for '{def}'"),
        }
    }
    Ok(ast)
}

#[cfg(test)]
mod tests {
    use super::*;
    use snowc_parse::ParserBuilder;
    #[test]
    fn string() {
        let src = r#"foo :: String;fn foo = "Hello";"#;
        let ast = ParserBuilder::default().build(src).parse().unwrap();
        let t = type_check(&ast);
        dbg!(&t);
        assert!(t.is_ok())
    }

    #[test]
    fn atom() {
        let src = r#"foo :: Int;fn foo = 12321321;"#;
        let ast = ParserBuilder::default().build(src).parse().unwrap();
        let t = type_check(&ast);
        dbg!(&t);
        assert!(t.is_ok());

        let src = r#"foo :: Float;fn foo = 12321.321;"#;
        let ast = ParserBuilder::default().build(src).parse().unwrap();
        let t = type_check(&ast);
        dbg!(&t);
        assert!(t.is_ok());

        let src = r#"foo :: Char;fn foo = 'c';"#;
        let ast = ParserBuilder::default().build(src).parse().unwrap();
        let t = type_check(&ast);
        dbg!(&t);
        assert!(t.is_ok());

        let src = r#"foo :: Bool;fn foo = true;"#;
        let ast = ParserBuilder::default().build(src).parse().unwrap();
        let t = type_check(&ast);
        dbg!(&t);
        assert!(t.is_ok());

        let src = r#"foo :: Bool;fn foo = false;"#;
        let ast = ParserBuilder::default().build(src).parse().unwrap();
        let t = type_check(&ast);
        dbg!(&t);
        assert!(t.is_ok())
    }

    #[test]
    fn unary() {
        let src = r#"bar :: Int;fn bar = -123;"#;
        let ast = ParserBuilder::default().build(src).parse().unwrap();
        let t = type_check(&ast);
        dbg!(&t);
        assert!(t.is_ok())
    }

    #[test]
    fn binary() {
        let src = r#"add :: Int -> Int -> Int;fn add x y = x + y;"#;
        let ast = ParserBuilder::default().build(src).parse().unwrap();
        let t = type_check(&ast);
        dbg!(&t);
        assert!(t.is_ok())
    }

    #[test]
    fn conditional() {
        let src = r#"max :: Int -> Int -> Int;
fn max x y = if x > y then x else y;"#;
        let ast = ParserBuilder::default().build(src).parse().unwrap();
        let t = type_check(&ast);
        dbg!(&t);
        assert!(t.is_ok())
    }

    #[test]
    fn func_dec_matches_func_two_args_return_int() {
        let src = r#"add :: Int -> Int -> Int;fn add x y = x + y;"#;
        let ast = ParserBuilder::default().build(src).parse().unwrap();
        let t = type_check(&ast);
        dbg!(&t);
        assert!(t.is_ok())
    }
}
