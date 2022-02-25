#![allow(unused)]
use super::combinators::{either, left, one_or_more, right, zero_or_more, ParseResult, Parser};
use super::position::{self, Pos, Span, Spanned};
use super::scanner::{self, KeyWord, Token};
use std::fmt;

use std::collections::HashMap;

pub type FunctionList = HashMap<String, Spanned<Expr>>;

// type ParseResult<'a, O, I> = Result<(Spanned<O>, &'a [Spanned<I>]), &'a [Spanned<I>]>;
#[derive(Debug, PartialEq, Clone, Copy, Eq)]
pub enum BuiltIn {
    Plus,
    Mins,
    Mult,
    Div,
    Eq,
    NEq,
    Not,
    Print,
    PrintLn,
}

impl fmt::Display for BuiltIn {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Plus => write!(f, "Plus"),
            Self::Mins => write!(f, "Mins"),
            Self::Mult => write!(f, "Mult"),
            Self::Div => write!(f, "Div"),
            Self::Eq => write!(f, "Eq"),
            Self::NEq => write!(f, "NEq"),
            Self::Not => write!(f, "Not"),
            Self::Print => write!(f, "print"),
            Self::PrintLn => write!(f, "println"),
        }
    }
}
impl From<(BuiltIn, Span)> for Spanned<BuiltIn> {
    fn from((node, span): (BuiltIn, Span)) -> Self {
        Spanned { node, span }
    }
}
#[derive(Debug, PartialEq, Clone)]
pub enum Atom {
    Int(i32),
    Float(f32),
    String(String),
    Keyword(KeyWord),
    Boolean(bool),
    BuiltIn(BuiltIn),
}
impl fmt::Display for Atom {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Int(i) => write!(f, "Int({})", i),
            Self::Float(i) => write!(f, "Float({})", i),
            Self::String(i) => write!(f, "String({})", i),
            Self::Keyword(i) => write!(f, "Keyword({})", i),
            Self::Boolean(i) => write!(f, "Boolean({})", i),
            Self::BuiltIn(i) => write!(f, "BuiltIn({})", i),
        }
    }
}
impl From<(Atom, Span)> for Spanned<Atom> {
    fn from((node, span): (Atom, Span)) -> Self {
        Spanned { node, span }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Expr {
    Constant(Atom),
    // func-name args
    Application(Box<Expr>, Vec<Spanned<Expr>>),
    // func-name prams body
    Lambda(Spanned<String>, Vec<Spanned<String>>, Box<Spanned<Expr>>),
    // func name's or pram name's
    Local(String),
    // (if predicate do-this)
    // If(Box<Expr>, Box<Expr>),
    // // (if predicate do-this otherwise-do-this)
    // IfElse(Box<Expr>, Box<Expr>, Box<Expr>),
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Local(name) => write!(f, "Local({})", name),
            Self::Constant(a) => write!(f, "Const({})", a),
            Self::Application(n, a) => write!(
                f,
                "App({},{:?})",
                n,
                a.iter()
                    .map(|p| p.node.to_string())
                    .collect::<Vec<String>>(),
            ),
            Self::Lambda(n, p, b) => write!(
                f,
                "Lambda({},{:?}, {})",
                n.node,
                p.iter()
                    .map(|p| p.node.to_string())
                    .collect::<Vec<String>>(),
                b.node,
            ),
        }
    }
}
impl From<(Expr, Span)> for Spanned<Expr> {
    fn from((node, span): (Expr, Span)) -> Self {
        Spanned { node, span }
    }
}

fn builtins<'a>() -> impl Parser<'a, Token, Spanned<BuiltIn>> {
    move |input: &'a [Spanned<Token>]| match &input.get(0) {
        Some(node) => match node.node {
            Token::Op("+") => Ok((&input[1..], (BuiltIn::Plus, input[0].span()).into())),
            Token::Op("-") => Ok((&input[1..], (BuiltIn::Mins, input[0].span()).into())),
            Token::Op("*") => Ok((&input[1..], (BuiltIn::Mult, input[0].span()).into())),
            Token::Op("/") => Ok((&input[1..], (BuiltIn::Div, input[0].span()).into())),
            Token::Op("==") => Ok((&input[1..], (BuiltIn::Eq, input[0].span()).into())),
            Token::Op("!=") => Ok((&input[1..], (BuiltIn::NEq, input[0].span()).into())),
            Token::Op("!") => Ok((&input[1..], (BuiltIn::Not, input[0].span()).into())),
            Token::KeyWord(kw) => match kw {
                KeyWord::Print => Ok((&input[1..], (BuiltIn::Print, input[0].span()).into())),
                KeyWord::PrintLn => Ok((&input[1..], (BuiltIn::PrintLn, input[0].span()).into())),
                _ => Err(input),
            },
            _ => Err(input),
        },
        _ => Err(input),
    }
}

fn number<'a>() -> impl Parser<'a, Token, Spanned<Atom>> {
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
fn string<'a>() -> impl Parser<'a, Token, Spanned<Atom>> {
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
fn keyword<'a>() -> impl Parser<'a, Token, Spanned<Atom>> {
    move |input: &'a [Spanned<Token>]| match &input.get(0) {
        Some(node) => match node.node {
            Token::KeyWord(kw) => Ok((&input[1..], (Atom::Keyword(kw), input[0].span()).into())),
            _ => Err(input),
        },
        _ => Err(input),
    }
}

fn boolean<'a>() -> impl Parser<'a, Token, Spanned<Atom>> {
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
fn constant<'a>() -> impl Parser<'a, Token, Spanned<Expr>> {
    move |input: &'a [Spanned<Token>]| {
        either(
            builtins().map(|b| (Atom::BuiltIn(b.node.clone()), b.span()).into()),
            either(boolean(), either(keyword(), either(string(), number()))),
        )
        .parse(input)
        .map(|(i, b)| (i, (Expr::Constant(b.node.clone()), b.span()).into()))
    }
}

fn app<'a>() -> impl Parser<'a, Token, Spanned<Expr>> {
    move |input: &'a [Spanned<Token>]| {
        let (i, name) = either(
            local(),
            builtins().map(|s| (Expr::Constant(Atom::BuiltIn(s.node.clone())), s.span()).into()),
        )
        .parse(input)?;
        let (i, args) = zero_or_more(either(local(), either(prans(), constant()))).parse(i)?;
        Ok((
            i,
            (
                Expr::Application(Box::new(name.node.clone()), args),
                name.span(),
            )
                .into(),
        ))
    }
}

fn parse_name<'a>() -> impl Parser<'a, Token, Spanned<String>> {
    move |input: &'a [Spanned<Token>]| match input.get(0) {
        Some(node) => match &node.node {
            Token::Id(name) => Ok((&input[1..], Spanned::new(name.to_string(), input[0].span()))),
            _ => Err(input),
        },
        _ => Err(input),
    }
}

fn local<'a>() -> impl Parser<'a, Token, Spanned<Expr>> {
    parse_name().map(|s| (Expr::Local(s.node.clone()), s.span()).into())
}

fn next_token<'a>(token: Token) -> impl Parser<'a, Token, Spanned<Token>> {
    move |input: &'a [Spanned<Token>]| match input.get(0) {
        Some(node) if node.node == token => Ok((&input[1..], node.clone())),
        _ => Err(input),
    }
}

fn lambda<'a>() -> impl Parser<'a, Token, Spanned<Expr>> {
    move |input: &'a [Spanned<Token>]| {
        let (i, start) = next_token(Token::DeDent).parse(input)?;
        let (i, name) = parse_name().parse(i)?;
        let (i, prams) = zero_or_more(parse_name()).parse(i)?;
        let (i, _) = next_token(Token::Op("=")).parse(i)?;
        let (i, body) = either(app(), constant()).parse(i)?;
        Ok((
            i,
            (
                Expr::Lambda(name, prams, Box::new(body.clone())),
                Span::new(start.span.start, body.span.end, &start.span.loc),
            )
                .into(),
        ))
    }
}

// fn expression<'a>() -> impl Parser<'a, Token, Spanned<Expr>> {
//     either(prans(), either(app(), constant()))
// }

fn prans<'a>() -> impl Parser<'a, Token, Spanned<Expr>> {
    right(
        next_token(Token::Ctrl('(')),
        left(
            either(app(), either(local(), constant())),
            next_token(Token::Ctrl(')')),
        ),
    )
}

pub fn parser<'a>() -> impl Parser<'a, Token, FunctionList> {
    move |input: &'a [Spanned<Token>]| {
        let (i, result) = one_or_more(lambda()).parse(input)?;
        let mut funcs = FunctionList::new();
        for f in result.iter() {
            match &f.node {
                Expr::Lambda(name, ..) => funcs.insert(name.node.clone(), f.clone()),
                x => unreachable!(x),
            };
        }
        Ok((i, funcs))
    }
}
