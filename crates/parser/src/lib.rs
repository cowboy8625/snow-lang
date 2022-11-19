#![allow(dead_code)]
type Span = std::ops::Range<usize>;
use error::CResult;
use scanner::{Scanner, Token};
use std::error::Error;
use std::iter::Peekable;

use std::fmt;

macro_rules! bail {
    ($span:expr $(, $arg:expr)* $(,)?) => {
        return Err(Box::new(ParserError::new(
                    format!($($arg,) *),
                    $span
        )))
    };
}

#[derive(Debug)]
pub struct ParserError {
    message: String,
    span: Span,
}

impl ParserError {
    pub fn new(message: String, span: Span) -> Self {
        let message = format!("{}:{} {}", span.start, span.end, message);
        Self { message, span }
    }

    pub fn span(&self) -> Span {
        self.span.clone()
    }
}

impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for ParserError {
    fn description(&self) -> &str {
        &self.message
    }
}

#[derive(Debug, Clone)]
pub enum Atom {
    Int(i32),
    Float(f32),
    Id(String),
    Bool(bool),
}

impl fmt::Display for Atom {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Int(i) => write!(f, "{i}"),
            Self::Float(i) => write!(f, "{i}"),
            Self::Id(id) => write!(f, "{id}"),
            Self::Bool(b) => write!(f, "{b}"),
        }
    }
}

#[allow(unused)]
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
enum Precedence {
    None,
    Primary,
    Term,       // + -
    Factor,     // * /
    Equality,   // == !=
    Comparison, // < > <= >=
    Assignment, // =
    Or,         // or
    And,        // and
    Pipe,       // |>
    Call,       // . ()
    Fn,         // fn function declaration
    Unary,      // ! -
}

impl Precedence {
    fn new(token: Token) -> Self {
        match token {
            Token::Int(_)
            | Token::Float(_)
            | Token::String(_)
            | Token::Char(_)
            | Token::Id(_) => Self::Primary,
            Token::KeyWord(ref b) if b == "true" || b == "false" => Self::Primary,
            Token::KeyWord(_) => Self::None,
            Token::Eof => Self::None,
            Token::Op(ref op) => match op.as_str() {
                "+" | "-" => Precedence::Term,
                "*" | "/" => Precedence::Factor,
                ">" | "<" | ">=" | "<=" => Precedence::Comparison,
                "==" | "!=" => Precedence::Equality,
                "=" => Precedence::Assignment,
                "|>" => Precedence::Pipe,
                _ => Precedence::None,
            },
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Op {
    Plus,
    Minus,
    Mult,
    Div,
    Grt,
    Les,
    GrtEq,
    LesEq,
    Eq,
    Neq,
    Not,
    Equals,
    Pipe,
}

impl TryFrom<&Token> for Op {
    type Error = &'static str;
    fn try_from(token: &Token) -> Result<Self, Self::Error> {
        match token {
            Token::Op(ref op) => Self::try_from(op),
            _ => Err("not a operator"),
        }
    }
}

impl TryFrom<&str> for Op {
    type Error = &'static str;
    fn try_from(op: &str) -> Result<Self, Self::Error> {
        match op {
            "+" => Ok(Self::Plus),
            "-" => Ok(Self::Minus),
            "*" => Ok(Self::Mult),
            "/" => Ok(Self::Div),
            ">" => Ok(Self::Grt),
            "<" => Ok(Self::Les),
            "<=" => Ok(Self::GrtEq),
            ">=" => Ok(Self::LesEq),
            "==" => Ok(Self::Eq),
            "!=" => Ok(Self::Neq),
            "!" => Ok(Self::Not),
            "=" => Ok(Self::Equals),
            "|>" => Ok(Self::Pipe),
            _ => Err("not an operator"),
        }
    }
}

impl TryFrom<&String> for Op {
    type Error = &'static str;
    fn try_from(op: &String) -> Result<Self, Self::Error> {
        Self::try_from(op.as_str())
    }
}

impl TryFrom<String> for Op {
    type Error = &'static str;
    fn try_from(op: String) -> Result<Self, Self::Error> {
        Self::try_from(op.as_str())
    }
}

impl fmt::Display for Op {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Plus => write!(f, "+"),
            Self::Minus => write!(f, "-"),
            Self::Mult => write!(f, "*"),
            Self::Div => write!(f, "/"),
            Self::Grt => write!(f, ">"),
            Self::Les => write!(f, "<"),
            Self::GrtEq => write!(f, ">="),
            Self::LesEq => write!(f, "<="),
            Self::Eq => write!(f, "=="),
            Self::Neq => write!(f, "!="),
            Self::Not => write!(f, "!"),
            Self::Equals => write!(f, "="),
            Self::Pipe => write!(f, "|>"),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Expr {
    Atom(Atom),
    Unary(Op, Box<Self>),
    Binary(Op, Box<Self>, Box<Self>),
    IfElse(Box<Expr>, Box<Expr>, Box<Expr>),
    Func(String, Vec<String>, Box<Expr>),
    App(Box<Self>, Vec<Expr>),
    Type(String, Vec<(String, Vec<String>)>),
    TypeDec(String, Vec<String>),
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Atom(i) => write!(f, "{i}"),
            Self::Unary(op, lhs) => write!(f, "({op} {lhs})"),
            Self::Binary(op, lhs, rhs) => write!(f, "({op} {lhs} {rhs})"),
            Self::IfElse(condition, branch1, branch2) => {
                write!(f, "(if ({condition}) then {branch1} else {branch2})")
            }
            Self::Func(name, args, body) => {
                write!(f, "<{}: (", name)?;
                for (i, arg) in args.iter().enumerate() {
                    write!(f, "{arg}")?;
                    if i < args.len() - 1 {
                        write!(f, ", ")?;
                    }
                }
                write!(f, ") = {}>", body)
            }
            Self::App(name, args) => {
                write!(f, "<{name}: (")?;
                for (i, arg) in args.iter().enumerate() {
                    write!(f, "{arg}")?;
                    if i < args.len() - 1 {
                        write!(f, ", ")?;
                    }
                }
                write!(f, ")>")?;
                Ok(())
            }
            Self::Type(name, args) => {
                let fstring = args.iter().enumerate().fold(
                    format!("<{name}: "),
                    |fstring, (i, (name, type_arg))| {
                        let targs = type_arg.iter().fold("".to_string(), |fstring, t| {
                            if fstring == "" {
                                format!("{t}")
                            } else {
                                format!("{fstring}, {t}")
                            }
                        });
                        if i < args.len() - 1 {
                            format!("{fstring}({name}, [{targs}]), ")
                        } else {
                            format!("{fstring}({name}, [{targs}])")
                        }
                    },
                );
                write!(f, "{fstring}>")
            }
            Self::TypeDec(name, type_list) => {
                let types = type_list.iter().fold("".to_string(), |fstring, item| {
                    if fstring == "" {
                        format!("{item}")
                    } else {
                        format!("{fstring} -> {item}")
                    }
                });
                write!(f, "<{name} :: {types}>")
            }
        }
    }
}

pub fn parse(input: &str, bs: bool) -> CResult<Vec<Expr>> {
    let lexer = Scanner::new(input).peekable();
    let mut parser = Parser::new(lexer);
    parser.parse(bs)
}

struct Parser<'a> {
    lexer: Peekable<Scanner<'a>>,
}
impl<'a> Parser<'a> {
    fn new(lexer: Peekable<Scanner<'a>>) -> Self {
        Self { lexer }
    }

    fn advance(&mut self, advance_if: Option<Token>) -> CResult<(Token, Span)> {
        let (token, span) = self.lexer.peek().cloned().unwrap_or((Token::Eof, 0..0));
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

    fn parse(&mut self, bs: bool) -> CResult<Vec<Expr>> {
        let mut result = vec![];
        while self.lexer.peek().is_some() {
            let e = self.declaration(bs)?;
            result.push(e);
        }
        Ok(result)
    }

    fn consume(&mut self, expected: Token, msg: &str) -> CResult<(Token, Span)> {
        let Some((token, span)) = self.lexer.peek().cloned() else {
            bail!(
                0..0,
                "expected '{:?}' but found '<NONE>'",
                expected);
        };

        if token != expected {
            bail!(
                span,
                "{msg}\nexpected '{:?}' but found '{:?}'",
                expected,
                token
            );
        }
        self.lexer.next();
        Ok((token, span))
    }

    fn declaration(&mut self, bs: bool) -> CResult<Expr> {
        match self.lexer.peek().cloned() {
            Some((Token::KeyWord(ref k), span)) if k == "fn" => {
                self.lexer.next();
                self.function(span)
            }
            Some((Token::KeyWord(ref k), span)) if k == "type" => {
                self.lexer.next();
                self.user_type_def(span)
            }
            Some((Token::Id(_), _)) => {
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
            Some((_, _)) if bs => {
                let expr = self.conditional();
                if expr.is_ok() {
                    self.consume(
                        Token::Op(";".into()),
                        "expressions need to be terminated with ';'",
                    )?;
                }
                expr
            }
            Some((t, span)) => {
                bail!(span, "'{:?}' are not allowed in global scope", t)
            }
            None => {
                bail!(0..0, "unexpected end to file")
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
            match self.lexer.peek().map(|(t, _)| t.clone()) {
                Some(Token::Op(ref op)) if op == "->" => {
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

        self.consume(Token::Op("=".into()), "expected '=' but found")?;
        let mut variants = vec![];
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
        self.consume(Token::Op(";".into()), "type's end with a ';'")?;
        Ok(Expr::Type(name.into(), variants))
    }

    fn function(&mut self, span: Span) -> CResult<Expr> {
        let Some((Token::Id(name), _span)) = self.lexer.next() else {
            bail!(span, "expected a identifier <name>");
        };
        let mut prams = vec![];
        while let Some((t, _)) = self.lexer.next_if(|(t, _)| matches!(t, Token::Id(_))) {
            let id = match t {
                Token::Id(id) => id.to_string(),
                _ => unreachable!(),
            };
            prams.push(id);
        }

        let (_, _) =
            self.consume(Token::Op("=".into()), "After args '=' then function body")?;
        let body = self.conditional()?;
        let (_, _span) =
            self.consume(Token::Op(";".into()), "functions end with a ';'")?;
        Ok(Expr::Func(name, prams, Box::new(body)))
    }

    fn conditional(&mut self) -> CResult<Expr> {
        if matches!(self.lexer.peek(), Some((Token::KeyWord(ref k), _)) if k == "if") {
            let (_, _start_span) = self.consume(Token::KeyWord("if".into()), "WHAT")?;
            let condition = self.expression(Precedence::None)?;
            self.consume(
                Token::KeyWord("then".into()),
                "else keyword is required with an if expression",
            )?;
            let branch1 = self.conditional()?;
            self.consume(
                Token::KeyWord("else".into()),
                "else keyword is required with an if expression",
            )?;
            let branch2 = self.conditional()?;
            return Ok(Expr::IfElse(
                Box::new(condition),
                Box::new(branch1),
                Box::new(branch2),
            ));
        }
        self.call(Precedence::None)
    }

    fn call(&mut self, min_bp: Precedence) -> CResult<Expr> {
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
                let lhs = self.call(Precedence::None)?;
                self.consume(Token::Op(")".into()), "closing ')' missing")?;
                Ok(lhs)
            }
            o @ ("-" | "!") => {
                let op = Op::try_from(o)?;
                let lhs = self.expression(Precedence::Unary)?;
                Ok(Expr::Unary(op, Box::new(lhs)))
            }
            c => bail!(span, "unknown op char: {}", c),
        }
    }

    fn expression(&mut self, min_bp: Precedence) -> CResult<Expr> {
        let (token, start_span) = self.advance(None)?;
        let mut lhs = match token {
            Token::KeyWord(ref b) if b == "true" => Expr::Atom(Atom::Bool(true)),
            Token::KeyWord(ref b) if b == "false" => Expr::Atom(Atom::Bool(false)),
            Token::Int(int) => Expr::Atom(Atom::Int(int.parse().unwrap())),
            Token::Float(float) => Expr::Atom(Atom::Float(float.parse().unwrap())),
            Token::Id(ref id) => Expr::Atom(Atom::Id(id.into())),
            Token::Op(ref op) => self.prefix_op(op, start_span.clone())?,
            t => bail!(start_span, "bad token: {:?}", t),
        };
        loop {
            let (token, span) = match self.lexer.peek().cloned() {
                Some((token, span)) => (token, span),
                None => (Token::Eof, (0..0)),
            };
            let cbp: Precedence = match token.clone() {
                Token::Op(_) => Precedence::new(token.clone()),
                _ => break,
            };
            if cbp < min_bp {
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

#[cfg(test)]
mod tests {
    use super::*;

    // macro_rules! setup_test {
    //     ($name:ident $(, $input:expr, $output:expr)* $(,)?) => {
    //         #[test]
    //         fn $name() -> CResult<()> {
    //             $(
    //                 let s = parse($input)?;
    //                 dbg!(&s);
    //                 for (i, o) in s.iter().zip($output) {
    //                     assert_eq!(i.to_string(), o);
    //                 }
    //             ) *
    //                 Ok(())
    //         }
    //     };
    // }

    #[test]
    fn expression() -> CResult<()> {
        let lexer = Scanner::new("1").peekable();
        let mut parser = Parser::new(lexer);
        let left = parser.expression(Precedence::None)?.to_string();
        assert_eq!(left, "1");

        let lexer = Scanner::new("1.2").peekable();
        let mut parser = Parser::new(lexer);
        let left = parser.expression(Precedence::None)?.to_string();
        assert_eq!(left, "1.2");

        let lexer = Scanner::new("a").peekable();
        let mut parser = Parser::new(lexer);
        let left = parser.expression(Precedence::None)?.to_string();
        assert_eq!(left, "a");

        Ok(())
    }
    #[test]
    fn unary() -> CResult<()> {
        let lexer = Scanner::new("-1").peekable();
        let mut parser = Parser::new(lexer);
        let left = parser.expression(Precedence::None)?.to_string();
        assert_eq!(left, "(- 1)");

        let lexer = Scanner::new("--1").peekable();
        let mut parser = Parser::new(lexer);
        let left = parser.expression(Precedence::None)?.to_string();
        assert_eq!(left, "(- (- 1))");

        let lexer = Scanner::new("(- 1.2)").peekable();
        let mut parser = Parser::new(lexer);
        let left = parser.expression(Precedence::None)?.to_string();
        assert_eq!(left, "(- 1.2)");

        let lexer = Scanner::new("-a").peekable();
        let mut parser = Parser::new(lexer);
        let left = parser.expression(Precedence::None)?.to_string();
        assert_eq!(left, "(- a)");

        Ok(())
    }

    #[test]
    fn binary() -> CResult<()> {
        let lexer = Scanner::new("1 + 2 * 3").peekable();
        let mut parser = Parser::new(lexer);
        let left = parser.expression(Precedence::None)?.to_string();
        assert_eq!(left, "(+ 1 (* 2 3))");
        Ok(())
    }

    #[test]
    fn binary_ids() -> CResult<()> {
        let lexer = Scanner::new("a + b * c * d + e").peekable();
        let mut parser = Parser::new(lexer);
        let left = parser.expression(Precedence::None)?.to_string();
        assert_eq!(left, "(+ a (+ (* b (* c d)) e))");

        let lexer = Scanner::new("a + b").peekable();
        let mut parser = Parser::new(lexer);
        let left = parser.expression(Precedence::None)?.to_string();
        assert_eq!(left, "(+ a b)");
        Ok(())
    }

    #[test]
    fn changing_precedence() -> CResult<()> {
        let lexer = Scanner::new("(-1 + 2) * 3 - -4").peekable();
        let mut parser = Parser::new(lexer);
        let left = parser.expression(Precedence::None)?.to_string();
        assert_eq!(left, "(- (* (+ (- 1) 2) 3) (- 4))");

        let lexer = Scanner::new("(((a)))").peekable();
        let mut parser = Parser::new(lexer);
        let left = parser.expression(Precedence::None)?.to_string();
        assert_eq!(left, "a");
        Ok(())
    }

    #[test]
    fn calling_operator() -> CResult<()> {
        let lexer = Scanner::new("(+) 1 2").peekable();
        let mut parser = Parser::new(lexer);
        let left = parser.call(Precedence::None)?.to_string();
        assert_eq!(left, "<(+): (1, 2)>");
        // let exprs = parser.parse(false)?;
        // let mut e = exprs.iter();
        // assert_eq!(
        //     e.next().map(ToString::to_string),
        //     Some("<(+): (1, 2)>".into())
        // );
        Ok(())
    }

    #[test]
    fn call() -> CResult<()> {
        let lexer = Scanner::new("add 1 2").peekable();
        let mut parser = Parser::new(lexer);
        let left = parser.call(Precedence::None)?.to_string();
        assert_eq!(left, "<add: (1, 2)>");
        Ok(())
    }

    #[test]
    fn pipe_call() -> CResult<()> {
        let lexer = Scanner::new("2 |> add 1").peekable();
        let mut parser = Parser::new(lexer);
        let left = parser.call(Precedence::None)?.to_string();
        assert_eq!(left, "(|> 2 <add: (1)>)");
        Ok(())
    }

    #[test]
    fn conditional() -> CResult<()> {
        let lexer = Scanner::new("if x > y then x else y").peekable();
        let mut parser = Parser::new(lexer);
        let left = parser.conditional()?.to_string();
        assert_eq!(left, "(if ((> x y)) then x else y)");
        Ok(())
    }

    #[test]
    fn super_duper_function_def() -> CResult<()> {
        let lexer =
            Scanner::new("fn main = print (max ((add 1 2) + (sub 1 2)) 20);").peekable();
        let mut parser = Parser::new(lexer);
        parser.lexer.next();
        let left = parser.function(0..0)?.to_string();
        assert_eq!(
            left,
            "<main: () = <print: (<max: ((+ <add: (1, 2)> <sub: (1, 2)>), 20)>)>>"
        );
        Ok(())
    }
    #[test]
    fn function_def() -> CResult<()> {
        let lexer = Scanner::new("fn add x y = x + y;").peekable();
        let mut parser = Parser::new(lexer);
        parser.lexer.next();
        let left = parser.function(0..0)?.to_string();
        assert_eq!(left, "<add: (x, y) = (+ x y)>");
        Ok(())
    }

    #[test]
    fn multi_function_def() -> CResult<()> {
        let lexer = Scanner::new("fn add x y = x + y; fn sub x y = x - y;").peekable();
        let mut parser = Parser::new(lexer);
        let exprs = parser.parse(false)?;
        let mut e = exprs.iter();
        assert_eq!(e.next().unwrap().to_string(), "<add: (x, y) = (+ x y)>");
        assert_eq!(e.next().unwrap().to_string(), "<sub: (x, y) = (- x y)>");

        Ok(())
    }

    #[test]
    fn user_type_def() -> CResult<()> {
        let lexer = Scanner::new("type Option = Some Int | None;").peekable();
        let mut parser = Parser::new(lexer);
        let exprs = parser.parse(false)?;
        let mut e = exprs.iter();
        assert_eq!(
            e.next().unwrap().to_string(),
            "<Option: (Some, [Int]), (None, [])>"
        );

        Ok(())
    }

    #[test]
    fn type_dec() -> CResult<()> {
        let lexer = Scanner::new("add :: Int -> Int -> Int;").peekable();
        let mut parser = Parser::new(lexer);
        let exprs = parser.parse(false)?;
        let mut e = exprs.iter();
        assert_eq!(e.next().unwrap().to_string(), "<add :: Int -> Int -> Int>");

        Ok(())
    }
}
