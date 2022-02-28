use super::{lambda, mini_parse::one_or_more, Expr, FunctionList, Parser, Spanned, Token};
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
