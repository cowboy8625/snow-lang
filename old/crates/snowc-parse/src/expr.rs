use super::{Ident, Op, Span, TokenPosition};
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
    Int(i32, TokenPosition, Span),
    Float(String, TokenPosition, Span),
    Id(String, TokenPosition, Span),
    Bool(bool, TokenPosition, Span),
    String(String, TokenPosition, Span),
    Char(char, TokenPosition, Span),
}

impl Atom {
    pub fn span(&self) -> Span {
        match self {
            Self::Int(_, _, span) => *span,
            Self::Float(_, _, span) => *span,
            Self::Id(_, _, span) => *span,
            Self::Bool(_, _, span) => *span,
            Self::String(_, _, span) => *span,
            Self::Char(_, _, span) => *span,
        }
    }

    pub fn position(&self) -> TokenPosition {
        match self {
            Self::Int(_, pos, ..) => *pos,
            Self::Float(_, pos, ..) => *pos,
            Self::Id(_, pos, ..) => *pos,
            Self::Bool(_, pos, ..) => *pos,
            Self::String(_, pos, ..) => *pos,
            Self::Char(_, pos, ..) => *pos,
        }
    }
}

impl fmt::Debug for Atom {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Int(i, ..) => write!(f, "{i:?}"),
            Self::Float(i, ..) => write!(f, "{i:?}"),
            Self::Id(id, ..) => write!(f, "{id:?}"),
            Self::Bool(b, ..) => write!(f, "{b:?}"),
            Self::String(s, ..) => write!(f, "{s:?}"),
            Self::Char(s, ..) => write!(f, "{s:?}"),
        }
    }
}

impl fmt::Display for Atom {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Int(i, ..) => write!(f, "{i}"),
            Self::Float(i, ..) => write!(f, "{i}"),
            Self::Id(id, ..) => write!(f, "{id}"),
            Self::Bool(b, ..) => write!(f, "{b}"),
            Self::String(s, ..) => write!(f, "{s}"),
            Self::Char(s, ..) => write!(f, "{s}"),
        }
    }
}

#[derive(Clone, Hash, PartialEq, Eq)]
pub struct Unary {
    pub op: Op,
    pub expr: Box<Expr>,
    pub pos: TokenPosition,
    pub span: Span,
}

#[derive(Clone, Hash, PartialEq, Eq)]
pub struct Binary {
    pub op: Op,
    pub left: Box<Expr>,
    pub right: Box<Expr>,
    pub pos: TokenPosition,
    pub span: Span,
}

#[derive(Clone, Hash, PartialEq, Eq)]
pub struct App {
    pub name: Box<Expr>,
    pub args: Vec<Expr>,
    pub pos: TokenPosition,
    pub span: Span,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum TypeInfo {
    Int,
    Float,
    Bool,
    String,
    Char,
    Array(Box<Self>),
    Custom(String),
}

impl From<Ident> for TypeInfo {
    fn from(ident: Ident) -> Self {
        match ident.lexme.as_str() {
            "Int" => Self::Int,
            "Float" => Self::Float,
            "Bool" => Self::Bool,
            "String" => Self::String,
            "Char" => Self::Char,
            name => Self::Custom(name.to_string()),
        }
    }
}

impl fmt::Display for TypeInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Int => write!(f, "Int"),
            Self::Float => write!(f, "Float"),
            Self::Bool => write!(f, "Bool"),
            Self::String => write!(f, "String"),
            Self::Char => write!(f, "Char"),
            Self::Array(type_info) => write!(f, "Array<{type_info}>"),
            Self::Custom(name) => write!(f, "{name}"),
        }
    }
}

#[derive(Clone, Hash, PartialEq, Eq)]
pub enum Expr {
    // App(Box<Self>, Vec<Self>, Span),
    App(App),
    Array(Vec<Self>, TokenPosition, Span),
    Atom(Atom),
    Binary(Binary),
    Closure(Box<Self>, Box<Self>, Span),
    Enum(String, Vec<(String, Vec<String>)>, Span),
    Error(Span),
    Func(String, Vec<TypeInfo>, Box<Self>, Span),
    IfElse(Box<Self>, Box<Self>, Box<Self>, Span),
    Unary(Unary),
}

impl Expr {
    pub fn map_position(self, f: impl Fn(TokenPosition) -> TokenPosition) -> Self {
        match self {
            Self::Atom(atom) => Self::Atom(match atom {
                Atom::Int(i, pos, span) => Atom::Int(i, f(pos), span),
                Atom::Float(i, pos, span) => Atom::Float(i, f(pos), span),
                Atom::Id(i, pos, span) => Atom::Id(i, f(pos), span),
                Atom::Bool(i, pos, span) => Atom::Bool(i, f(pos), span),
                Atom::String(i, pos, span) => Atom::String(i, f(pos), span),
                Atom::Char(i, pos, span) => Atom::Char(i, f(pos), span),
            }),
            Self::Unary(Unary {
                op,
                expr,
                pos,
                span,
            }) => Self::Unary(Unary {
                op,
                expr,
                pos: f(pos),
                span,
            }),
            Self::Binary(Binary {
                op,
                left,
                right,
                pos,
                span,
            }) => Self::Binary(Binary {
                op,
                left,
                right,
                pos: f(pos),
                span,
            }),
            Self::IfElse(cond, then, r#else, span) => {
                Self::IfElse(cond, then, r#else, span)
            }
            Self::Closure(head, tail, span) => Self::Closure(head, tail, span),
            Self::Func(name, args, body, span) => Self::Func(name, args, body, span),
            Self::App(App {
                name,
                args,
                pos,
                span,
            }) => Self::App(App {
                name,
                args,
                pos: f(pos),
                span,
            }),
            Self::Array(array, pos, span) => Self::Array(array, f(pos), span),
            Self::Enum(name, variants, span) => Self::Enum(name, variants, span),
            Self::Error(span) => Self::Error(span),
        }
    }

    pub fn span(&self) -> Span {
        match self {
            Self::Atom(atom) => atom.span(),
            Self::Unary(unary) => unary.span,
            Self::Binary(binary) => binary.span,
            Self::IfElse(.., span) => *span,
            Self::Closure(.., span) => *span,
            Self::Func(.., span) => *span,
            Self::App(app) => app.span,
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
            Self::Unary(unary) => unary.expr.is_error(),
            Self::Binary(binary) => binary.left.is_error() || binary.right.is_error(),
            Self::IfElse(c, t, e, ..) => c.is_error() || t.is_error() || e.is_error(),
            Self::Closure(h, t, ..) => h.is_error() || t.is_error(),
            Self::Func(_, _, e, ..) => e.is_error(),
            Self::App(app) => app.name.is_error(),
            Self::Array(array, ..) => array.iter().any(|e| e.is_error()),
            Self::Error(..) => true,
            _ => false,
        }
    }

    pub fn is_id(&self) -> bool {
        let Expr::Atom(Atom::Id(..)) = self else {
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

    pub fn position(&self) -> TokenPosition {
        match self {
            Self::Atom(atom) => atom.position(),
            Self::Unary(unary) => unary.pos,
            Self::Binary(binary) => binary.pos,
            Self::App(app) => app.pos,
            Self::Closure(_, tail, ..) => tail.position(),
            Self::Array(_, pos, ..) => *pos,
            Self::IfElse(_, _, r#else, ..) => r#else.position(),
            _ => unimplemented!("for {self:?}"),
        }
    }
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Atom(atom) => write!(f, "{atom}"),
            Self::Unary(unary) => write!(f, "({} {})", unary.op, unary.expr),
            Self::Binary(b) => write!(f, "({} {} {})", b.op, b.left, b.right),
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
            Self::App(app) => {
                write!(f, "<{}: (", app.name)?;
                for (i, arg) in app.args.iter().enumerate() {
                    write!(f, "{arg}")?;
                    if i < app.args.len() - 1 {
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
            Self::Atom(atom) => write!(f, "{atom:?}"),
            Self::Unary(unary) => write!(f, "({} {:?})", unary.op, unary.expr),
            Self::Binary(b) => write!(f, "({} {:?} {:?})", b.op, b.left, b.right),
            Self::IfElse(condition, branch1, branch2, ..) => {
                write!(f, "(if ({condition:?}) then {branch1:?} else {branch2:?})")
            }
            Self::Closure(head, tail, ..) => {
                write!(f, "(\\{head:?} -> {tail:?})")
            }
            Self::Func(name, t, clouser, ..) => {
                let t = t.iter().fold("".to_string(), |acc, name| {
                    if acc.is_empty() {
                        return name.to_string();
                    }
                    format!("{acc} -> {name}")
                });
                write!(f, "<{name:?}: {t} = {clouser:?}>")
            }
            Self::App(App { name, args, .. }) => {
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
