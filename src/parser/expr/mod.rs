mod let_expr;
use super::mini_parse::{self, either, left, one_or_more, right, zero_or_more, Parser};
use super::{boolean, builtin, keyword, number, string, Atom, KeyWord, Span, Spanned, Token};
use let_expr::{let_expr_app, let_expr_do};
use std::fmt;

pub fn constant<'a>() -> impl Parser<'a, Token, Spanned<Expr>> {
    move |input: &'a [Spanned<Token>]| {
        either(
            builtin().map(|b| (Atom::BuiltIn(b.node.clone()), b.span()).into()),
            either(boolean(), either(keyword(), either(string(), number()))),
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
    parse_name().map(|s| (Expr::Local(s.node.clone()), s.span()).into())
}

fn next_token<'a>(token: Token) -> impl Parser<'a, Token, Spanned<Token>> {
    move |input: &'a [Spanned<Token>]| match input.get(0) {
        Some(node) if node.node == token => Ok((&input[1..], node.clone())),
        _ => Err(input),
    }
}

fn do_block<'a>() -> impl Parser<'a, Token, Spanned<Expr>> {
    move |input: &'a [Spanned<Token>]| {
        let (i, span_start) = next_token(Token::KeyWord(KeyWord::Do)).parse(input)?;
        // TODO FIXME: constant expr should be in here?
        // LOOK IT UP DUDE!
        let (i, body) = one_or_more(right(
            // TODO: Make a custom parser for indents.
            next_token(Token::InDent(4)),
            either(let_expr_do(), either(app(), constant())),
        ))
        .parse(i)?;
        let span: Span = (span_start.span(), body.last().unwrap().span()).into();
        Ok((i, (Expr::Do(body), span).into()))
    }
}

fn prans<'a>() -> impl Parser<'a, Token, Spanned<Expr>> {
    right(
        next_token(Token::Ctrl('(')),
        left(
            either(app(), either(local(), constant())),
            next_token(Token::Ctrl(')')),
        ),
    )
}

pub fn app<'a>() -> impl Parser<'a, Token, Spanned<Expr>> {
    move |input: &'a [Spanned<Token>]| {
        let (i, name) = either(
            local(),
            builtin().map(|s| (Expr::Constant(Atom::BuiltIn(s.node.clone())), s.span()).into()),
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

pub(crate) fn lambda<'a>() -> impl Parser<'a, Token, Spanned<Expr>> {
    move |input: &'a [Spanned<Token>]| {
        let expr = either(
            let_expr_app(),
            either(do_block(), either(app(), constant())),
        );
        let (i, start) = next_token(Token::DeDent).parse(input)?;
        let (i, name) = parse_name().parse(i)?;
        let (i, prams) = zero_or_more(parse_name()).parse(i)?;
        let (i, _) = next_token(Token::Op("=")).parse(i)?;
        let (i, body) = expr.parse(i)?;
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

#[derive(Debug, PartialEq, Clone)]
pub enum Expr {
    Constant(Atom),
    // func-name args
    Application(Box<Self>, Vec<Spanned<Self>>),
    // func-name prams body
    Lambda(Spanned<String>, Vec<Spanned<String>>, Box<Spanned<Self>>),
    // func name's or pram name's
    // TODO: Turn into Spanned<String>
    Local(String),
    // do block
    Do(Vec<Spanned<Self>>),
    //   name       bindings            body
    Let(String, Box<Spanned<Self>>, Box<Spanned<Self>>),
    // (if predicate do-this)
    // If(Box<Expr>, Box<Expr>),
    // // (if predicate do-this otherwise-do-this)
    // IfElse(Box<Expr>, Box<Expr>, Box<Expr>),
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Local(name) => write!(f, "Local({})", name),
            Self::Constant(a) => write!(f, "{}", a),
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
            Self::Do(d) => write!(f, "{:?}", d),
            Self::Let(n, e, b) => write!(f, "Let({}, {}, {})", n, e.node, b.node,),
        }
    }
}
