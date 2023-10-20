use super::Span;
use super::{Ident, Op};
use std::fmt;

macro_rules! is_expr {
    ($i:ident, $t:ident) => {
        pub fn $i(&self) -> bool {
            match self {
                Self::$t(..) => true,
                _ => false,
            }
        }
    };
}

#[derive(Clone, Hash, PartialEq, Eq)]
pub enum Atom {
    Int(i32),
    Float(String),
    Id(String),
    Bool(bool),
    String(String),
    Char(char),
}

impl fmt::Debug for Atom {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Int(i) => write!(f, "{i:?}"),
            Self::Float(i) => write!(f, "{i:?}"),
            Self::Id(id) => write!(f, "{id:?}"),
            Self::Bool(b) => write!(f, "{b:?}"),
            Self::String(s) => write!(f, "{s:?}"),
            Self::Char(s) => write!(f, "{s:?}"),
        }
    }
}

impl fmt::Display for Atom {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Int(i) => write!(f, "{i}"),
            Self::Float(i) => write!(f, "{i}"),
            Self::Id(id) => write!(f, "{id}"),
            Self::Bool(b) => write!(f, "{b}"),
            Self::String(s) => write!(f, "{s}"),
            Self::Char(s) => write!(f, "{s}"),
        }
    }
}

#[derive(Clone, Hash, PartialEq, Eq)]
pub enum Expr {
    Atom(Atom, Span),
    Unary(Op, Box<Self>, Span),
    Binary(Op, Box<Self>, Box<Self>, Span),
    IfElse(Box<Self>, Box<Self>, Box<Self>, Span),
    Closure(Box<Self>, Box<Self>, Span),
    Func(String, Vec<Ident>, Box<Self>, Span),
    App(Box<Self>, Vec<Self>, Span),
    Array(Vec<Self>, Span),
    Enum(String, Vec<(String, Vec<String>)>, Span),
    Error(Span),
}

impl Expr {
    pub fn and_then<F: FnOnce(Self) -> Self>(self, op: F) -> Self {
        match self {
            expr @ Self::Error(..) => expr,
            expr => op(expr),
        }
    }

    pub fn or_else<F: FnOnce(Self) -> Self>(self, op: F) -> Self {
        match self {
            expr @ Self::Error(..) => op(expr),
            expr => expr,
        }
    }

    pub fn span(&self) -> Span {
        match self {
            Self::Atom(.., span) => *span,
            Self::Unary(.., span) => *span,
            Self::Binary(.., span) => *span,
            Self::IfElse(.., span) => *span,
            Self::Closure(.., span) => *span,
            Self::Func(.., span) => *span,
            Self::App(.., span) => *span,
            Self::Array(.., span) => *span,
            Self::Enum(.., span) => *span,
            Self::Error(span) => *span,
        }
    }
    is_expr!(is_atom, Atom);
    is_expr!(is_unary, Unary);
    is_expr!(is_binary, Binary);
    is_expr!(is_if_else, IfElse);
    is_expr!(is_clouser, Closure);
    is_expr!(is_func, Func);
    is_expr!(is_app, App);
    is_expr!(is_type, Enum);
    is_expr!(is_array, Array);

    pub fn is_error(&self) -> bool {
        match self {
            Self::Unary(_, e, _) => e.is_error(),
            Self::Binary(_, l, r, _) => l.is_error() || r.is_error(),
            Self::IfElse(c, t, e, ..) => c.is_error() || t.is_error() || e.is_error(),
            Self::Closure(h, t, ..) => h.is_error() || t.is_error(),
            Self::Func(_, _, e, ..) => e.is_error(),
            Self::App(e, ..) => e.is_error(),
            Self::Array(array, ..) => array.iter().any(|e| e.is_error()),
            Self::Error(..) => true,
            _ => false,
        }
    }

    pub fn is_id(&self) -> bool {
        let Expr::Atom(Atom::Id(_), _) = self else {
            return false;
        };
        true
    }

    pub fn get_head(&self) -> Option<&Self> {
        match self {
            Expr::Closure(ref head, ..) => Some(head),
            _ => None,
        }
    }

    pub fn get_tail(&self) -> Option<&Self> {
        match self {
            Expr::Closure(_, ref tail, ..) => Some(tail),
            _ => None,
        }
    }
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Atom(i, ..) => write!(f, "{i}"),
            Self::Unary(op, lhs, ..) => write!(f, "({op} {lhs})"),
            Self::Binary(op, lhs, rhs, ..) => write!(f, "({op} {lhs} {rhs})"),
            Self::IfElse(condition, branch1, branch2, ..) => {
                write!(f, "(if ({condition}) then {branch1} else {branch2})")
            }
            Self::Closure(head, tail, ..) => {
                write!(f, "(\\{head} -> {tail})")
            }
            Self::Func(name, typed, clouser, ..) => {
                let t = typed.iter().fold("".to_string(), |acc, name| {
                    if acc.is_empty() {
                        return name.to_string();
                    }
                    format!("{acc} -> {name}")
                });
                write!(f, "<{name}: {t} = {clouser}>")
            }
            Self::App(name, args, ..) => {
                write!(f, "<{name}: (")?;
                for (i, arg) in args.iter().enumerate() {
                    write!(f, "{arg}")?;
                    if i < args.len() - 1 {
                        write!(f, ", ")?;
                    }
                }
                write!(f, ")>")?;
                Ok(())
            }
            Self::Array(array, ..) => {
                let mut a = array.iter().enumerate().fold(
                    "[".to_string(),
                    |mut acc, (idx, item)| {
                        if idx != 0 {
                            acc += ", ";
                        }
                        acc += item.to_string().as_str();
                        acc
                    },
                );
                a += "]";
                write!(f, "{a}")
            }
            Self::Enum(name, args, ..) => {
                if args.is_empty() {
                    return write!(f, "<{name}>");
                }
                let fstring = args.iter().enumerate().fold(
                    format!("<{name}: "),
                    |fstring, (i, (name, type_arg))| {
                        let targs = type_arg.iter().fold("".to_string(), |fstring, t| {
                            if fstring.is_empty() {
                                t.to_string()
                            } else {
                                format!("{fstring}, {t}")
                            }
                        });
                        if i < args.len() - 1 {
                            format!("{fstring}({name}, [{targs}]), ")
                        } else {
                            format!("{fstring}({name}, [{targs}])")
                        }
                    },
                );
                write!(f, "{fstring}>")
            }
            Self::Error(..) => write!(f, "Error"),
        }
    }
}

impl fmt::Debug for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Atom(i, ..) => write!(f, "{i:?}"),
            Self::Unary(op, lhs, ..) => write!(f, "({op} {lhs:?})"),
            Self::Binary(op, lhs, rhs, ..) => write!(f, "({op} {lhs:?} {rhs:?})"),
            Self::IfElse(condition, branch1, branch2, ..) => {
                write!(f, "(if ({condition:?}) then {branch1:?} else {branch2:?})")
            }
            Self::Closure(head, tail, ..) => {
                write!(f, "(\\{head:?} -> {tail:?})")
            }
            Self::Func(name, clouser, ..) => {
                write!(f, "<{name:?}: {clouser:?}>")
            }
            Self::App(name, args, ..) => {
                write!(f, "<{name:?}: (")?;
                for (i, arg) in args.iter().enumerate() {
                    write!(f, "{arg:?}")?;
                    if i < args.len() - 1 {
                        write!(f, ", ")?;
                    }
                }
                write!(f, ")>")?;
                Ok(())
            }
            Self::Array(array, ..) => {
                let mut a = array.iter().enumerate().fold(
                    "[".to_string(),
                    |mut acc, (idx, item)| {
                        if idx != 0 {
                            acc += ", ";
                        }
                        acc += format!("{item:?}").as_str();
                        acc
                    },
                );
                a += "]";
                write!(f, "{a}")
            }
            Self::Enum(name, args, ..) => {
                if args.is_empty() {
                    return write!(f, "<{name:?}>");
                }
                let fstring = args.iter().enumerate().fold(
                    format!("<{name:?}: "),
                    |fstring, (i, (name, type_arg))| {
                        let targs = type_arg.iter().fold(String::new(), |fstring, t| {
                            if fstring.is_empty() {
                                t.to_string()
                            } else {
                                format!("{fstring}, {t}")
                            }
                        });
                        if i < args.len() - 1 {
                            format!("{fstring:?}({name:?}, [{targs:?}]), ")
                        } else {
                            format!("{fstring:?}({name:?}, [{targs:?}])")
                        }
                    },
                );
                write!(f, "{fstring}>")
            }
            Self::Error(..) => write!(f, "Error"),
        }
    }
}
