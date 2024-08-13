use super::Span;
use std::fmt;

macro_rules! map_a {
    ($name:ident, $kind:ident) => {
        pub fn $name<T>(&self, f: impl FnOnce(&$kind) -> T) -> Option<T> {
            match self {
                Self::$kind(i) => Some(f(i)),
                _ => None,
            }
        }
    };
}

macro_rules! init_token {
    ($name:ident) => {
        #[derive(Debug, Clone, PartialEq, Eq, Hash)]
        pub struct $name {
            pub lexme: String,
            pub pos: TokenPosition,
            pub span: Span,
        }
        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                let Self { lexme, .. } = self;
                write!(f, "{lexme}")
            }
        }
    };
}

init_token!(KeyWord);
init_token!(Ident);
init_token!(Op);
init_token!(Ctrl);
init_token!(Int);
init_token!(Float);
init_token!(Str);
init_token!(Char);
init_token!(Error);

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token {
    KeyWord(KeyWord),
    Ident(Ident),
    Op(Op),
    Ctrl(Ctrl),
    Int(Int),
    Float(Float),
    Str(Str),
    Char(Char),
    Error(Error),
}

impl Token {
    map_a!(map_keyword, KeyWord);
    map_a!(map_ident, Ident);
    map_a!(map_op, Op);
    map_a!(map_ctrl, Ctrl);
    map_a!(map_int, Int);
    map_a!(map_float, Float);
    map_a!(map_string, Str);
    map_a!(map_char, Char);
    map_a!(map_error, Error);

    pub fn span(&self) -> Span {
        match self {
            Self::KeyWord(KeyWord { span, .. }) => *span,
            Self::Ident(Ident { span, .. }) => *span,
            Self::Op(Op { span, .. }) => *span,
            Self::Ctrl(Ctrl { span, .. }) => *span,
            Self::Int(Int { span, .. }) => *span,
            Self::Float(Float { span, .. }) => *span,
            Self::Str(Str { span, .. }) => *span,
            Self::Char(Char { span, .. }) => *span,
            Self::Error(Error { span, .. }) => *span,
        }
    }

    pub fn position(&self) -> &TokenPosition {
        match self {
            Self::KeyWord(KeyWord { pos, .. }) => pos,
            Self::Ident(Ident { pos, .. }) => pos,
            Self::Op(Op { pos, .. }) => pos,
            Self::Ctrl(Ctrl { pos, .. }) => pos,
            Self::Int(Int { pos, .. }) => pos,
            Self::Float(Float { pos, .. }) => pos,
            Self::Str(Str { pos, .. }) => pos,
            Self::Char(Char { pos, .. }) => pos,
            // FIXME: I don't like this
            Self::Error(..) => &TokenPosition::End,
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::KeyWord(i) => write!(f, "{i}"),
            Self::Ident(i) => write!(f, "{i}"),
            Self::Op(i) => write!(f, "{i}"),
            Self::Ctrl(i) => write!(f, "{i}"),
            Self::Int(i) => write!(f, "{i}"),
            Self::Float(i) => write!(f, "{i}"),
            Self::Str(i) => write!(f, "{i}"),
            Self::Char(i) => write!(f, "{i}"),
            Self::Error(i) => write!(f, "{i}"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TokenPosition {
    Start,
    Middle,
    End,
    FullSpan,
}

impl Default for TokenPosition {
    fn default() -> Self {
        Self::End
    }
}
