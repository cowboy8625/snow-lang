use super::{
    error::Error,
    expr::{Atom, Expr},
    op::Op,
    precedence::Precedence,
    Scanner, Span, Token,
};

use std::iter::Peekable;

pub struct Parser<'a> {
    pub(crate) lexer: Peekable<Scanner<'a>>,
    token_stream: Vec<Token>,
    errors: Vec<Error>,
}
impl<'a> Parser<'a> {
    pub fn new(lexer: Peekable<Scanner<'a>>) -> Self {
        Self {
            lexer,
            token_stream: vec![],
            errors: Vec::new(),
        }
    }

    fn next_if<F: FnOnce(Token) -> bool>(&mut self, func: F) -> Option<Token> {
        if func(self.peek()) {
            return Some(self.next());
        }
        None
    }

    fn next(&mut self) -> Token {
        let token = self
            .lexer
            .next()
            .map(|t| match t {
                Token::Error(ch, span) => {
                    self.report(Error::InvalidChar(
                        ch.chars().last().unwrap_or('\0'),
                        span,
                    ));
                    Token::Eof(span)
                }
                _ => t,
            })
            .unwrap_or(Token::Eof(Span::default()));
        self.token_stream.push(token.clone());
        token
    }

    fn peek(&mut self) -> Token {
        self.lexer
            .peek()
            .cloned()
            .unwrap_or(Token::Eof(Span::default()))
    }

    // fn previous(&self) -> Option<&Token> {
    //     self.token_stream.last()
    // }

    fn is_end(&mut self) -> bool {
        matches!(self.peek(), Token::Eof(..))
    }

    fn remove_last_error(&mut self) {
        self.errors.pop();
    }

    fn report(&mut self, error: Error) -> Expr {
        let span = error.span();
        self.errors.push(error);
        Expr::Error(span)
    }

    fn recover(&mut self) {
        loop {
            let token = self.next();
            if token.is_op_a(";") || token.is_eof() {
                break;
            }
        }
    }

    // FIXME: This is broken
    // fn recover(&mut self, deliminators: &[Token]) {
    //     if let Some(error) = &self.errors {
    //         if error.get_error_code::<ErrorCode>() == ErrorCode::E0010 {
    //             return;
    //         }
    //     }
    //     let mut last_span = self
    //         .previous()
    //         .map(|t| t.span())
    //         .unwrap_or_else(Span::default);
    //     println!("recovering");
    //     while let Some(tok) = self.next_if(|t| !deliminators.contains(&t)) {
    //         if tok.span().line > last_span.line {
    //             break;
    //         }
    //         last_span = tok.span();
    //         if self.is_end() {
    //             break;
    //         }
    //     }
    //     self.next();
    //     dbg!(self.peek());
    // }

    pub fn parse(mut self) -> Result<Vec<Expr>, Vec<Error>> {
        let mut ast = vec![];
        while !self.is_end() {
            let e = self.declaration();
            ast.push(e);
        }
        if !self.errors.is_empty() || ast.iter().any(|x| x.is_error()) {
            return Err(self.errors);
        }
        Ok(ast)
    }

    fn is_function(&mut self, token: &Token) -> bool {
        (token.is_id() && self.peek().is_op_a("="))
            || (token.is_id() && self.peek().is_id())
    }

    fn declaration(&mut self) -> Expr {
        let token = self.next();
        if self.is_function(&token) {
            let expr = self.function(&token);
            if expr.is_error() {
                // self.recover(&[Token::Op(";".into(), token.span())]);
                self.recover();
                return self.declaration();
            }
            return expr;
        } else if token.is_keyword_a("enum") {
            let expr = self.enum_def(token.span());
            if expr.is_error() {
                // self.recover(&[Token::Op(";".into(), token.span())]);
                self.recover();
                return self.declaration();
            }
            return expr;
        } else if token.is_id() && self.peek().is_op_a("::") {
            // } else if let Token::Id(id, span) = token {
            let expr = self.type_dec(&token);
            if expr.is_error() {
                // self.recover(&[Token::Op(";".into(), token.span())]);
                self.recover();
                return self.declaration();
            }
            return expr;
        }
        self.recover();
        self.report(Error::ItemNotAllowedInGlobalSpace(token.span()))
    }

    fn type_dec(&mut self, token: &Token) -> Expr {
        let name = token.value();
        let start = token.span();
        if !self.next().is_op_a("::") {
            return Expr::Error(token.span());
        }
        // FIXME:[1](cowboy) types need to currently are only string.
        //          Moving to a Ident { String, Span } could be
        //          nice for error messages.
        let mut types = vec![];
        let mut last_type_span = start;
        while let Some(ref tok) = self.next_if(|t| !t.is_op_a(";")) {
            let type_name = match tok {
                Token::Id(id, ..) => id.to_string(),
                _ => {
                    // FIXME: not pointing to right location
                    let idx = self.token_stream.len().saturating_sub(3);
                    let last_token = &self.token_stream[idx];
                    let span = last_token.span();
                    return self.report(Error::MissingDeliminator(span));
                }
            };
            types.push(type_name);
            if !self.peek().is_op_a("->") {
                break;
            }
            self.next();
            last_type_span = tok.span();
        }

        if !self.peek().is_op_a(";") {
            // FIXME: This probably does not point to correct span
            let span_start = self.peek().span();
            let span = Span::from((span_start, last_type_span));
            return self.report(Error::MissingDeliminator(span));
        }
        let span_end = self.next().span();
        let span = Span::from((start, span_end));
        Expr::TypeDec(name.into(), types, span)
    }

    fn enum_def(&mut self, start: Span) -> Expr {
        let Token::Id(name, ..) = self.next() else {
            return Expr::Error(start);
        };

        let mut variants = vec![];
        if self.next_if(|t| t.is_op_a("=")).is_some() {
            while let Token::Id(name, ..) = self.next() {
                let mut type_list = vec![];
                while let Some(Token::Id(type_id, ..)) = self.next_if(|t| t.is_id()) {
                    type_list.push(type_id);
                }
                variants.push((name, type_list));
                if self.next_if(|t| t.is_op_a("|")).is_none() {
                    break;
                }
            }
        }
        if !self.peek().is_op_a(";") {
            // let span = self.peek().span();
            let end = self.peek().span();
            let span = Span::from((start, end));
            return self.report(Error::MissingDeliminator(span));
        }
        let end = self.next().span();
        let span = Span::from((start, end));
        Expr::Enum(name, variants, span)
    }

    pub(crate) fn function(&mut self, token: &Token) -> Expr {
        let start = token.span();
        let name = token.value();
        let body = self
            .expression(Precedence::Fn)
            .and_then(|lhs| {
                let mut args = vec![lhs];
                while self.peek().is_id() {
                    args.push(self.expression(Precedence::Fn));
                }
                if self.next_if(|t| t.is_op_a("=")).is_none() {
                    let span = self.peek().span();
                    return self.report(Error::Unknown(span));
                }
                let body = self.closure();
                args.into_iter().rev().fold(body, |last, next| {
                    let span = Span::from((last.span(), next.span()));
                    Expr::Closure(Box::new(next), Box::new(last), span)
                })
            })
            .or_else(|_| {
                self.remove_last_error();
                if self.next_if(|t| t.is_op_a("=")).is_none() {
                    let span = self.peek().span();
                    return self.report(Error::Unknown(span));
                };
                self.closure()
            });
        if let Some(Error::MissingDeliminator(_span)) = self.errors.last() {
            return Expr::Error(Span::default());
        }
        if !self.peek().is_op_a(";") {
            let span = self.peek().span();
            return self.report(Error::MissingDeliminator(span));
        }
        let end = self.next().span();
        let span = Span::from((start, end));
        Expr::Func(name.into(), Box::new(body), span)
    }

    pub(crate) fn closure(&mut self) -> Expr {
        // FIXME: I think the problem is in closure function due to its being called in
        // function_def and closuer test are failing too.
        if self
            .next_if(|t| t.is_op_a("λ") || t.is_op_a("\\"))
            .is_some()
        {
            let head = Box::new(self.expression(Precedence::Fn));
            let tail = Box::new(
                self.closure()
                    .and_then(|mut lhs| {
                        while let Token::Id(..) = self.peek() {
                            let tail = Box::new(self.expression(Precedence::Fn));
                            let start = lhs.span();
                            let end = tail.span();
                            let span = Span::from((start, end));
                            lhs = Expr::Closure(Box::new(lhs), tail, span);
                        }
                        if self.next_if(|t| t.is_op_a("->")).is_none() {
                            let span = self.peek().span();
                            return self.report(Error::Unknown(span));
                        };
                        let body = self.closure();
                        let start = lhs.span();
                        let end = body.span();
                        let span = Span::from((start, end));
                        Expr::Closure(Box::new(lhs), Box::new(body), span)
                    })
                    .or_else(|_| {
                        self.remove_last_error();
                        if self.next_if(|t| t.is_op_a("->")).is_none() {
                            let span = self.peek().span();
                            return self.report(Error::Unknown(span));
                        };
                        self.closure()
                    }),
            );
            let start = head.span();
            let end = tail.span();
            let span = Span::from((start, end));
            return Expr::Closure(head, tail, span);
        }
        self.conditional()
    }

    pub(crate) fn conditional(&mut self) -> Expr {
        let start = self.peek().span();
        if self.next_if(|t| t.is_keyword_a("if")).is_some() {
            let condition = self.expression(Precedence::None);
            if condition.is_error() {
                self.remove_last_error();
                let end = condition.span();
                let span = Span::from((start, end));
                return self.report(Error::ExpectedConditionForStatement(span));
            }
            if self.next_if(|t| t.is_keyword_a("then")).is_none() {
                let span = self.peek().span();
                return self.report(Error::Unknown(span));
            }
            let branch1 = self.closure();
            if self.next_if(|t| t.is_keyword_a("else")).is_none() {
                let span = self.peek().span();
                return self.report(Error::Unknown(span));
            }
            let branch2 = self.closure();
            let end = branch2.span();
            let span = Span::from((start, end));
            return Expr::IfElse(
                Box::new(condition),
                Box::new(branch1),
                Box::new(branch2),
                span,
            );
        }
        self.call(Precedence::None)
    }

    pub(crate) fn call(&mut self, min_bp: Precedence) -> Expr {
        let mut lhs = self.expression(min_bp);
        if match self.peek() {
            Token::Op(ref op, ..) if op == "(" => true,
            Token::Op(ref op, ..) if op == "[" => true,
            Token::Op(..) | Token::KeyWord(..) | Token::Eof(..) => false,
            _ => true,
        } {
            let mut args = vec![];
            let last;
            loop {
                match self.peek() {
                    Token::Op(ref op, ..) if op == "(" || op == "[" => {}
                    Token::Op(.., span) | Token::Eof(.., span) => {
                        last = Some(span);
                        break;
                    }
                    _ => {}
                };
                args.push(self.expression(Precedence::None));
            }
            let start = lhs.span();
            let end = last.unwrap_or(start);
            let span = Span::from((start, end));
            lhs = Expr::App(Box::new(lhs), args, span);
        }
        lhs
    }

    fn prefix_op(&mut self, op: &str, span: Span) -> Expr {
        match op {
            "(" => {
                self.next();
                let lhs = self.closure();
                if self.next_if(|t| t.is_op_a(")")).is_none() {
                    let span = self.peek().span();
                    // return self.report("E13", "closing ')' missing", span);
                    return self.report(Error::Unknown(span));
                };
                lhs
            }
            "[" => {
                self.next();
                let mut lhs = vec![];
                while !self.peek().is_op_a("]") {
                    let expr = self.closure();
                    lhs.push(expr);
                    self.next_if(|t| t.is_op_a(","));
                }
                if self.next_if(|t| t.is_op_a("]")).is_none() {
                    let span = lhs.last().map(|t| t.span()).unwrap_or(self.peek().span());
                    return self.report(Error::Unknown(span));
                };
                let span = lhs
                    .first()
                    .and_then(|f| {
                        lhs.last().map_or(Some(f.span()), |l| {
                            Some(Span::from((f.span(), l.span())))
                        })
                    })
                    .unwrap_or_default();
                Expr::Array(lhs, span)
            }
            o @ ("-" | "!") => {
                self.next();
                let op = Op::try_from(o).unwrap();
                let lhs = self.expression(Precedence::Unary);
                let end = lhs.span();
                let span = Span::from((span, end));
                Expr::Unary(op, Box::new(lhs), span)
            }
            _ => {
                let mut l = self.lexer.clone();
                l.next();
                let is_closing_pran = l.peek().map(|t| t.is_op_a(")")).unwrap_or(false);
                if Op::try_from(op).is_ok() && is_closing_pran {
                    self.lexer = l;
                    let op = Op::try_from(op).unwrap();
                    Expr::Atom(Atom::Id(format!("({op})")), span)
                } else {
                    self.report(Error::UnknownOperator(span))
                }
            }
        }
    }

    pub(crate) fn expression(&mut self, min_bp: Precedence) -> Expr {
        let mut lhs = match self.peek() {
            Token::KeyWord(ref b, span) if b == "true" => {
                self.next();
                Expr::Atom(Atom::Bool(true), span)
            }
            Token::KeyWord(ref b, span) if b == "false" => {
                self.next();
                Expr::Atom(Atom::Bool(false), span)
            }
            Token::Int(int, span) => {
                self.next();
                Expr::Atom(Atom::Int(int.parse().unwrap()), span)
            }
            Token::Float(float, span) => {
                self.next();
                Expr::Atom(Atom::Float(float.parse().unwrap()), span)
            }
            Token::Id(ref id, span) => {
                self.next();
                Expr::Atom(Atom::Id(id.into()), span)
            }
            Token::String(ref string, span) => {
                self.next();
                Expr::Atom(Atom::String(string.into()), span)
            }
            Token::Char(ref c, span) => {
                self.next();
                if c.chars().count() > 1 {
                    // return self.report("E3", "invalid op char", span);
                    return self.report(Error::Unknown(span));
                }
                let Some(c) = c.chars().next() else {
                    // return self.report("E4", "invalid char definition", span);
                    return self.report(Error::Unknown(span));
                };
                Expr::Atom(Atom::Char(c), span)
            }
            Token::Op(ref op, span) => self.prefix_op(op, span),
            _ => {
                let span = self.peek().span();
                // return self.report("E5", "invalid token", span);
                return self.report(Error::Unknown(span));
            }
        };
        let start_span = lhs.span();
        loop {
            let token = self.peek();
            let cbp: Precedence = match token.clone() {
                Token::Op(..) => Precedence::try_from(token.clone()).unwrap(),
                Token::KeyWord(ref k, ..) if k == "and" => {
                    Precedence::try_from(token.clone()).unwrap()
                }
                Token::KeyWord(ref k, ..) if k == "or" => {
                    Precedence::try_from(token.clone()).unwrap()
                }
                Token::KeyWord(ref k, ..) if k == "mod" => {
                    Precedence::try_from(token.clone()).unwrap()
                }
                _ => break,
            };
            if cbp <= min_bp {
                break;
            }
            let span = Span::from((start_span, token.span()));
            match cbp {
                Precedence::Term
                | Precedence::Factor
                | Precedence::Comparison
                | Precedence::Equality
                | Precedence::And
                | Precedence::Or => {
                    let _ = self.next();
                    let rhs = self.expression(cbp);
                    lhs = Expr::Binary(
                        Op::try_from(&token).unwrap(),
                        Box::new(lhs),
                        Box::new(rhs),
                        span,
                    );
                }
                Precedence::Assignment | Precedence::None => break,
                Precedence::RLPipe => {
                    let _ = self.next();
                    let rhs = self.closure();
                    eprintln!("{rhs:?} {lhs:?}");
                    if let Expr::App(_, args, _) = &mut lhs {
                        args.push(rhs)
                    } else {
                        let l = Box::new(lhs);
                        lhs = Expr::App(l, vec![rhs], span);
                    }
                }
                Precedence::LRPipe => {
                    let _ = self.next();
                    let mut rhs = self.closure();
                    if let Expr::App(_, args, _) = &mut rhs {
                        args.push(lhs);
                        lhs = rhs;
                    } else {
                        let r = Box::new(rhs);
                        lhs = Expr::App(r, vec![lhs], span);
                    }
                }
                _ => {
                    // lhs = self.report("E5", "invalid token", span);
                    return self.report(Error::Unknown(span));
                }
            }
        }
        lhs
    }
}
