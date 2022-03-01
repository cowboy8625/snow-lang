use super::mini_parse::{either, pair, right, zero_or_more, Parser};
use super::{app, constant, local, next_token, parse_name, Expr, KeyWord, Span, Spanned, Token};

fn let_token<'a>() -> impl Parser<'a, Token, Spanned<Token>> {
    next_token(Token::KeyWord(KeyWord::Let))
}

fn indent_token<'a>() -> impl Parser<'a, Token, Spanned<Token>> {
    next_token(Token::InDent)
}

fn ident_let_token<'a>() -> impl Parser<'a, Token, Spanned<Token>> {
    right(next_token(Token::InDent), let_token())
}

fn parse_let<'a>() -> impl Parser<'a, Token, Span> {
    move |input: &'a [Spanned<Token>]| {
        let (i, t) = either(let_token(), ident_let_token()).parse(input)?;
        Ok((i, t.span()))
    }
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

pub(crate) fn let_expr<'a>() -> impl Parser<'a, Token, Spanned<Expr>> {
    move |input: &'a [Spanned<Token>]| {
        // Let
        let (i, start) = parse_let().parse(input)?;
        // expr
        let (i, (first, mut args)) = right(
            zero_or_more(next_token(Token::DeDent)),
            pair(let_function(), zero_or_more(right(comma(), let_function()))),
        )
        .parse(i)?;
        args.insert(0, first);
        // In
        let (i, _) = right(
            zero_or_more(next_token(Token::DeDent)),
            next_token(Token::KeyWord(KeyWord::In)),
        )
        .parse(i)?;

        // Let(String, Box<Spanned<Self>>, Box<Spanned<Self>>),
        // Return/Body?
        let (i, body) = right(zero_or_more(indent_token()), parse_body()).parse(i)?;
        let r#let = Expr::Let(args, Box::new(body.clone()));
        let span = (start, body.span()).into();
        Ok((i, (r#let, span).into()))
        // let r#let = args.iter().rev().fold(body, |acc, expr| {
        //     (
        //         Expr::Let(Box::new(expr.clone()), Box::new(acc.clone())),
        //         (expr.span(), acc.span()).into(),
        //     )
        //         .into()
        // });
        // Ok((i, r#let))
    }
}

// TODO bring back the do in let
// pub(crate) fn let_expr_do<'a>() -> impl Parser<'a, Token, Spanned<Expr>> {
//     move |input: &'a [Spanned<Token>]| {
//         let (i, _start) = parse_let().parse(input)?;
//         // expr
//         let (i, (first, mut args)) =
//             pair(let_function(), zero_or_more(right(comma(), let_function()))).parse(i)?;
//         args.insert(0, first);
//
//         let (i, body) = right(zero_or_more(indent_token()), parse_body()).parse(i)?;
//         let r#let = args.iter().rev().fold(body, |acc, expr| {
//             (
//                 Expr::Let(Box::new(expr.clone()), Box::new(acc.clone())),
//                 (expr.span(), acc.span()).into(),
//             )
//                 .into()
//         });
//         Ok((i, r#let))
//     }
// }
