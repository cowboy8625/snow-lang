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
        #[derive(Debug, Clone, PartialEq, Eq)]
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenPosition {
    Start,
    Middle,
    End,
    FullSpan,
}

// macro_rules! is_token {
//     ($i:ident, $t:ident) => {
//         pub fn $i(&self) -> bool {
//             match self {
//                 Self::$t(..) => true,
//                 _ => false,
//             }
//         }
//     };
// }
//
//
// macro_rules! map_a {
//     ($name:ident, $kind:ident) => {
//         pub fn $name<T>(
//             &self,
//             f: impl FnOnce(&str, &TokenPosition, &Span) -> T,
//         ) -> Option<T> {
//             match self {
//                 Self::$kind(lexme, tp, span) => Some(f(lexme, tp, span)),
//                 _ => None,
//             }
//         }
//     };
// }
//
// #[derive(Clone, PartialEq, Eq)]
// pub enum Token {
//     KeyWord(String, TokenPosition, Span),
//     Id(String, TokenPosition, Span),
//     Op(String, TokenPosition, Span),
//     Int(String, TokenPosition, Span),
//     Float(String, TokenPosition, Span),
//     String(String, TokenPosition, Span),
//     Char(String, TokenPosition, Span),
//     Error(String, Span),
//     Eof(Span),
// }
//
// impl Token {
//     is_token!(is_keyword, KeyWord);
//     is_token!(is_id, Id);
//     is_token!(is_op, Op);
//     is_token!(is_int, Int);
//     is_token!(is_float, Float);
//     is_token!(is_string, String);
//     is_token!(is_char, Char);
//     is_token!(is_error, Error);
//     is_token!(is_eof, Eof);
//
//     pub fn is_keyword_a(&self, item: impl Into<String>) -> bool {
//         match self {
//             Self::KeyWord(ref inner, ..) => inner == &item.into(),
//             _ => false,
//         }
//     }
//
//     pub fn is_op_a(&self, item: impl Into<String>) -> bool {
//         match self {
//             Self::Op(ref inner, ..) => inner == &item.into(),
//             _ => false,
//         }
//     }
//
//     pub fn is_id_a(&self, item: impl Into<String>) -> bool {
//         match self {
//             Self::Id(ref inner, ..) => inner == &item.into(),
//             _ => false,
//         }
//     }
//
//     pub fn map<T>(&self, f: impl FnOnce(&str, &TokenPosition, &Span) -> T) -> Option<T> {
//         match self {
//             Self::KeyWord(lexme, tp, span)
//             | Self::Id(lexme, tp, span)
//             | Self::Op(lexme, tp, span)
//             | Self::Int(lexme, tp, span)
//             | Self::Float(lexme, tp, span)
//             | Self::String(lexme, tp, span)
//             | Self::Char(lexme, tp, span) => Some(f(lexme, tp, span)),
//             _ => None,
//         }
//     }
//
//     map_a!(map_keyword, KeyWord);
//     map_a!(map_id, Id);
//     map_a!(map_op, Op);
//     map_a!(map_int, Int);
//     map_a!(map_float, Float);
//     map_a!(map_string, String);
//     map_a!(map_char, Char);
//
//     pub fn position(&self) -> &TokenPosition {
//         match self {
//             Self::KeyWord(_, tp, ..) => tp,
//             Self::Id(_, tp, ..) => tp,
//             Self::Op(_, tp, ..) => tp,
//             Self::Int(_, tp, ..) => tp,
//             Self::Float(_, tp, ..) => tp,
//             Self::String(_, tp, ..) => tp,
//             Self::Char(_, tp, ..) => tp,
//             // FIXME: I don't like this
//             Self::Error(_, ..) => &TokenPosition::End,
//             Self::Eof(..) => &TokenPosition::End,
//         }
//     }
//
//     pub fn value(&self) -> &str {
//         match self {
//             Self::KeyWord(i, ..) => i,
//             Self::Id(i, ..) => i,
//             Self::Op(i, ..) => i,
//             Self::Int(i, ..) => i,
//             Self::Float(i, ..) => i,
//             Self::String(i, ..) => i,
//             Self::Char(i, ..) => i,
//             Self::Error(i, ..) => i,
//             Self::Eof(..) => "Eof",
//         }
//     }
//
//     pub fn span(&self) -> Span {
//         match self {
//             Self::KeyWord(.., span) => *span,
//             Self::Id(.., span) => *span,
//             Self::Op(.., span) => *span,
//             Self::Int(.., span) => *span,
//             Self::Float(.., span) => *span,
//             Self::String(.., span) => *span,
//             Self::Char(.., span) => *span,
//             Self::Error(.., span) => *span,
//             Self::Eof(span) => *span,
//         }
//     }
// }
//
// impl fmt::Debug for Token {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         match self {
//             Self::KeyWord(i, tp, ..) => write!(f, "{i} {tp:?}"),
//             Self::Id(i, tp, ..) => write!(f, "{i} {tp:?}"),
//             Self::Op(i, tp, ..) => write!(f, "{i} {tp:?}"),
//             Self::Int(i, tp, ..) => write!(f, "{i} {tp:?}"),
//             Self::Float(i, tp, ..) => write!(f, "{i} {tp:?}"),
//             Self::String(i, tp, ..) => write!(f, "{i:?} {tp:?}"),
//             Self::Char(i, tp, ..) => write!(f, "{i} {tp:?}"),
//             Self::Error(i, ..) => write!(f, "{i}"),
//             Self::Eof(..) => write!(f, "EOF"),
//         }
//     }
// }
//
// impl fmt::Display for Token {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         match self {
//             Self::KeyWord(i, ..) => write!(f, "{i}"),
//             Self::Id(i, ..) => write!(f, "{i}"),
//             Self::Op(i, ..) => write!(f, "{i}"),
//             Self::Int(i, ..) => write!(f, "{i}"),
//             Self::Float(i, ..) => write!(f, "{i}"),
//             Self::String(i, ..) => write!(f, "{i}"),
//             Self::Char(i, ..) => write!(f, "{i}"),
//             Self::Error(i, ..) => write!(f, "{i}"),
//             Self::Eof(..) => write!(f, "EOF"),
//         }
//     }
// }
//
// init_token!();
