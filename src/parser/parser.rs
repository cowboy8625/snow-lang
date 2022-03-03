use super::{
    function, mini_parse::one_or_more, Expr, Function, FunctionList, Parser, Spanned, Token,
};
pub fn parser<'a>() -> impl Parser<'a, Token, FunctionList> {
    move |input: &'a [Spanned<Token>]| {
        let (i, result) = one_or_more(function()).parse(input)?;
        let mut funcs = FunctionList::new();
        for f in result.iter() {
            match &f.node {
                Expr::Function(name, prams, body) => funcs.insert(
                    name.node.clone(),
                    Function::new(
                        &name.node,
                        prams,
                        body.node.clone(),
                        (name.span(), body.span()).into(),
                    ),
                ),
                x => unreachable!(x),
            };
        }
        Ok((i, funcs))
    }
}
