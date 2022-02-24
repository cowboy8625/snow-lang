use chumsky::prelude::*;
use std::collections::HashMap;

use super::token::{BlockType, KeyWord, Spanned, Span, Token};

#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    Boolean(bool),
    Int(i128),
    Float(f64),
    String(String),
    List(Vec<Value>),
    Func(String),
}

impl Value {
    fn int(self, span: Span) -> Result<i128, Error> {
        if let Value::Int(x) = self {
            Ok(x)
        } else {
            Err(Error {
                span,
                msg: format!("'{}' is not a Int", self),
            })
        }
    }

    fn float(self, span: Span) -> Result<f64, Error> {
        if let Value::Float(x) = self {
            Ok(x)
        } else {
            Err(Error {
                span,
                msg: format!("'{}' is not a Float", self),
            })
        }
    }
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Boolean(x) => write!(f, "{}", x),
            Self::Int(x) => write!(f, "{}", x),
            Self::Float(x) => write!(f, "{}", x),
            Self::String(x) => write!(f, "{}", x),
            Self::List(xs) => write!(
                f,
                "[{}]",
                xs.iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
            Self::Func(name) => write!(f, "<function: {}>", name),
        }
    }
}

#[derive(Clone, Debug)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
    Eq,
    NotEq,
}

// An expression node in the AST. Children are spanned so we can generate useful runtime errors.
#[derive(Debug)]
pub enum Expr {
    Error,
    Value(Value),
    List(Vec<Spanned<Self>>),
    Local(String),
    Let(String, Box<Spanned<Self>>, Box<Spanned<Self>>),
    Then(Box<Spanned<Self>>, Box<Spanned<Self>>),
    Binary(Box<Spanned<Self>>, BinaryOp, Box<Spanned<Self>>),
    Call(Box<Spanned<Self>>, Spanned<Vec<Spanned<Self>>>),
    If(Box<Spanned<Self>>, Box<Spanned<Self>>, Box<Spanned<Self>>),
    Print(Box<Spanned<Self>>),
    PrintLn(Box<Spanned<Self>>),
}

// A function node in the AST.
#[derive(Debug)]
pub struct Func {
    pub args: Vec<String>,
    pub body: Spanned<Expr>,
}

#[derive(Debug)]
pub struct Error {
    pub span: Span,
    pub msg: String,
}
pub fn expr_parser() -> impl Parser<Token, Spanned<Expr>, Error = Simple<Token>> + Clone {
    recursive(|expr| {
        let raw_expr = recursive(|raw_expr| {
            let val = filter_map(|span, tok| match tok {
                Token::KeyWord(KeyWord::True) => Ok(Expr::Value(Value::Boolean(true))),
                Token::KeyWord(KeyWord::False) => Ok(Expr::Value(Value::Boolean(false))),
                Token::Int(n) => Ok(Expr::Value(Value::Int(n.parse::<i128>().unwrap()))),
                Token::Float(n) => Ok(Expr::Value(Value::Float(n.parse::<f64>().unwrap()))),
                Token::String(s) => Ok(Expr::Value(Value::String(s))),
                _ => Err(Simple::expected_input_found(span, Vec::new(), Some(tok))),
            })
            .labelled("value");

            let ident = filter_map(|span, tok| match tok {
                Token::Id(ident) => Ok(ident.clone()),
                _ => Err(Simple::expected_input_found(span, Vec::new(), Some(tok))),
            })
            .labelled("identifier");

            let args = expr
                .clone()
                .repeated()
                .or_not()
                .map(|item| item.unwrap_or_else(Vec::new));

            // A list of expressions
            let items = expr
                .clone()
                .chain(just(Token::Ctrl(',')).ignore_then(expr.clone()).repeated())
                .then_ignore(just(Token::Ctrl(',')).or_not())
                .or_not()
                .map(|item| item.unwrap_or_else(Vec::new));

            // A let expression
            let let_ = just(Token::OpenBlock(BlockType::Let))
                .ignore_then(ident)
                .then_ignore(just(Token::Op("=".to_string())))
                .then(raw_expr)
                .then_ignore(just(Token::CloseBlock(BlockType::Let)))
                .then(expr.clone())
                .map(|((name, val), body)| Expr::Let(name, Box::new(val), Box::new(body)));

            let list = items
                .clone()
                .delimited_by(just(Token::Ctrl('[')), just(Token::Ctrl(']')))
                .map(Expr::List);

            // 'Atoms' are expressions that contain no ambiguity
            let atom = val
                .or(ident.map(Expr::Local))
                .or(let_)
                .or(list)
                // In Nano Rust, `print` is just a keyword, just like Python 2, for simplicity
                .or(just(Token::KeyWord(KeyWord::Print))
                    .ignore_then(
                        expr.clone()
                            .delimited_by(just(Token::OpenBlock(BlockType::Arg)), just(Token::CloseBlock(BlockType::Arg))),
                    )
                    .map(|expr| Expr::Print(Box::new(expr))))
                .or(just(Token::KeyWord(KeyWord::PrintLn))
                    .ignore_then(
                        expr.clone()
                            .delimited_by(just(Token::OpenBlock(BlockType::Arg)), just(Token::CloseBlock(BlockType::Arg))),
                    )
                    .map(|expr| Expr::PrintLn(Box::new(expr))))
                .map_with_span(|expr, span| (expr, span))
                // Atoms can also just be normal expressions, but surrounded with parentheses
                .or(expr
                    .clone()
                    .delimited_by(just(Token::Ctrl('(')), just(Token::Ctrl(')'))))
                // Attempt to recover anything that looks like a parenthesised expression but contains errors
                .recover_with(nested_delimiters(
                    Token::Ctrl('('),
                    Token::Ctrl(')'),
                    [
                        (Token::Ctrl('['), Token::Ctrl(']')),
                        (Token::Ctrl('{'), Token::Ctrl('}')),
                        (Token::OpenBlock(BlockType::Fn), Token::CloseBlock(BlockType::Fn)),
                        (Token::OpenBlock(BlockType::FnBlock), Token::CloseBlock(BlockType::FnBlock)),
                        (Token::OpenBlock(BlockType::Pram), Token::CloseBlock(BlockType::Pram)),
                        (Token::OpenBlock(BlockType::Let), Token::CloseBlock(BlockType::Let)),
                        (Token::OpenBlock(BlockType::Do), Token::CloseBlock(BlockType::Do)),
                    ],
                    |span| (Expr::Error, span),
                ))
                // Attempt to recover anything that looks like a list but contains errors
                .recover_with(nested_delimiters(
                    Token::Ctrl('['),
                    Token::Ctrl(']'),
                    [
                        (Token::Ctrl('('), Token::Ctrl(')')),
                        (Token::Ctrl('{'), Token::Ctrl('}')),
                        (Token::OpenBlock(BlockType::Fn), Token::CloseBlock(BlockType::Fn)),
                        (Token::OpenBlock(BlockType::FnBlock), Token::CloseBlock(BlockType::FnBlock)),
                        (Token::OpenBlock(BlockType::Pram), Token::CloseBlock(BlockType::Pram)),
                        (Token::OpenBlock(BlockType::Let), Token::CloseBlock(BlockType::Let)),
                        (Token::OpenBlock(BlockType::Do), Token::CloseBlock(BlockType::Do)),
                    ],
                    |span| (Expr::Error, span),
                ));

            // Function calls have very high precedence so we prioritise them
            let call = atom
                .then(
                    args.delimited_by(
                        just(Token::OpenBlock(BlockType::Arg)),
                        just(Token::CloseBlock(BlockType::Arg)),
                    )
                    .map_with_span(|args, span| (args, span))
                    .repeated(),
                )
                .foldl(|f, args| {
                    let span = f.1.start..args.1.end;
                    (Expr::Call(Box::new(f), args), span)
                });

            // Product ops (multiply and divide) have equal precedence
            let op = just(Token::Op("*".to_string()))
                .to(BinaryOp::Mul)
                .or(just(Token::Op("/".to_string())).to(BinaryOp::Div));
            let product = call
                .clone()
                .then(op.then(call).repeated())
                .foldl(|a, (op, b)| {
                    let span = a.1.start..b.1.end;
                    (Expr::Binary(Box::new(a), op, Box::new(b)), span)
                });

            // Sum ops (add and subtract) have equal precedence
            let op = just(Token::Op("+".to_string()))
                .to(BinaryOp::Add)
                .or(just(Token::Op("-".to_string())).to(BinaryOp::Sub));
            let sum = product
                .clone()
                .then(op.then(product).repeated())
                .foldl(|a, (op, b)| {
                    let span = a.1.start..b.1.end;
                    (Expr::Binary(Box::new(a), op, Box::new(b)), span)
                });

            // Comparison ops (equal, not-equal) have equal precedence
            let op = just(Token::Op("==".to_string()))
                .to(BinaryOp::Eq)
                .or(just(Token::Op("!=".to_string())).to(BinaryOp::NotEq));
            let compare = sum
                .clone()
                .then(op.then(sum).repeated())
                .foldl(|a, (op, b)| {
                    let span = a.1.start..b.1.end;
                    (Expr::Binary(Box::new(a), op, Box::new(b)), span)
                });

            compare
        });

        // Blocks are expressions but delimited with braces
        let block = expr
            .clone()
            .delimited_by(just(Token::OpenBlock(BlockType::Do)), just(Token::CloseBlock(BlockType::Do)))
            // Attempt to recover anything that looks like a block but contains errors
            .recover_with(nested_delimiters(
                Token::Ctrl('{'),
                Token::Ctrl('}'),
                [
                    (Token::Ctrl('('), Token::Ctrl(')')),
                    (Token::Ctrl('['), Token::Ctrl(']')),
                ],
                |span| (Expr::Error, span),
            ));
        // Blocks are expressions but delimited with braces
        let _doblock = expr
            .clone()
            .delimited_by(just(Token::OpenBlock(BlockType::Do)), just(Token::CloseBlock(BlockType::Do)))
            // Attempt to recover anything that looks like a block but contains errors
            .recover_with(nested_delimiters(
                Token::OpenBlock(BlockType::Do),
                Token::CloseBlock(BlockType::Do),
                [
                    (Token::Ctrl('{'), Token::Ctrl('}')),
                    (Token::Ctrl('('), Token::Ctrl(')')),
                    (Token::Ctrl('['), Token::Ctrl(']')),
                ],
                |span| (Expr::Error, span),
            ));

        let if_ = recursive(|if_| {
            just(Token::KeyWord(KeyWord::If))
                .ignore_then(expr.clone())
                .then(block.clone())
                .then(
                    just(Token::KeyWord(KeyWord::Else))
                        .ignore_then(block.clone().or(if_))
                        .or_not(),
                )
                .map_with_span(|((cond, a), b), span| {
                    (
                        Expr::If(
                            Box::new(cond),
                            Box::new(a),
                            Box::new(match b {
                                Some(b) => b,
                                // If an `if` expression has no trailing `else` block, we magic up one that just produces null
                                None => panic!("Must have return."),
                            }),
                        ),
                        span,
                    )
                })
        });

        // Both blocks and `if` are 'block expressions' and can appear in the place of statements
        let block_expr = block.or(if_).labelled("block");

        let block_chain = block_expr
            .clone()
            .then(block_expr.clone().repeated())
            .foldl(|a, b| {
                let span = a.1.start..b.1.end;
                (Expr::Then(Box::new(a), Box::new(b)), span)
            });

        block_chain
            // Expressions, chained by semicolons, are statements
            .or(raw_expr.clone())
            .then(just(Token::Ctrl(';')).ignore_then(expr.or_not()).repeated())
            .foldl(|a, b| {
                let span = a.1.clone(); // TODO: Not correct
                (
                    Expr::Then(
                        Box::new(a),
                        Box::new(match b {
                            Some(b) => b,
                            None => panic!("Must have a return"),
                        }),
                    ),
                    span,
                )
            })
    })
}

pub fn funcs_parser() -> impl Parser<Token, HashMap<String, Func>, Error = Simple<Token>> + Clone {
    let ident = filter_map(|span, tok| match tok {
        Token::Id(ident) => Ok(ident.clone()),
        _ => Err(Simple::expected_input_found(span, Vec::new(), Some(tok))),
    });

    // Pramument lists are just identifiers separated by commas, surrounded by parentheses
    let args = ident
        .clone()
        .repeated()
        // .separated_by(just(Token::Ctrl(',')))
        // .allow_trailing()
        .delimited_by(
            just(Token::OpenBlock(BlockType::Pram)),
            just(Token::CloseBlock(BlockType::Pram)),
        ).or_not()
        .map(|item| item.unwrap_or_else(Vec::new))
        .labelled("function args");

    let func = just(Token::OpenBlock(BlockType::Fn))
        .ignore_then(
            ident
                .map_with_span(|name, span| (name, span))
                .labelled("function name"),
        )
        .then(args)
        .then_ignore(just(Token::Op("=".into())))
        .then(
            expr_parser()
                .delimited_by(just(Token::OpenBlock(BlockType::FnBlock)), just(Token::CloseBlock(BlockType::FnBlock)))
                // Attempt to recover anything that looks like a function body but contains errors
                .recover_with(nested_delimiters(
                    Token::OpenBlock(BlockType::FnBlock),
                    Token::CloseBlock(BlockType::FnBlock),
                    [
                        (Token::Ctrl('('), Token::Ctrl(')')),
                        (Token::Ctrl('['), Token::Ctrl(']')),
                    ],
                    |span| (Expr::Error, span),
                )),
        )
        .then_ignore(just(Token::CloseBlock(BlockType::Fn)))
        .map(|((name, args), body)| (name, Func { args, body }))
        .labelled("function");

    func.repeated()
        .try_map(|fs, _| {
            let mut funcs = HashMap::new();
            for ((name, name_span), f) in fs {
                if funcs.insert(name.clone(), f).is_some() {
                    return Err(Simple::custom(
                        name_span.clone(),
                        format!("Function '{}' already exists", name),
                    ));
                }
            }
            Ok(funcs)
        })
        .then_ignore(end())
}

pub fn eval_expr(
    expr: &Spanned<Expr>,
    funcs: &HashMap<String, Func>,
    stack: &mut Vec<(String, Value)>,
) -> Result<Value, Error> {
    Ok(match &expr.0 {
        Expr::Error => unreachable!(), // Error expressions only get created by parser errors, so cannot exist in a valid AST
        Expr::Value(val) => val.clone(),
        Expr::List(items) => Value::List(
            items
                .iter()
                .map(|item| eval_expr(item, funcs, stack))
                .collect::<Result<_, _>>()?,
        ),
        Expr::Local(name) => stack
            .iter()
            .rev()
            .find(|(l, _)| l == name)
            .map(|(_, v)| v.clone())
            .or_else(|| Some(Value::Func(name.clone())).filter(|_| funcs.contains_key(name)))
            .ok_or_else(|| Error {
                span: expr.1.clone(),
                msg: format!("No such variable '{}' in scope", name),
            })?,
        Expr::Let(local, val, body) => {
            let val = eval_expr(val, funcs, stack)?;
            stack.push((local.clone(), val));
            let res = eval_expr(body, funcs, stack)?;
            stack.pop();
            res
        }
        Expr::Then(a, b) => {
            eval_expr(a, funcs, stack)?;
            eval_expr(b, funcs, stack)?
        }
        Expr::Binary(a, BinaryOp::Add, b) => {
            if let (Ok(lhs), Ok(rhs)) = (
                eval_expr(a, funcs, stack)?.int(a.1.clone()),
                eval_expr(b, funcs, stack)?.int(b.1.clone()),
            ) {
                Value::Int(lhs + rhs)
            } else {
                Value::Float(eval_expr(a, funcs, stack)?.float(a.1.clone())? + eval_expr(b, funcs, stack)?.float(b.1.clone())?)
            }
        }
        Expr::Binary(a, BinaryOp::Sub, b) => {
            if let (Ok(lhs), Ok(rhs)) = (
                eval_expr(a, funcs, stack)?.int(a.1.clone()),
                eval_expr(b, funcs, stack)?.int(b.1.clone()),
            ) {
                Value::Int(lhs - rhs)
            } else {
                Value::Float(eval_expr(a, funcs, stack)?.float(a.1.clone())? - eval_expr(b, funcs, stack)?.float(b.1.clone())?)
            }
        }
        Expr::Binary(a, BinaryOp::Mul, b) => {
            if let (Ok(lhs), Ok(rhs)) = (
                eval_expr(a, funcs, stack)?.int(a.1.clone()),
                eval_expr(b, funcs, stack)?.int(b.1.clone()),
            ) {
                Value::Int(lhs * rhs)
            } else {
                Value::Float(eval_expr(a, funcs, stack)?.float(a.1.clone())? * eval_expr(b, funcs, stack)?.float(b.1.clone())?)
            }
        }
        Expr::Binary(a, BinaryOp::Div, b) => {
            if let (Ok(lhs), Ok(rhs)) = (
                eval_expr(a, funcs, stack)?.int(a.1.clone()),
                eval_expr(b, funcs, stack)?.int(b.1.clone()),
            ) {
                Value::Int(lhs / rhs)
            } else {
                Value::Float(eval_expr(a, funcs, stack)?.float(a.1.clone())? / eval_expr(b, funcs, stack)?.float(b.1.clone())?)
            }
        }
        Expr::Binary(a, BinaryOp::Eq, b) => Value::Boolean(eval_expr(a, funcs, stack)? == eval_expr(b, funcs, stack)?),
        Expr::Binary(a, BinaryOp::NotEq, b) => Value::Boolean(eval_expr(a, funcs, stack)? != eval_expr(b, funcs, stack)?),
        Expr::Call(func, (args, args_span)) => {
            let f = eval_expr(func, funcs, stack)?;
            match f {
                Value::Func(name) => {
                    let f = &funcs[&name];
                    let mut stack = if f.args.len() != args.len() {
                        return Err(Error {
                            span: args_span.clone(),
                            msg: format!("'{}' called with wrong number of arguments (expected {}, found {})", name, f.args.len(), args.len()),
                        });
                    } else {
                        f.args
                            .iter()
                            .zip(args.iter())
                            .map(|(name, arg)| Ok((name.clone(), eval_expr(arg, funcs, stack)?)))
                            .collect::<Result<_, _>>()?
                    };
                    eval_expr(&f.body, funcs, &mut stack)?
                }
                f => {
                    return Err(Error {
                        span: func.1.clone(),
                        msg: format!("'{:?}' is not callable", f),
                    })
                }
            }
        }
        Expr::If(cond, a, b) => {
            let c = eval_expr(cond, funcs, stack)?;
            match c {
                Value::Boolean(true) => eval_expr(a, funcs, stack)?,
                Value::Boolean(false) => eval_expr(b, funcs, stack)?,
                c => {
                    return Err(Error {
                        span: cond.1.clone(),
                        msg: format!("Conditions must be booleans, found '{:?}'", c),
                    })
                }
            }
        }
        Expr::Print(a) => {
            let val = eval_expr(a, funcs, stack)?;
            print!("{}", val);
            val
        }
        Expr::PrintLn(a) => {
            let val = eval_expr(a, funcs, stack)?;
            println!("{}", val);
            val
        }
    })
}
