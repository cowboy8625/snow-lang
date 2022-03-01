use super::{BuiltIn, KeyWord, Parser, Spanned, Token};
use std::fmt;
pub(crate) fn number<'a>() -> impl Parser<'a, Token, Spanned<Atom>> {
    move |input: &'a [Spanned<Token>]| match &input.get(0) {
        Some(node) => match &node.node {
            Token::Int(i) => Ok((
                &input[1..],
                (Atom::Int(i.parse().unwrap()), input[0].span()).into(),
            )),
            Token::Float(f) => Ok((
                &input[1..],
                (Atom::Float(f.parse().unwrap()), input[0].span()).into(),
            )),
            _ => Err(input),
        },
        _ => Err(input),
    }
}

pub(crate) fn string<'a>() -> impl Parser<'a, Token, Spanned<Atom>> {
    move |input: &'a [Spanned<Token>]| match &input.get(0) {
        Some(node) => match &node.node {
            Token::String(s) => Ok((
                &input[1..],
                (Atom::String(s.to_string()), input[0].span()).into(),
            )),
            _ => Err(input),
        },
        _ => Err(input),
    }
}

// pub(crate) fn _keyword<'a>() -> impl Parser<'a, Token, Spanned<Atom>> {
//     move |input: &'a [Spanned<Token>]| match &input.get(0) {
//         Some(node) => match node.node {
//             Token::KeyWord(kw) => Ok((&input[1..], (Atom::Keyword(kw), input[0].span()).into())),
//             _ => Err(input),
//         },
//         _ => Err(input),
//     }
// }

pub(crate) fn boolean<'a>() -> impl Parser<'a, Token, Spanned<Atom>> {
    move |input: &'a [Spanned<Token>]| match &input.get(0) {
        Some(node) => match node.node {
            Token::KeyWord(KeyWord::True) => {
                Ok((&input[1..], (Atom::Boolean(true), input[0].span()).into()))
            }
            Token::KeyWord(KeyWord::False) => {
                Ok((&input[1..], (Atom::Boolean(false), input[0].span()).into()))
            }
            _ => Err(input),
        },
        _ => Err(input),
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Atom {
    Int(i32),
    Float(f32),
    String(String),
    Boolean(bool),
    // Keyword(KeyWord),
    BuiltIn(BuiltIn),
}

impl fmt::Display for Atom {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Int(i) => write!(f, "{}", i),
            Self::Float(i) => write!(f, "{}", i),
            Self::String(i) => write!(f, "{}", i),
            Self::Boolean(i) => write!(f, "{}", i),
            // Self::Keyword(i) => write!(f, "{}", i),
            Self::BuiltIn(i) => write!(f, "{}", i),
        }
    }
}
