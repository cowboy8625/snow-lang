use super::token::{/* BlockType, */ KeyWord, Span, Spanned, Token};
// use std::{/* collections::HashMap, */ env, fmt, fs};

#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    Boolean(bool),
    Int(i128),
    Float(f64),
    String(String),
    List(Vec<Value>),
    Func(String),
    // Data(),
    // Records(),
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

impl BinaryOp {
    fn new(token: &Token) -> Self {
        if token == &Token::Op("+".into()) {
            return Self::Add;
        } else if token == &Token::Op("-".into()) {
            return Self::Sub;
        } else if token == &Token::Op("*".into()) {
            return Self::Mul;
        } else if token == &Token::Op("/".into()) {
            return Self::Div;
        } else if token == &Token::Op("==".into()) {
            return Self::Eq;
        } else if token == &Token::Op("!=".into()) {
            return Self::NotEq;
        }
        unreachable!()
    }
}

#[derive(Clone, Debug)]
pub enum UnaryOp {
    Not,
    Neg,
}

impl UnaryOp {
    fn new(token: &Token) -> Self {
        if token == &Token::KeyWord(KeyWord::Not) || token == &Token::Op("!".into()) {
            return Self::Not;
        } else if token == &Token::Op("-".into()) {
            return Self::Neg;
        }
        unreachable!()
    }
}

// An expression node in the AST. Children are spanned so we can generate useful runtime errors.
#[derive(Debug)]
pub enum Expr {
    Error,
    Value(Value),
    List(Vec<Spanned<Self>>),
    Local(String),
    Let(String, Box<Spanned<Self>>, Box<Spanned<Self>>),
    Binary(Box<Spanned<Self>>, Spanned<BinaryOp>, Box<Spanned<Self>>),
    Grouping(Box<Spanned<Self>>),
    Unary(Spanned<UnaryOp>, Box<Spanned<Self>>),
    App(Box<Spanned<Self>>, Spanned<Vec<Spanned<Self>>>),
    If(Box<Spanned<Self>>, Box<Spanned<Self>>, Box<Spanned<Self>>),
    Then(Box<Spanned<Self>>, Box<Spanned<Self>>),
    Print(Box<Spanned<Self>>),
}

// A function node in the AST.
#[derive(Debug)]
pub struct Func {
    args: Vec<String>,
    body: Spanned<Expr>,
}

#[derive(Debug)]
pub struct Error {
    span: Span,
    msg: String,
}

// expression
// equality
// comparison
// term
// factor
// unary
// primary

pub struct Parser<'a> {
    tokens: &'a [Spanned<Token>],
    current: usize,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a [Spanned<Token>]) -> Self {
        Self { tokens, current: 0 }
    }

    fn match_(&mut self, tokens: &[Token]) -> bool {
        for token in tokens.iter() {
            if self.check(token) {
                self.advance();
                return true;
            }
        }

        false
    }

    fn check(&self, token: &Token) -> bool {
        if self.is_at_end() {
            return false;
        }
        &self.peek().0 == token
    }

    fn advance(&mut self) -> Spanned<Token> {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous().clone()
    }

    fn is_at_end(&self) -> bool {
        self.peek().0 == Token::Eof
    }

    fn peek(&self) -> &Spanned<Token> {
        &self.tokens[self.current]
    }

    fn previous(&self) -> &Spanned<Token> {
        &self.tokens[self.current - 1]
    }

    fn consume(&mut self, token: Token, message: &str) -> Spanned<Token> {
        if self.check(&token) {
            return self.advance();
        }

        let spanned = self.peek();
        let token = &spanned.0;
        let span = &spanned.1;
        panic!("{:?}:{}: {}", span, token, message);
    }

    pub fn parse(&mut self) -> Spanned<Expr> {
        self.expression()
    }

    fn expression(&mut self) -> Spanned<Expr> {
        self.equality()
    }

    fn equality(&mut self) -> Spanned<Expr> {
        let (mut expr_tok, mut expr_span) = self.comparison();

        while self.match_(&[Token::Op("!=".into()), Token::Op("==".into())]) {
            let (token, span) = self.previous();
            let operator = (BinaryOp::new(token), span.clone());
            let (right_tok, rspan) = self.comparison();
            expr_tok = Expr::Binary(
                Box::new((expr_tok, expr_span.clone())),
                operator,
                Box::new((right_tok, rspan.clone())),
            );
            expr_span = expr_span.start..rspan.end;
        }

        (expr_tok, expr_span)
    }

    fn comparison(&mut self) -> Spanned<Expr> {
        use Token::Op;
        let (mut expr_tok, mut expr_span) = self.term();

        while self.match_(&[
            Op(">".into()),
            Op(">=".into()),
            Op("<".into()),
            Op(">=".into()),
        ]) {
            let (token, span) = self.previous();
            let operator = (BinaryOp::new(token), span.clone());
            let (right_tok, rspan) = self.term();
            expr_tok = Expr::Binary(
                Box::new((expr_tok, expr_span.clone())),
                operator,
                Box::new((right_tok, rspan.clone())),
            );
            expr_span = expr_span.start..rspan.end;
        }

        (expr_tok, expr_span)
    }

    fn term(&mut self) -> Spanned<Expr> {
        use Token::Op;
        let (mut expr_tok, mut expr_span) = self.factor();

        while self.match_(&[Op("-".into()), Op("+".into())]) {
            let (token, span) = self.previous();
            let operator = (BinaryOp::new(token), span.clone());
            let (right_tok, rspan) = self.factor();
            expr_tok = Expr::Binary(
                Box::new((expr_tok, expr_span.clone())),
                operator,
                Box::new((right_tok, rspan.clone())),
            );
            expr_span = expr_span.start..rspan.end;
        }

        (expr_tok, expr_span)
    }

    fn factor(&mut self) -> Spanned<Expr> {
        use Token::Op;
        let (mut expr_tok, mut expr_span) = self.unary();

        while self.match_(&[Op("/".into()), Op("*".into())]) {
            let (token, span) = self.previous();
            let operator = (BinaryOp::new(token), span.clone());
            let (right_tok, rspan) = self.unary();
            expr_tok = Expr::Binary(
                Box::new((expr_tok, expr_span.clone())),
                operator,
                Box::new((right_tok, rspan.clone())),
            );
            expr_span = expr_span.start..rspan.end;
        }

        (expr_tok, expr_span)
    }

    fn unary(&mut self) -> Spanned<Expr> {
        use Token::Op;
        if self.match_(&[Op("!".into()), Op("-".into())]) {
            let (token, span) = self.previous();
            let span = span.clone();
            let operator = (UnaryOp::new(token), span.clone());
            let (right_tok, rspan) = self.unary();
            return (
                Expr::Unary(operator, Box::new((right_tok, rspan.clone()))),
                span.start..rspan.end,
            );
        }

        self.primary()
    }

    fn primary(&mut self) -> Spanned<Expr> {
        let next = self.peek().clone();
        match next {
            (Token::KeyWord(KeyWord::False), span) => {
                self.advance();
                return (Expr::Value(Value::Boolean(false)), span.clone());
            }
            (Token::KeyWord(KeyWord::True), span) => {
                self.advance();
                return (Expr::Value(Value::Boolean(true)), span.clone());
            }
            (Token::String(s), span) => {
                self.advance();
                return (Expr::Value(Value::String(s.into())), span.clone());
            }
            (Token::Int(i), span) => {
                self.advance();
                return (
                    Expr::Value(Value::Int(i.parse::<i128>().unwrap())),
                    span.clone(),
                );
            }
            (Token::Float(f), span) => {
                self.advance();
                return (
                    Expr::Value(Value::Float(f.parse::<f64>().unwrap())),
                    span.clone(),
                );
            }
            (Token::Ctrl('('), span) => {
                // self.advance();
                let expr = self.expression();
                self.consume(Token::Ctrl(')'), "Expect ')' after expression.");
                return (Expr::Grouping(Box::new(expr)), span.clone());
            }
            // (Token::Eof, span) => (Expr::Error, span.clone()),
            _ => panic!("Primary"),
        }
    }
}

pub fn eval_expr(expr: &Spanned<Expr>) -> Result<Value, Error> {
    Ok(match &expr.0 {
        Expr::Error => unreachable!(), // Error expressions only get created by parser errors, so cannot exist in a valid AST
        Expr::Value(val) => val.clone(),
        Expr::Binary(a, (BinaryOp::Add, _), b) => {
            if let (Ok(lhs), Ok(rhs)) = (
                eval_expr(a)?.int(a.1.clone()),
                eval_expr(b)?.int(b.1.clone()),
            ) {
                Value::Int(lhs + rhs)
            } else {
                Value::Float(eval_expr(a)?.float(a.1.clone())? + eval_expr(b)?.float(b.1.clone())?)
            }
        }
        Expr::Binary(a, (BinaryOp::Sub, _), b) => {
            if let (Ok(lhs), Ok(rhs)) = (
                eval_expr(a)?.int(a.1.clone()),
                eval_expr(b)?.int(b.1.clone()),
            ) {
                Value::Int(lhs - rhs)
            } else {
                Value::Float(eval_expr(a)?.float(a.1.clone())? - eval_expr(b)?.float(b.1.clone())?)
            }
        }
        Expr::Binary(a, (BinaryOp::Mul, _), b) => {
            if let (Ok(lhs), Ok(rhs)) = (
                eval_expr(a)?.int(a.1.clone()),
                eval_expr(b)?.int(b.1.clone()),
            ) {
                Value::Int(lhs * rhs)
            } else {
                Value::Float(eval_expr(a)?.float(a.1.clone())? * eval_expr(b)?.float(b.1.clone())?)
            }
        }
        Expr::Binary(a, (BinaryOp::Div, _), b) => {
            if let (Ok(lhs), Ok(rhs)) = (
                eval_expr(a)?.int(a.1.clone()),
                eval_expr(b)?.int(b.1.clone()),
            ) {
                Value::Int(lhs / rhs)
            } else {
                Value::Float(eval_expr(a)?.float(a.1.clone())? / eval_expr(b)?.float(b.1.clone())?)
            }
        }
        Expr::Binary(a, (BinaryOp::Eq, _), b) => Value::Boolean(eval_expr(a)? == eval_expr(b)?),
        Expr::Binary(a, (BinaryOp::NotEq, _), b) => Value::Boolean(eval_expr(a)? != eval_expr(b)?),
        _ => unreachable!(),
    })
}

// fn eval_expr(
//     expr: &Spanned<Expr>,
//     funcs: &HashMap<String, Func>,
//     stack: &mut Vec<(String, Value)>,
// ) -> Result<Value, Error> {
//     Ok(match &expr.0 {
//         Expr::Error => unreachable!(), // Error expressions only get created by parser errors, so cannot exist in a valid AST
//         Expr::Value(val) => val.clone(),
//         Expr::List(items) => Value::List(
//             items
//                 .iter()
//                 .map(|item| eval_expr(item, funcs, stack))
//                 .collect::<Result<_, _>>()?,
//         ),
//         Expr::Local(name) => stack
//             .iter()
//             .rev()
//             .find(|(l, _)| l == name)
//             .map(|(_, v)| v.clone())
//             .or_else(|| Some(Value::Func(name.clone())).filter(|_| funcs.contains_key(name)))
//             .ok_or_else(|| Error {
//                 span: expr.1.clone(),
//                 msg: format!("No such variable '{}' in scope", name),
//             })?,
//         Expr::Let(local, val, body) => {
//             let val = eval_expr(val, funcs, stack)?;
//             stack.push((local.clone(), val));
//             let res = eval_expr(body, funcs, stack)?;
//             stack.pop();
//             res
//         }
//         Expr::Then(a, b) => {
//             eval_expr(a, funcs, stack)?;
//             eval_expr(b, funcs, stack)?
//         }
//         Expr::Binary(a, BinaryOp::Add, b) => Value::Int(
//             eval_expr(a, funcs, stack)?.num(a.1.clone())?
//                 + eval_expr(b, funcs, stack)?.num(b.1.clone())?,
//         ),
//         Expr::Binary(a, BinaryOp::Sub, b) => Value::Int(
//             eval_expr(a, funcs, stack)?.num(a.1.clone())?
//                 - eval_expr(b, funcs, stack)?.num(b.1.clone())?,
//         ),
//         Expr::Binary(a, BinaryOp::Mul, b) => Value::Int(
//             eval_expr(a, funcs, stack)?.num(a.1.clone())?
//                 * eval_expr(b, funcs, stack)?.num(b.1.clone())?,
//         ),
//         Expr::Binary(a, BinaryOp::Div, b) => Value::Int(
//             eval_expr(a, funcs, stack)?.num(a.1.clone())?
//                 / eval_expr(b, funcs, stack)?.num(b.1.clone())?,
//         ),
//         Expr::Binary(a, BinaryOp::Eq, b) => {
//             Value::Boolean(eval_expr(a, funcs, stack)? == eval_expr(b, funcs, stack)?)
//         }
//         Expr::Binary(a, BinaryOp::NotEq, b) => {
//             Value::Boolean(eval_expr(a, funcs, stack)? != eval_expr(b, funcs, stack)?)
//         }
//         Expr::Call(func, (args, args_span)) => {
//             let f = eval_expr(func, funcs, stack)?;
//             match f {
//                 Value::Func(name) => {
//                     let f = &funcs[&name];
//                     let mut stack = if f.args.len() != args.len() {
//                         return Err(Error {
//                             span: args_span.clone(),
//                             msg: format!("'{}' called with wrong number of arguments (expected {}, found {})", name, f.args.len(), args.len()),
//                         });
//                     } else {
//                         f.args
//                             .iter()
//                             .zip(args.iter())
//                             .map(|(name, arg)| Ok((name.clone(), eval_expr(arg, funcs, stack)?)))
//                             .collect::<Result<_, _>>()?
//                     };
//                     eval_expr(&f.body, funcs, &mut stack)?
//                 }
//                 f => {
//                     return Err(Error {
//                         span: func.1.clone(),
//                         msg: format!("'{:?}' is not callable", f),
//                     })
//                 }
//             }
//         }
//         Expr::If(cond, a, b) => {
//             let c = eval_expr(cond, funcs, stack)?;
//             match c {
//                 Value::Boolean(true) => eval_expr(a, funcs, stack)?,
//                 Value::Boolean(false) => eval_expr(b, funcs, stack)?,
//                 c => {
//                     return Err(Error {
//                         span: cond.1.clone(),
//                         msg: format!("Conditions must be booleans, found '{:?}'", c),
//                     })
//                 }
//             }
//         }
//         Expr::Print(a) => {
//             let val = eval_expr(a, funcs, stack)?;
//             println!("{}", val);
//             val
//         }
//     })
// }
