use super::Op;
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

#[derive(Debug, Clone, Hash)]
pub enum Atom {
    Int(i32),
    Float(String),
    Id(String),
    Bool(bool),
    String(String),
    Char(char),
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

#[derive(Debug, Clone, Hash)]
pub enum Expr {
    Atom(Atom),
    Unary(Op, Box<Self>),
    Binary(Op, Box<Self>, Box<Self>),
    IfElse(Box<Self>, Box<Self>, Box<Self>),
    Closure(Box<Self>, Box<Self>),
    Func(String, Box<Self>),
    App(Box<Self>, Vec<Self>),
    Type(String, Vec<(String, Vec<String>)>),
    TypeDec(String, Vec<String>),
}

impl Expr {
    is_expr!(is_atom, Atom);
    is_expr!(is_unary, Unary);
    is_expr!(is_binary, Binary);
    is_expr!(is_if_else, IfElse);
    is_expr!(is_clouser, Closure);
    is_expr!(is_func, Func);
    is_expr!(is_app, App);
    is_expr!(is_type, Type);
    is_expr!(is_type_dec, TypeDec);
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Atom(i) => write!(f, "{i}"),
            Self::Unary(op, lhs) => write!(f, "({op} {lhs})"),
            Self::Binary(op, lhs, rhs) => write!(f, "({op} {lhs} {rhs})"),
            Self::IfElse(condition, branch1, branch2) => {
                write!(f, "(if ({condition}) then {branch1} else {branch2})")
            }
            Self::Closure(head, tail) => {
                write!(f, "(\\{head} -> {tail})")
            }
            Self::Func(name, clouser) => {
                write!(f, "<{name}: {clouser}>")
            }
            Self::App(name, args) => {
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
            Self::Type(name, args) => {
                if args.is_empty() {
                    return write!(f, "<{name}>");
                }
                let fstring = args.iter().enumerate().fold(
                    format!("<{name}: "),
                    |fstring, (i, (name, type_arg))| {
                        let targs = type_arg.iter().fold("".to_string(), |fstring, t| {
                            if fstring == "" {
                                format!("{t}")
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
            Self::TypeDec(name, type_list) => {
                let types = type_list.iter().fold("".to_string(), |fstring, item| {
                    if fstring == "" {
                        format!("{item}")
                    } else {
                        format!("{fstring} -> {item}")
                    }
                });
                write!(f, "<{name} :: {types}>")
            }
        }
    }
}
