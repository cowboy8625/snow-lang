use super::{
    expr::{Atom, Expr},
    op::Op,
    precedence::Precedence,
    Error, ParserDebug, Scanner, Span, Token,
};
use std::iter::Peekable;

pub struct Parser<'a> {
    pub(crate) lexer: Peekable<Scanner<'a>>,
    errors: Vec<Error>,
    debug_parser: ParserDebug,
}
impl<'a> Parser<'a> {
    pub fn new(lexer: Peekable<Scanner<'a>>) -> Self {
        let debug_parser = ParserDebug::Off;
        Self::new_with_debug(lexer, debug_parser)
    }

    pub fn new_with_debug(
        lexer: Peekable<Scanner<'a>>,
        debug_parser: ParserDebug,
    ) -> Self {
        Self {
            lexer,
            errors: vec![],
            debug_parser,
        }
    }

    fn next_if<F: FnOnce(Token) -> bool>(&mut self, func: F) -> Option<(Token, Span)> {
        let token = self.peek().0;
        if func(token) {
            return Some(self.next());
        }
        None
    }
    fn next(&mut self) -> (Token, Span) {
        let (t, s) = self.lexer.next().unwrap();
        (t, s)
    }
    fn peek(&mut self) -> (Token, Span) {
        self.lexer.peek().cloned().unwrap()
    }

    fn is_end(&mut self) -> bool {
        matches!(self.peek(), (Token::Eof, _))
    }

    fn consume(&mut self, expected: Token) -> Option<(Token, Span)> {
        if self.peek().0 != expected {
            return None;
        }
        Some(self.next())
    }

    fn report(&mut self, id: &str, label: &str, span: Span) -> Expr {
        let id = id.into();
        let label = label.into();
        let s = span.clone();
        self.errors.push(Error { id, label, span });
        Expr::Error(s)
    }

    fn recover(&mut self, deliminators: &[Token]) {
        while let Some(_) = self.next_if(|t| !deliminators.contains(&t)) {
            if self.is_end() {
                break;
            }
        }
        self.next();
    }

    pub fn parse(mut self) -> Result<Vec<Expr>, Vec<Error>> {
        let Self { debug_parser, .. } = self;
        let mut ast = vec![];
        while !self.is_end() {
            let e = self.declaration();
            ast.push(e);
        }
        if let ParserDebug::On = debug_parser {
            dbg!(&ast);
        }
        if !self.errors.is_empty() {
            return Err(self.errors);
        }
        Ok(ast)
    }

    fn declaration(&mut self) -> Expr {
        match self.next() {
            (Token::KeyWord(ref k), span) if k == "fn" => {
                let expr = self.function(span);
                if expr.is_error() {
                    self.recover(&[Token::Op(";".into())]);
                }
                expr
            }
            (Token::KeyWord(ref k), span) if k == "type" => {
                let expr = self.user_type_def(span);
                if expr.is_error() {
                    self.recover(&[Token::Op(";".into())]);
                }
                expr
            }
            (Token::Id(id), span) => {
                let expr = self.type_dec(&id, span);
                if expr.is_error() {
                    self.recover(&[Token::Op(";".into())]);
                }
                expr
            }
            (_, span) => {
                self.report("E0", "expressions not allowed in global scope", span)
            }
        }
    }

    fn type_dec(&mut self, name: &str, start: Span) -> Expr {
        let Some((_, _)) = self.consume(Token::Op("::".into())) else {
            return self.report("", "", start);
        };
        let mut types = vec![];
        while let Some((t, _)) = self
            .lexer
            .next_if(|(t, _)| !matches!(t, Token::Op(ref op) if op == ";"))
        {
            let type_name = match t {
                Token::Id(id) => id.to_string(),
                _ => {
                    let span = self.peek().1;
                    return self.report("E10", "type declaration's end with a ';'", span);
                }
            };
            match self.peek().0 {
                Token::Op(ref op) if op == "->" => {
                    self.next();
                }
                _ => {}
            }
            types.push(type_name);
        }

        let Some((_, end)) = self
            .consume(Token::Op(";".into())) else {
                let span = self.peek().1;
                return self.report("E10", "type declaration's end with a ';'", span);
            };
        let span = start.start..end.end;
        Expr::TypeDec(name.into(), types, span)
    }

    fn user_type_def(&mut self, start: Span) -> Expr {
        let Token::Id(name) = self.next().0 else {
            return self.report("E1", "missing identifier", start);
        };

        let mut variants = vec![];
        if let Some(_) = self.consume(Token::Op("=".into())) {
            while let Token::Id(name) = self.next().0 {
                let mut type_list = vec![];
                while let Some((Token::Id(type_id), _)) =
                    self.next_if(|t| matches!(t, Token::Id(_)))
                {
                    type_list.push(type_id);
                }
                variants.push((name, type_list));
                if let None = self.next_if(|t| t == Token::Op("|".into())) {
                    break;
                }
            }
        }
        let Some((_, end)) = self
            .consume(Token::Op(";".into())) else {
                let span = self.peek().1;
                return self.report("E10", "type declaration's end with a ';'", span);
            };
        let span = start.start..end.end;
        Expr::Type(name.into(), variants, span)
    }

    pub(crate) fn function(&mut self, start: Span) -> Expr {
        let Token::Id(name) = self.next().0 else {
            return self.report("E1", "missing identifier", start);
        };
        let body = self
            .expression(Precedence::Fn)
            .and_then(|lhs| {
                let mut args = vec![lhs];
                while let Token::Id(_) = self.peek().0 {
                    args.push(self.expression(Precedence::Fn));
                }
                let Some(_) = self.consume(Token::Op("=".into())) else {
                    let span = self.peek().1;
                    return self.report("E13", "After args '=' then function body", span);
                };
                let body = self.closure();
                let f = args.into_iter().rev().fold(body, |last, next| {
                    let span = start.start..last.span().end;
                    Expr::Closure(Box::new(next), Box::new(last), span)
                });
                f
            })
        .or_else(|_| {
            self.errors.pop();
            let Some(_) = self.consume(Token::Op("=".into())) else {
                let span = self.peek().1;
                return self.report("E11", "function requires '=' after name or params", span);
            };
            let lhs = self.closure();
            lhs
        });
        let Some((_, end)) = self.consume(Token::Op(";".into())) else {
            let span = self.peek().1;
            return self.report("E10", "functions end with a ';'", span);
        };
        let span = start.start..end.end;
        let func = Expr::Func(name, Box::new(body), span);
        func
    }

    pub(crate) fn closure(&mut self) -> Expr {
        if matches!(self.peek(), (Token::Op(ref op), _) if op == "λ" || op == "\\") {
            self.lexer.next();
            let head = Box::new(self.expression(Precedence::Fn));
            let tail = Box::new(
                self.closure()
                .and_then(|mut lhs| {
                    while let Token::Id(_) = self.peek().0 {
                        let tail = Box::new(self.expression(Precedence::Fn));
                        let span = lhs.span().start..tail.span().end;
                        lhs = Expr::Closure(Box::new(lhs), tail, span);
                    }
                    let Some(_) = self.consume(Token::Op("->".into())) else {
                        let span = self.peek().1;
                        return self.report("E12", "missing '->' after closure param", span);
                    };
                    let body = self.closure();
                    let span = lhs.span().start..body.span().end;
                    Expr::Closure(Box::new(lhs), Box::new(body), span)
                })
                .or_else(|_| {
                    self.errors.pop();
                    let Some(_) = self.consume(Token::Op("->".into())) else {
                        let span = self.peek().1;
                        return self.report("E12", "missing '->' after closure param", span);
                    };
                    self.closure()
                }),
                );
            let span = head.span().start..tail.span().end;
            return Expr::Closure(head, tail, span);
        }
        self.conditional()
    }

    pub(crate) fn conditional(&mut self) -> Expr {
        if matches!(self.peek(), (Token::KeyWord(ref k), _) if k == "if") {
            let Some((_, _)) = self.consume(Token::KeyWord("if".into())) else {
                let span = self.peek().1;
                return self.report("E13", "missing if keyword", span);
            };
            let condition = self.expression(Precedence::None);
            if condition.is_error() {
                self.errors.pop();
                return self.report(
                    "E20",
                    "expected a condition for if statement",
                    condition.span(),
                );
            }
            let Some((_, start)) = self.consume(Token::KeyWord("then".into())) else {
                let span = self.peek().1;
                return self.report("E13", "missing then keyword after if condition", span);
            };
            let branch1 = self.closure();
            let Some(_) = self.consume(Token::KeyWord("else".into())) else {
                let span = self.peek().1;
                return self.report("E13", "missing else keyword after then branch", span);
            };
            let branch2 = self.closure();
            let span = start.start..branch2.span().end;
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
        if match self.peek().0 {
            Token::Op(ref op) if op == "(" => true,
            Token::Op(_) | Token::KeyWord(_) | Token::Eof => false,
            _ => true,
        } {
            let mut args = vec![];
            let last;
            loop {
                match self.peek() {
                    (Token::Op(ref op), _) if op == "(" => {}
                    (Token::Op(_) | Token::Eof, span) => {
                        last = Some(span);
                        break;
                    }
                    (_, _) => {}
                };
                args.push(self.expression(Precedence::None));
            }
            let span = lhs.span().start..last.unwrap_or(lhs.span()).end;
            lhs = Expr::App(Box::new(lhs), args, span);
        }
        lhs
    }

    fn prefix_op(&mut self, op: &str, span: Span) -> Expr {
        match op {
            "(" => {
                self.next();
                let lhs = self.closure();
                let Some(_) = self.consume(Token::Op(")".into())) else {
                    let span = self.peek().1;
                    return self.report("E13", "closing ')' missing", span);
                };
                lhs
            }
            o @ ("-" | "!") => {
                self.next();
                let op = Op::try_from(o).unwrap();
                let lhs = self.expression(Precedence::Unary);
                let span = span.start..lhs.span().end;
                Expr::Unary(op, Box::new(lhs), span)
            }
            _ => {
                let mut l = self.lexer.clone();
                l.next();
                let peek = l.peek().map(|(t, _)| t.clone()).unwrap_or(Token::Eof);
                if Op::try_from(op).is_ok() && matches!(peek, Token::Op(op) if op == ")")
                {
                    self.lexer = l;
                    let op = Op::try_from(op).unwrap();
                    Expr::Atom(Atom::Id(format!("({op})")), span)
                } else {
                    return self.report("E2", "unknown op char", span);
                }
            }
        }
    }

    pub(crate) fn expression(&mut self, min_bp: Precedence) -> Expr {
        let (token, start_span) = self.peek();
        let mut lhs = match token {
            Token::KeyWord(ref b) if b == "true" => {
                self.next();
                Expr::Atom(Atom::Bool(true), start_span.clone())
            }
            Token::KeyWord(ref b) if b == "false" => {
                self.next();
                Expr::Atom(Atom::Bool(false), start_span.clone())
            }
            Token::Int(int) => {
                self.next();
                Expr::Atom(Atom::Int(int.parse().unwrap()), start_span.clone())
            }
            Token::Float(float) => {
                self.next();
                Expr::Atom(Atom::Float(float.parse().unwrap()), start_span.clone())
            }
            Token::Id(ref id) => {
                self.next();
                Expr::Atom(Atom::Id(id.into()), start_span.clone())
            }
            Token::String(ref string) => {
                self.next();
                Expr::Atom(Atom::String(string.into()), start_span.clone())
            }
            Token::Char(ref c) => {
                self.next();
                if c.chars().count() > 1 {
                    return self.report("E3", "invalid op char", start_span);
                }
                let Some(c) = c.chars().nth(0) else {
                    return self.report("E4", "invalid char definition", start_span);
                };
                Expr::Atom(Atom::Char(c), start_span.clone())
            }
            Token::Op(ref op) => self.prefix_op(op, start_span.clone()),
            _ => {
                return self.report("E5", "invalid token", start_span.clone());
            }
        };
        loop {
            let (token, span) = self.peek();
            let cbp: Precedence = match token.clone() {
                Token::Op(_) => Precedence::try_from(token.clone()).unwrap(),
                _ => break,
            };
            if cbp <= min_bp {
                break;
            }
            let span = start_span.clone().start..span.end;
            match cbp {
                Precedence::Term
                | Precedence::Factor
                | Precedence::Comparison
                | Precedence::Equality => {
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
                Precedence::Pipe => {
                    let _ = self.next();
                    let rhs = self.call(cbp);
                    lhs = Expr::Binary(
                        Op::try_from(&token).unwrap(),
                        Box::new(lhs),
                        Box::new(rhs),
                        span,
                    );
                }
                _ => {
                    lhs = self.report("E5", "invalid token", span);
                }
            }
        }
        lhs
    }
}
