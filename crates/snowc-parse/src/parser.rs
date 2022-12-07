use super::{
    bail,
    expr::{Atom, Expr},
    op::Op,
    precedence::Precedence,
    CResult, Scanner, Span, Token,
};
use std::iter::Peekable;
pub fn parse(input: &str, bs: bool) -> CResult<Vec<Expr>> {
    let lexer = Scanner::new(input).peekable();
    let mut parser = Parser::new(lexer);
    parser.parse(bs)
}

pub(crate) struct Parser<'a> {
    pub(crate) lexer: Peekable<Scanner<'a>>,
}
impl<'a> Parser<'a> {
    pub(crate) fn new(lexer: Peekable<Scanner<'a>>) -> Self {
        Self { lexer }
    }

    fn peek(&mut self) -> (Token, Span) {
        self.lexer.peek().cloned().unwrap_or((Token::Eof, 0..0))
    }

    fn is_end(&mut self) -> bool {
        self.lexer.peek().is_none()
    }

    fn advance(&mut self, advance_if: Option<Token>) -> CResult<(Token, Span)> {
        let (token, span) = self.peek();
        match advance_if {
            Some(maybe_token) if maybe_token == token => {
                self.lexer.next();
                Ok((token, span))
            }
            Some(maybe_token) => bail!(
                span,
                "fail to advance do to not matching next expected token {:?} {:?}",
                maybe_token,
                token
            ),
            None => {
                self.lexer.next();
                Ok((token, span))
            }
        }
    }

    fn consume(&mut self, expected: Token, msg: &str) -> CResult<(Token, Span)> {
        let (token, span) = self.peek();
        if token != expected {
            bail!(
                span,
                "{msg}\r\nexpected '{:?}' but found '{:?}'",
                expected,
                token
            );
        }
        self.lexer.next();
        Ok((token, span))
    }

    pub(crate) fn parse(&mut self, bs: bool) -> CResult<Vec<Expr>> {
        let mut result = vec![];
        while !self.is_end() {
            let e = self.declaration(bs)?;
            result.push(e);
        }
        Ok(result)
    }

    fn declaration(&mut self, bs: bool) -> CResult<Expr> {
        match self.peek() {
            (Token::KeyWord(ref k), span) if k == "fn" => {
                self.lexer.next();
                self.function(span)
            }
            (Token::KeyWord(ref k), span) if k == "type" => {
                self.lexer.next();
                self.user_type_def(span)
            }
            (Token::Id(_), _) => {
                let mut l = self.lexer.clone();
                l.next();
                if matches!(l.next(), Some((Token::Op(ref op), _)) if op == "::") {
                    let Some((Token::Id(ref id), span)) = self.lexer.next() else {
                        panic!("not to sure what went wrong at this point");
                    };
                    self.consume(Token::Op("::".into()), "")?;
                    self.type_dec(id, span)
                } else {
                    let expr = self.call(Precedence::None);
                    if expr.is_ok() {
                        self.consume(
                            Token::Op(";".into()),
                            "expressions need to be terminated with ';'",
                        )?;
                    }
                    expr
                }
            }
            (_, _) if bs => {
                let expr = self.closure();
                if expr.is_ok() {
                    self.consume(
                        Token::Op(";".into()),
                        "expressions need to be terminated with ';'",
                    )?;
                }
                expr
            }
            (t, span) => {
                bail!(span, "'{:?}' are not allowed in global scope", t)
            }
        }
    }

    fn type_dec(&mut self, name: &str, _span: Span) -> CResult<Expr> {
        let mut types = vec![];
        while let Some((t, _)) = self
            .lexer
            .next_if(|(t, _)| !matches!(t, Token::Op(ref op) if op == ";"))
        {
            let type_name = match t {
                Token::Id(id) => id.to_string(),
                t => unreachable!("found: {t:?}"),
            };
            match self.peek().0 {
                Token::Op(ref op) if op == "->" => {
                    self.lexer.next();
                }
                _ => {}
            }
            types.push(type_name);
        }

        let (_, _span) =
            self.consume(Token::Op(";".into()), "type declaration's end with a ';'")?;
        Ok(Expr::TypeDec(name.into(), types))
    }

    fn user_type_def(&mut self, span: Span) -> CResult<Expr> {
        let Some((Token::Id(name), _name_span)) = self.lexer.next() else {
            bail!(
                span,
                "expected '{:?}' but found 'NONE'",
                Token::Id("<name>".into()));
        };

        let mut variants = vec![];
        match self.consume(Token::Op("=".into()), "expected '=' but found") {
            Ok(_) => {
                while let Some((Token::Id(name), _)) = self.lexer.next() {
                    let mut type_list = vec![];
                    while let Some((Token::Id(type_id), _)) =
                        self.lexer.next_if(|(t, _)| matches!(t, Token::Id(_)))
                    {
                        type_list.push(type_id);
                    }
                    variants.push((name, type_list));
                    if self.advance(Some(Token::Op("|".into()))).is_err() {
                        break;
                    }
                }
            }
            Err(_) => {}
        }
        self.consume(Token::Op(";".into()), "type's end with a ';'")?;
        Ok(Expr::Type(name.into(), variants))
    }

    pub(crate) fn function(&mut self, span: Span) -> CResult<Expr> {
        let Some((Token::Id(name), _span)) = self.lexer.next() else {
            bail!(span, "expected a identifier <name>");
        };
        Ok(Expr::Func(
            name,
            Box::new(
                self.expression(Precedence::Fn)
                    .and_then(|lhs| {
                        let mut args = vec![lhs];
                        while let Some((Token::Id(_), _)) = self.lexer.peek().cloned() {
                            args.push(self.expression(Precedence::Fn)?);
                        }
                        self.consume(
                            Token::Op("=".into()),
                            "After args '=' then function body",
                        )?;
                        let body = self.closure()?;
                        let f = args.into_iter().rev().fold(body, |last, next| {
                            Expr::Closure(Box::new(next), Box::new(last))
                        });
                        self.consume(Token::Op(";".into()), "functions end with a ';'")?;
                        Ok(f)
                    })
                    .or_else(|_| {
                        self.consume(
                            Token::Op("=".into()),
                            "After args '=' then function body",
                        )?;
                        let lhs = self.closure()?;
                        self.consume(Token::Op(";".into()), "functions end with a ';'")?;
                        Ok::<Expr, Box<dyn std::error::Error>>(lhs)
                    })?,
            ),
        ))
    }

    pub(crate) fn closure(&mut self) -> CResult<Expr> {
        if matches!(self.peek(), (Token::Op(ref op), _) if op == "λ" || op == "\\") {
            self.lexer.next();
            return Ok(Expr::Closure(
                Box::new(self.expression(Precedence::Fn)?),
                Box::new(
                    self.closure()
                        .and_then(|mut lhs| {
                            while let Some((Token::Id(_), _)) = self.lexer.peek().cloned()
                            {
                                lhs = Expr::Closure(
                                    Box::new(lhs),
                                    Box::new(self.expression(Precedence::Fn)?),
                                );
                            }
                            self.consume(Token::Op("->".into()), "lambda expressions")?;
                            let body = self.closure()?;
                            Ok(Expr::Closure(Box::new(lhs), Box::new(body)))
                        })
                        .or_else(|_| {
                            self.consume(Token::Op("->".into()), "lambda expressions")?;
                            let lhs = self.closure()?;
                            Ok::<Expr, Box<dyn std::error::Error>>(lhs)
                        })?,
                ),
            ));
        }
        self.conditional()
    }

    pub(crate) fn conditional(&mut self) -> CResult<Expr> {
        if matches!(self.peek(), (Token::KeyWord(ref k), _) if k == "if") {
            let (_, _start_span) = self.consume(Token::KeyWord("if".into()), "WHAT")?;
            let condition = self.expression(Precedence::None)?;
            self.consume(
                Token::KeyWord("then".into()),
                "else keyword is required with an if expression",
            )?;
            let branch1 = self.closure()?;
            self.consume(
                Token::KeyWord("else".into()),
                "else keyword is required with an if expression",
            )?;
            let branch2 = self.closure()?;
            return Ok(Expr::IfElse(
                Box::new(condition),
                Box::new(branch1),
                Box::new(branch2),
            ));
        }
        self.call(Precedence::None)
    }

    pub(crate) fn call(&mut self, min_bp: Precedence) -> CResult<Expr> {
        let mut lhs = self.expression(min_bp)?;
        if match self.lexer.peek().cloned() {
            Some((Token::Op(ref op), _)) if op == "(" => true,
            Some((Token::Op(_), _)) => false,
            Some((Token::KeyWord(_), _)) => false,
            None => false,
            _ => true,
        } {
            let mut args = vec![];
            loop {
                match self.lexer.peek().map(|(t, _)| t.clone()) {
                    Some(Token::Op(ref op)) if op == "(" => {}
                    Some(Token::Op(_)) => break,
                    None => break,
                    _ => {}
                };
                args.push(self.expression(Precedence::None)?);
            }
            lhs = Expr::App(Box::new(lhs), args);
        }
        Ok(lhs)
    }

    fn prefix_op(&mut self, op: &str, span: Span) -> CResult<Expr> {
        match op {
            "(" => {
                self.lexer.next();
                let lhs = self.closure()?;
                self.consume(Token::Op(")".into()), "closing ')' missing")?;
                Ok(lhs)
            }
            o @ ("-" | "!") => {
                self.lexer.next();
                let op = Op::try_from(o)?;
                let lhs = self.expression(Precedence::Unary)?;
                Ok(Expr::Unary(op, Box::new(lhs)))
            }
            c => {
                let mut l = self.lexer.clone();
                l.next();
                let peek = l.peek().map(|(t, _)| t.clone()).unwrap_or(Token::Eof);
                if Op::try_from(op).is_ok() && matches!(peek, Token::Op(op) if op == ")")
                {
                    self.lexer = l;
                    let op = Op::try_from(op)?;
                    Ok(Expr::Atom(Atom::Id(format!("({op})"))))
                } else {
                    bail!(span, "unknown op char: {}", c)
                }
            }
        }
    }

    pub(crate) fn expression(&mut self, min_bp: Precedence) -> CResult<Expr> {
        let (token, start_span) = self.peek();
        let mut lhs = match token {
            Token::KeyWord(ref b) if b == "true" => {
                self.lexer.next();
                Expr::Atom(Atom::Bool(true))
            }
            Token::KeyWord(ref b) if b == "false" => {
                self.lexer.next();
                Expr::Atom(Atom::Bool(false))
            }
            Token::Int(int) => {
                self.lexer.next();
                Expr::Atom(Atom::Int(int.parse().unwrap()))
            }
            Token::Float(float) => {
                self.lexer.next();
                Expr::Atom(Atom::Float(float.parse().unwrap()))
            }
            Token::Id(ref id) => {
                self.lexer.next();
                Expr::Atom(Atom::Id(id.into()))
            }
            Token::String(ref string) => {
                self.lexer.next();
                Expr::Atom(Atom::String(string.into()))
            }
            Token::Char(ref c) => {
                self.lexer.next();
                if c.chars().count() > 1 {
                    bail!(start_span, "bad char '{c}'");
                }
                let Some(c) = c.chars().nth(0) else {
                    bail!(start_span, "char type can not be empty");
                };
                Expr::Atom(Atom::Char(c))
            }
            Token::Op(ref op) => self.prefix_op(op, start_span.clone())?,
            t => bail!(start_span, "bad token: {:?}", t),
        };
        loop {
            let (token, span) = self.peek();
            let cbp: Precedence = match token.clone() {
                Token::Op(_) => Precedence::try_from(token.clone())?,
                _ => break,
            };
            if cbp <= min_bp {
                break;
            }
            match cbp {
                Precedence::Term
                | Precedence::Factor
                | Precedence::Comparison
                | Precedence::Equality => {
                    let _ = self.lexer.next();
                    let rhs = self.expression(cbp)?;
                    lhs =
                        Expr::Binary(Op::try_from(&token)?, Box::new(lhs), Box::new(rhs));
                }
                Precedence::Assignment | Precedence::None => break,
                Precedence::Pipe => {
                    let _ = self.lexer.next();
                    let rhs = self.call(cbp)?;
                    lhs =
                        Expr::Binary(Op::try_from(&token)?, Box::new(lhs), Box::new(rhs));
                }
                _ => bail!(start_span.start..span.end, "cbp: {cbp:?}, token: {token}"),
            }
        }
        Ok(lhs)
    }
}
