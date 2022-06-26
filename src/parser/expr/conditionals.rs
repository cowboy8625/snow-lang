use super::mini_parse::{either, pair, right, surround, zero_or_more, zero_or_one, Parser};
use super::{
    app, constant, dedent_token, indent_token, left, next_token, Expr, KeyWord, Spanned, Token,
};

// DeDent,
// Id("main".into()),
// Op("="),
// KeyWord(If),
// KeyWord(True),
// KeyWord(Then),
// KeyWord(PrintLn),
// String("If".into()),
// KeyWord(Else),
// KeyWord(If),
// KeyWord(False),
// KeyWord(Then),
// KeyWord(PrintLn),
// String("Else If".into()),
// KeyWord(Else),
// KeyWord(PrintLn),
// String("Else".into()),
//
// (if predicate do-this)
// If(Box<Expr>, Box<Expr>),
// (if predicate do-this otherwise-do-this)
// IfElse(Box<Expr>, Box<Expr>, Box<Expr>),
//
// main = do
// If(Box<Expr>, Box<Expr>),
// IfElse(True, ,println "If", IfElse(False, println "Else If", println "Else")),
//     if True then
//         println "If"
//     else if False then
//         println "Else If"
//     else
//         println "Else"

pub fn conditional<'a>() -> impl Parser<'a, Token, Spanned<Expr>> {
    move |input: &'a [Spanned<Token>]| {
        let (i, (r#if, (mut else_if, r#else))) = pair(
            if_expr(),
            pair(zero_or_more(else_if_expr()), zero_or_one(else_expr())),
        )
        .parse(input)?;
        else_if.insert(0, r#if);
        let expression = else_if
            .iter()
            .rev()
            .fold(r#else, |acc, (pred, true_branch)| {
                Some(if let Some(false_branch) = acc {
                    (
                        Expr::IfElse(
                            Box::new(pred.clone()),
                            Box::new(true_branch.clone()),
                            Box::new(false_branch.clone()),
                        ),
                        (pred.span(), false_branch.span()).into(),
                    )
                        .into()
                } else {
                    (
                        Expr::If(Box::new(pred.clone()), Box::new(true_branch.clone())),
                        (pred.span(), true_branch.span()).into(),
                    )
                        .into()
                })
            });
        Ok((i, expression.unwrap()))
    }
}
pub fn else_expr<'a>() -> impl Parser<'a, Token, Spanned<Expr>> {
    right(else_token(), either(expr(), left(expr(), dedent_token())))
}
pub fn else_if_expr<'a>() -> impl Parser<'a, Token, (Spanned<Expr>, Spanned<Expr>)> {
    pair(
        right(
            pair(else_token(), if_token()),
            left(condition(), then_token()),
        ),
        either(expr(), left(expr(), dedent_token())),
    )
}
pub fn if_expr<'a>() -> impl Parser<'a, Token, (Spanned<Expr>, Spanned<Expr>)> {
    pair(
        right(if_token(), left(condition(), then_token())),
        either(expr(), left(expr(), dedent_token())),
    )
}
pub fn condition<'a>() -> impl Parser<'a, Token, Spanned<Expr>> {
    either(app(), constant())
}

fn expr<'a>() -> impl Parser<'a, Token, Spanned<Expr>> {
    either(
        surround(indent_token(), either(app(), constant()), dedent_token()),
        either(app(), constant()),
    )
}

fn if_token<'a>() -> impl Parser<'a, Token, Spanned<Token>> {
    next_token(Token::KeyWord(KeyWord::If))
}

fn then_token<'a>() -> impl Parser<'a, Token, Spanned<Token>> {
    next_token(Token::KeyWord(KeyWord::Then))
}

fn else_token<'a>() -> impl Parser<'a, Token, Spanned<Token>> {
    next_token(Token::KeyWord(KeyWord::Else))
}
