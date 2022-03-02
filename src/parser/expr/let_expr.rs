use super::mini_parse::{either, left, pair, right, zero_or_more, zero_or_one, Parser};
use super::{
    app, constant, dedent_token, indent_token, local, next_token, parse_name, Expr, KeyWord, Span,
    Spanned, Token,
};

fn let_token<'a>() -> impl Parser<'a, Token, Spanned<Token>> {
    next_token(Token::KeyWord(KeyWord::Let))
}

fn parse_body<'a>() -> impl Parser<'a, Token, Spanned<Expr>> {
    either(app(), either(local(), constant()))
}

fn comma<'a>() -> impl Parser<'a, Token, Spanned<Token>> {
    next_token(Token::Ctrl(','))
}

pub(crate) fn let_function<'a>() -> impl Parser<'a, Token, Spanned<Expr>> {
    move |input: &'a [Spanned<Token>]| {
        let expr = either(app(), constant());
        let (i, name) = parse_name().parse(input)?;
        let (i, prams) = zero_or_more(parse_name()).parse(i)?;
        let (i, _) = next_token(Token::Op("=")).parse(i)?;
        let (i, body) = expr.parse(i)?;
        Ok((
            i,
            (
                Expr::Function(name.clone(), prams, Box::new(body.clone())),
                Span::new(name.span().start, body.span.end, &name.span().loc),
            )
                .into(),
        ))
    }
}

pub(crate) fn let_binding<'a>() -> impl Parser<'a, Token, (Spanned<Expr>, Vec<Spanned<Expr>>)> {
    left(
        pair(let_function(), zero_or_more(right(comma(), let_function()))),
        zero_or_one(comma()),
    )
}

pub(crate) fn let_expr<'a>() -> impl Parser<'a, Token, Spanned<Expr>> {
    move |input: &'a [Spanned<Token>]| {
        // Let
        let (i, start) = let_token().parse(input)?;
        // expr
        let (i, (first, mut args)) = either(
            right(indent_token(), left(let_binding(), dedent_token())),
            let_binding(),
        )
        .parse(i)?;
        args.insert(0, first);
        // In
        let (i, _) = next_token(Token::KeyWord(KeyWord::In)).parse(i)?;

        // Let(String, Box<Spanned<Self>>, Box<Spanned<Self>>),
        // Return/Body?
        let (i, body) = parse_body().parse(i)?;
        let r#let = Expr::Let(args, Box::new(body.clone()));
        let span = (start.span(), body.span()).into();
        Ok((i, (r#let, span).into()))
    }
}
