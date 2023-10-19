use super::expr::{Atom, Expr};
use super::op::Op as Oper;
use super::op::Op::*;
use snowc_lexer::{Ctrl, Ident, KeyWord, Op, Scanner, Span, Token, TokenPosition};

pub fn parse(src: &str) -> Result<Vec<Expr>, Vec<crate::error::Error>> {
    let mut tokens: Vec<Token> = Scanner::new(src).collect();
    let mut ast = Vec::new();
    while tokens.len() > 0 {
        let Some(func) = function(&mut tokens) else {
            let expr = expression(&mut tokens);
            ast.push(expr);
            continue;
        };
        ast.push(func);
    }
    Ok(ast)
}

/// Functions are just syntax sugar for closures.
/// ```haskell
/// add x y = x + y
/// -- is the same as
/// add = (\x -> (\y -> x + y))
/// ```
fn function(tokens: &mut Vec<Token>) -> Option<Expr> {
    let Some(Token::Ident(Ident{lexme: name, span: start, ..})) = tokens.get(0).cloned() else {
        return None;
    };
    tokens.remove(0);
    let args = get_function_args(tokens);
    consume(
        tokens,
        |t| matches!(t, Token::Ctrl(Ctrl{lexme, ..}) if lexme == "="),
    );
    let body = get_block(tokens);
    let end = body.span();
    let closures = create_closures(args, body);
    let span = Span::from((start, end));
    // Func(String, Box<Self>, Span),
    Some(Expr::Func(name, Box::new(closures), span))
}

fn get_block(tokens: &mut Vec<Token>) -> Expr {
    expression(tokens)
}

fn create_closures(args: Vec<Expr>, body: Expr) -> Expr {
    args.into_iter().rev().fold(body, |last, next| {
        let span = Span::from((last.span(), next.span()));
        Expr::Closure(Box::new(next), Box::new(last), span)
    })
}

fn get_function_args(tokens: &mut Vec<Token>) -> Vec<Expr> {
    let mut args = Vec::new();
    while let Some(Token::Ident(ident)) = tokens.get(0) {
        args.push(Expr::Atom(Atom::Id(ident.lexme.clone()), ident.span));
        tokens.remove(0);
    }
    args
}

fn a_operator(token: Option<&Token>) -> bool {
    match token {
        Some(Token::Op(_)) => true,
        Some(Token::Ctrl(Ctrl { lexme, .. })) if lexme != "(" => false,
        _ => false,
    }
}

// TODO: move call logic into its own function and move that call to be in the primary function.
fn expression(tokens: &mut Vec<Token>) -> Expr {
    equality(tokens)
}

fn equality(tokens: &mut Vec<Token>) -> Expr {
    let mut lhs = comparison(tokens);
    while let Some(op @ (Eq | Neq)) = get_op(tokens.get(0)) {
        tokens.remove(0);
        let rhs = comparison(tokens);
        let span = Span::from((lhs.span(), rhs.span()));
        lhs = Expr::Binary(op, Box::new(lhs), Box::new(rhs), span);
    }
    lhs
}

fn comparison(tokens: &mut Vec<Token>) -> Expr {
    let mut lhs = term(tokens);
    while let Some(op @ (Grt | Les | GrtEq | LesEq)) = get_op(tokens.get(0)) {
        tokens.remove(0);
        let rhs = term(tokens);
        let span = Span::from((lhs.span(), rhs.span()));
        lhs = Expr::Binary(op, Box::new(lhs), Box::new(rhs), span);
    }
    lhs
}

fn term(tokens: &mut Vec<Token>) -> Expr {
    let mut lhs = factor(tokens);
    while let Some(op @ (Plus | Minus)) = get_op(tokens.get(0)) {
        tokens.remove(0);
        let rhs = factor(tokens);
        let span = Span::from((lhs.span(), rhs.span()));
        lhs = Expr::Binary(op, Box::new(lhs), Box::new(rhs), span);
    }
    lhs
}

fn factor(tokens: &mut Vec<Token>) -> Expr {
    let mut lhs = unary(tokens);
    while let Some(op @ (Mult | Div | Mod)) = get_op(tokens.get(0)) {
        tokens.remove(0);
        let rhs = unary(tokens);
        let span = Span::from((lhs.span(), rhs.span()));
        lhs = Expr::Binary(op, Box::new(lhs), Box::new(rhs), span);
    }
    lhs
}

fn unary(tokens: &mut Vec<Token>) -> Expr {
    if let Some(op @ (Minus | Not)) = get_op(tokens.get(0)) {
        let token = tokens.remove(0);
        let rhs = unary(tokens);
        let span = Span::from((token.span(), rhs.span()));
        return Expr::Unary(op, Box::new(rhs), span);
    }
    call(tokens)
}

fn call(tokens: &mut Vec<Token>) -> Expr {
    let expr = primary(tokens);
    let Expr::Atom(Atom::Id(func_name), start) = expr.clone() else {
        return expr;
    };

    if !is_atom(tokens.get(0)) || is_deliminator(&tokens) {
        return expr;
    }

    let mut args = Vec::new();
    while !tokens.is_empty() {
        if !is_atom(tokens.get(0)) || is_deliminator(&tokens) {
            break;
        }

        args.push(primary(tokens));

        if !is_atom(tokens.get(0)) || is_deliminator(&tokens) {
            break;
        }
    }
    let end = args.last().map(|e| e.span()).unwrap_or(start);

    return Expr::App(
        Box::new(Expr::Atom(Atom::Id(func_name), start)),
        args,
        Span::from((start, end)),
    );
}

fn primary(tokens: &mut Vec<Token>) -> Expr {
    match tokens.remove(0) {
        Token::Int(num) => {
            Expr::Atom(Atom::Int(num.lexme.parse().unwrap_or_default()), num.span)
        }
        Token::Float(float) => Expr::Atom(
            Atom::Float(float.lexme.parse().unwrap_or_default()),
            float.span,
        ),
        Token::Ident(id) => Expr::Atom(Atom::Id(id.lexme), id.span),
        Token::KeyWord(kw) if kw.lexme == "true" => {
            Expr::Atom(Atom::Bool(kw.lexme.parse().unwrap_or_default()), kw.span)
        }
        Token::KeyWord(kw) if kw.lexme == "false" => {
            Expr::Atom(Atom::Bool(kw.lexme.parse().unwrap_or_default()), kw.span)
        }
        Token::Str(string) => Expr::Atom(Atom::String(string.lexme), string.span),
        Token::Char(c) => {
            Expr::Atom(Atom::Char(c.lexme.parse().unwrap_or_default()), c.span)
        }
        Token::Ctrl(c) if c.lexme == "(" => {
            // TODO: handle parens a bit better
            let expr = expression(tokens);
            let left_paren = tokens.remove(0);
            // TODO: report error in one location for parens
            let Token::Ctrl(Ctrl{lexme, ..}) = left_paren else {
                // TODO: report error
                panic!("missing right paren");
            };
            if lexme != ")" {
                // TODO: report error
                panic!("missing right paren");
            }
            expr
        }
        _ => unreachable!("unexpected token {:?}", tokens.get(0)),
    }
}

fn is_atom(token: Option<&Token>) -> bool {
    matches!(
        token,
        Some(Token::Int(_) | Token::Float(_) | Token::Char(_) | Token::Str(_))
    ) || matches!(token, Some(Token::KeyWord(KeyWord { lexme, .. })) if matches!(lexme.as_str(), "true" | "false"))
        || matches!(token, Some(Token::Ctrl(Ctrl { lexme, .. })) if lexme.as_str() == "(")
}

fn get_op(token: Option<&Token>) -> Option<Oper> {
    token.and_then(|t| {
        t.map_op::<Option<Oper>>(|Op { lexme, .. }| crate::op::Op::try_from(lexme).ok())
            .flatten()
    })
}

fn consume<F>(tokens: &mut Vec<Token>, func: F)
where
    F: FnOnce(&Token) -> bool,
{
    if func(&tokens.remove(0)) {
        return;
    }
    panic!("unexpected token {:?}", tokens[0]);
}

fn is_deliminator(tokens: &[Token]) -> bool {
    let Some(first) = tokens.get(0) else {
        return false;
    };
    let pos1 = first.position();

    let Some(second) = tokens.get(1) else {
        return false;
    };

    let pos2 = second.position();

    matches!((pos1, pos2), (TokenPosition::End, TokenPosition::Start))
}

#[test]
fn parse_test() {
    use pretty_assertions::assert_eq;
    let ast = parse("add x y = x + y\nmain = print (add 1 2)");
    let left = ast
        .unwrap_or_default()
        .iter()
        .map(ToString::to_string)
        .collect::<Vec<_>>();
    assert_eq!(left, vec!["<add: (\\x -> (\\y -> (+ x y)))>"]);
}
