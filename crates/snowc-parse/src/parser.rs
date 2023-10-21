use crate::expr::Binary;

use super::expr::{Atom, Expr, Unary};
use super::op::Op as Oper;
use super::op::Op::*;
use snowc_lexer::{Ctrl, Ident, KeyWord, Op, Scanner, Span, Token, TokenPosition};

pub fn parse(src: &str) -> Result<Vec<Expr>, Vec<crate::error::Error>> {
    let mut tokens: Vec<Token> = Scanner::new(src).collect();
    let mut ast = Vec::new();
    while tokens.len() > 0 {
        let Some(func) = function(&mut tokens) else {
            panic!("\nonly function defs are allowed in global scope. leftovers: {:?}\n{:#?}", tokens.len(), tokens);
        };
        ast.push(func);
    }
    Ok(ast)
}

/// Functions are just syntax sugar for closures.
/// ```hs
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
    let typed = get_function_type(tokens);
    consume_ctrl(tokens, "=");
    let body = get_block(tokens);
    let end = body.span();
    let closures = create_closures(args, body);
    let span = Span::from((start, end));
    Some(Expr::Func(name, typed, Box::new(closures), span))
}

fn get_block(tokens: &mut Vec<Token>) -> Expr {
    if_expression(tokens)
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
        args.push(Expr::Atom(Atom::Id(ident.lexme.clone(), ident.span)));
        tokens.remove(0);
    }
    args
}

fn get_function_type(tokens: &mut Vec<Token>) -> Vec<Ident> {
    let mut types = Vec::new();
    if consume_ctrl_if(tokens, ":").is_none() {
        return types;
    }

    while !matches!(tokens.get(0), Some(Token::Ctrl(Ctrl{lexme, ..})) if lexme == "=") {
        let Some(Token::Ident(ident)) = tokens.get(0).cloned() else {
            break;
        };
        types.push(ident);
        tokens.remove(0);
        if consume_ctrl_if(tokens, "->").is_none() {
            break;
        }
    }
    types
}

fn if_expression(tokens: &mut Vec<Token>) -> Expr {
    let Some(Token::KeyWord(KeyWord{span: start, ..})) = consume_keyword_if(tokens, "if") else {
        return expression(tokens);
    };
    let condition = expression(tokens);
    consume_keyword(tokens, "then");
    let true_branch = expression(tokens);
    consume_keyword(tokens, "else");
    let false_branch = expression(tokens);
    let span = Span::from((start, false_branch.span()));
    Expr::IfElse(
        Box::new(condition),
        Box::new(true_branch),
        Box::new(false_branch),
        span,
    )
}

fn expression(tokens: &mut Vec<Token>) -> Expr {
    equality(tokens)
}

fn equality(tokens: &mut Vec<Token>) -> Expr {
    let mut lhs = comparison(tokens);
    while let Some(op @ (Eq | Neq)) = get_op(tokens.get(0)) {
        tokens.remove(0);
        let rhs = comparison(tokens);
        let span = Span::from((lhs.span(), rhs.span()));
        let left = Box::new(lhs);
        let right = Box::new(rhs);
        lhs = Expr::Binary(Binary {
            op,
            left,
            right,
            span,
        });
    }
    lhs
}

fn comparison(tokens: &mut Vec<Token>) -> Expr {
    let mut lhs = term(tokens);
    while let Some(op @ (Grt | Les | GrtEq | LesEq)) = get_op(tokens.get(0)) {
        tokens.remove(0);
        let rhs = term(tokens);
        let span = Span::from((lhs.span(), rhs.span()));
        let left = Box::new(lhs);
        let right = Box::new(rhs);
        lhs = Expr::Binary(Binary {
            op,
            left,
            right,
            span,
        });
    }
    lhs
}

fn term(tokens: &mut Vec<Token>) -> Expr {
    let mut lhs = factor(tokens);
    while let Some(op @ (Plus | Minus)) = get_op(tokens.get(0)) {
        tokens.remove(0);
        let rhs = factor(tokens);
        let span = Span::from((lhs.span(), rhs.span()));
        let left = Box::new(lhs);
        let right = Box::new(rhs);
        lhs = Expr::Binary(Binary {
            op,
            left,
            right,
            span,
        });
    }
    lhs
}

fn factor(tokens: &mut Vec<Token>) -> Expr {
    let mut lhs = unary(tokens);
    while let Some(op @ (Mult | Div | Mod)) = get_op(tokens.get(0)) {
        tokens.remove(0);
        let rhs = unary(tokens);
        let span = Span::from((lhs.span(), rhs.span()));
        let left = Box::new(lhs);
        let right = Box::new(rhs);
        lhs = Expr::Binary(Binary {
            op,
            left,
            right,
            span,
        });
    }
    lhs
}

fn unary(tokens: &mut Vec<Token>) -> Expr {
    if let Some(op @ (Minus | Not)) = get_op(tokens.get(0)) {
        let token = tokens.remove(0);
        let rhs = unary(tokens);
        let span = Span::from((token.span(), rhs.span()));
        let expr = Box::new(rhs);
        return Expr::Unary(Unary { op, expr, span });
    }
    call(tokens)
}

fn call(tokens: &mut Vec<Token>) -> Expr {
    let pos1 = tokens
        .get(0)
        .map(|t| t.position())
        .cloned()
        .unwrap_or(TokenPosition::Start);
    let expr = primary(tokens);
    let Expr::Atom(Atom::Id(func_name, start)) = expr.clone() else {
        return expr;
    };
    let mut pos2 = tokens
        .get(0)
        .map(|t| t.position())
        .cloned()
        .unwrap_or(TokenPosition::End);
    let foo =
        |pos1, pos2| matches!((pos1, pos2), (TokenPosition::End, TokenPosition::Start));
    if !is_atom(tokens.get(0)) || foo(pos1, pos2) || is_keyword(tokens.get(0)) {
        return expr;
    }

    let mut pos3 = tokens
        .get(1)
        .map(|t| t.position())
        .cloned()
        .unwrap_or(TokenPosition::End);
    let mut args = Vec::new();
    while !tokens.is_empty() {
        eprintln!("{func_name} {:?}, {:?}", pos2, pos3);
        if !is_atom(tokens.get(0)) || foo(pos2, pos3) || is_keyword(tokens.get(0)) {
            eprintln!("break {:#?}", tokens.get(0));
            break;
        }

        args.push(primary(tokens));
        pos2 = pos3;
        pos3 = tokens
            .get(1)
            .map(|t| t.position())
            .cloned()
            .unwrap_or(TokenPosition::End);
    }
    let end = args.last().map(|e| e.span()).unwrap_or(start);

    return Expr::App(
        Box::new(Expr::Atom(Atom::Id(func_name, start))),
        args,
        Span::from((start, end)),
    );
}

fn primary(tokens: &mut Vec<Token>) -> Expr {
    let token = tokens.remove(0);
    match token {
        Token::Int(num) => {
            Expr::Atom(Atom::Int(num.lexme.parse().unwrap_or_default(), num.span))
        }
        Token::Float(float) => Expr::Atom(Atom::Float(
            float.lexme.parse().unwrap_or_default(),
            float.span,
        )),
        Token::Ident(id) => Expr::Atom(Atom::Id(id.lexme, id.span)),
        Token::KeyWord(kw) if kw.lexme == "true" => {
            Expr::Atom(Atom::Bool(kw.lexme.parse().unwrap_or_default(), kw.span))
        }
        Token::KeyWord(kw) if kw.lexme == "false" => {
            Expr::Atom(Atom::Bool(kw.lexme.parse().unwrap_or_default(), kw.span))
        }
        Token::Str(string) => Expr::Atom(Atom::String(string.lexme, string.span)),
        Token::Char(c) => {
            Expr::Atom(Atom::Char(c.lexme.parse().unwrap_or_default(), c.span))
        }
        Token::Ctrl(c) if c.lexme == "(" => {
            let expr = if_expression(tokens);
            consume_ctrl(tokens, ")");
            expr
        }
        _ => unreachable!(
            "unexpected token current: {token:?}\nNEXT1: {:?}\nNEXT2: {:?}",
            tokens.get(0),
            tokens.get(1)
        ),
    }
}

fn is_atom(token: Option<&Token>) -> bool {
    let Some(token) = token else {
        return false
    };
    let lexme = match token {
        Token::KeyWord(KeyWord { lexme, .. }) => lexme.as_str(),
        Token::Ctrl(Ctrl { lexme, .. }) => lexme.as_str(),
        Token::Ident(_)
        | Token::Int(_)
        | Token::Float(_)
        | Token::Char(_)
        | Token::Str(_) => return true,
        _ => return false,
    };

    match lexme {
        "true" | "false" | "(" => true,
        _ => false,
    }
}

fn is_keyword(token: Option<&Token>) -> bool {
    matches!(token, Some(Token::KeyWord(_)))
}

fn get_op(token: Option<&Token>) -> Option<Oper> {
    token.and_then(|t| {
        t.map_op::<Option<Oper>>(|Op { lexme, .. }| crate::op::Op::try_from(lexme).ok())
            .flatten()
    })
}

fn consume_ctrl(tokens: &mut Vec<Token>, expected: &str) {
    let token = tokens.remove(0);
    if matches!(&token, Token::Ctrl(Ctrl{lexme, ..}) if lexme != expected) {
        panic!("expected {expected:?} but got {:?}", token);
    }
}

fn consume_ctrl_if(tokens: &mut Vec<Token>, expected: &str) -> Option<Token> {
    let token = tokens.get(0);
    if !matches!(&token, Some(Token::Ctrl(Ctrl{lexme, ..})) if lexme == expected) {
        return None;
    }
    return Some(tokens.remove(0));
}

fn consume_keyword(tokens: &mut Vec<Token>, expected: &str) {
    let token = tokens.remove(0);
    if matches!(&token, Token::KeyWord(KeyWord{lexme, ..}) if lexme != expected) {
        panic!("expected {expected:?} but got {:?}", token);
    }
}

fn consume_keyword_if(tokens: &mut Vec<Token>, expected: &str) -> Option<Token> {
    let token = tokens.get(0);
    if !matches!(&token, Some(Token::KeyWord(KeyWord{lexme, ..})) if lexme == expected) {
        return None;
    }
    return Some(tokens.remove(0));
}

fn _is_deliminator(tokens: &[Token]) -> bool {
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
    let src = include_str!("./../../../samples/std.snow");
    let ast = parse(src);
    let left = ast
        .unwrap_or_default()
        .iter()
        .map(ToString::to_string)
        .collect::<Vec<_>>();
    assert_eq!(left, vec!["<add: (\\x -> (\\y -> (+ x y)))>"]);
}
