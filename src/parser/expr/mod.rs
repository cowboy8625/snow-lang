mod conditionals;
mod let_expr;
use super::mini_parse::{self, either, left, one_or_more, surround, zero_or_more, Parser};
use super::{boolean, builtin, number, string, Atom, Error, KeyWord, Span, Spanned, Token};
use crate::one_of;
use conditionals::conditional;
use let_expr::let_expr;
use std::fmt;

fn _print_input<T>(input: &[Spanned<Token>], result: &T, msg: &str)
where
    T: fmt::Display,
{
    eprintln!("--Start--{}------", msg);
    eprintln!("--Result--{}-----", result);
    for i in input.iter() {
        eprintln!("{}", i);
    }
    eprintln!("--End--{}------", msg);
}

pub fn indent_token<'a>() -> impl Parser<'a, Token, Spanned<Token>> {
    next_token(Token::InDent)
}

pub fn equal_token<'a>() -> impl Parser<'a, Token, Spanned<Token>> {
    next_token(Token::Op("="))
}

pub fn do_token<'a>() -> impl Parser<'a, Token, Spanned<Token>> {
    next_token(Token::KeyWord(KeyWord::Do))
}

pub fn delimiter_token<'a>() -> impl Parser<'a, Token, Spanned<Token>> {
    next_token(Token::Delimiter)
}

pub fn dedent_token<'a>() -> impl Parser<'a, Token, Spanned<Token>> {
    next_token(Token::DeDent)
}

pub fn constant<'a>() -> impl Parser<'a, Token, Spanned<Expr>> {
    move |input: &'a [Spanned<Token>]| {
        one_of!(
            builtin().map(|b| (Atom::BuiltIn(b.node.clone()), b.span()).into()),
            boolean(),
            string(),
            number()
        )
        .parse(input)
        .map(|(i, b)| (i, (Expr::Constant(b.node.clone()), b.span()).into()))
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
    parse_name().map(|s| (Expr::Local(s.clone()), s.span()).into())
}

fn next_token<'a>(token: Token) -> impl Parser<'a, Token, Spanned<Token>> {
    move |input: &'a [Spanned<Token>]| match input.get(0) {
        Some(node) if node.node == token => Ok((&input[1..], node.clone())),
        _ => Err(input),
    }
}

fn do_expr<'a>() -> impl Parser<'a, Token, Spanned<Expr>> {
    move |input: &'a [Spanned<Token>]| {
        let (i, span_start) = left(do_token(), indent_token()).parse(input)?;
        let (i, body) = one_or_more(left(func_expr(), delimiter_token())).parse(i)?;
        let (i, end) = dedent_token().parse(i)?;
        let span: Span = (span_start.span(), end.span()).into();
        Ok((i, (Expr::Do(body), span).into()))
    }
}

// TODO: This will not work with "+ ((1)) 1"
fn expr<'a>() -> impl Parser<'a, Token, Spanned<Expr>> {
    one_of!(app(), local(), constant())
}
fn prans<'a>() -> impl Parser<'a, Token, Spanned<Expr>> {
    surround(
        next_token(Token::Ctrl('(')),
        expr(),
        next_token(Token::Ctrl(')')),
    )
}

// + 1 2 3 4 5
// (+ 1) + 2 -> (+ (2 (1))
// (+ 1) + 2 -> (+ [1]) 1
// (+ 1) + -> +
// (+ 1) +
pub fn app<'a>() -> impl Parser<'a, Token, Spanned<Expr>> {
    move |input: &'a [Spanned<Token>]| {
        let (i, name) = one_of!(
            prans(),
            local(),
            builtin().map(|s| (Expr::Constant(Atom::BuiltIn(s.node.clone())), s.span()).into()),
        )
        .parse(input)?;
        // _print_input(i, &name.node, "App name");
        let (i, args) = zero_or_more(one_of!(local(), prans(), constant())).parse(i)?;
        // _print_input(i, &"", "Args");
        Ok((
            i,
            (Expr::Application(Box::new(name.clone()), args), name.span()).into(),
        ))
    }
}

pub fn func_expr<'a>() -> impl Parser<'a, Token, Spanned<Expr>> {
    one_of!(
        prans(),
        let_expr(),
        do_expr(),
        conditional(),
        app(),
        constant()
    )
}

fn fn_token<'a>() -> impl Parser<'a, Token, Spanned<String>> {
    move |input: &'a [Spanned<Token>]| match input.get(0) {
        Some(node) => match &node.node {
            Token::Fn(name) => Ok((&input[1..], Spanned::new(name.to_string(), input[0].span()))),
            _ => Err(input),
        },
        _ => Err(input),
    }
}

pub(crate) fn function<'a>() -> impl Parser<'a, Token, Spanned<Expr>> {
    move |input: &'a [Spanned<Token>]| {
        // Gets name of Function
        let (i, name) = fn_token().parse(input)?;
        // Gets Parameters
        let (i, prams) = zero_or_more(parse_name()).parse(i)?;
        let (i, _) = either(left(equal_token(), indent_token()), equal_token()).parse(i)?;
        let (i, body) = either(left(func_expr(), dedent_token()), func_expr()).parse(i)?;
        Ok((
            i,
            (
                Expr::Function(name.clone(), prams, Box::new(body.clone())),
                (name.span(), body.span()).into(),
            )
                .into(),
        ))
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Expr {
    Error(Error),
    Constant(Atom),
    Application(Box<Spanned<Self>>, Vec<Spanned<Self>>),
    Function(Spanned<String>, Vec<Spanned<String>>, Box<Spanned<Self>>),
    Local(Spanned<String>),
    Do(Vec<Spanned<Self>>),
    Let(Vec<Spanned<Self>>, Box<Spanned<Self>>),
    If(Box<Spanned<Expr>>, Box<Spanned<Expr>>),
    IfElse(Box<Spanned<Expr>>, Box<Spanned<Expr>>, Box<Spanned<Expr>>),
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Error(_) => write!(f, "Error"),
            Self::Local(name) => write!(f, "Local({})", name.node),
            Self::Constant(a) => write!(f, "{}", a),
            Self::Application(n, a) => write!(
                f,
                "App({},{:?})",
                n,
                a.iter()
                    .map(|p| p.node.to_string())
                    .collect::<Vec<String>>(),
            ),
            Self::Function(n, p, b) => write!(
                f,
                "Function({},{:?}, {})",
                n.node,
                p.iter()
                    .map(|p| p.node.to_string())
                    .collect::<Vec<String>>(),
                b.node,
            ),
            Self::Do(d) => write!(f, "{:?}", d),
            Self::Let(e, b) => write!(f, "Let({:?}, {})", e, b.node,),
            Self::If(c, e) => write!(f, "If({}, {})", c, e),
            Self::IfElse(c, e, o) => write!(f, "If({}, {}, {})", c, e, o),
        }
    }
}
