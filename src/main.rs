// #[cfg(test)]
// mod test

// fn snow_source_file(filename: &str) -> Result<String, String> {
//     if filename.ends_with(".snow") {
//         match std::fs::read_to_string(filename) {
//             Ok(file) => Ok(file),
//             Err(e) => Err(e.to_string()),
//         }
//     } else {
//         Err("This is not `snow` source file.".into())
//     }
// }
#![allow(unused)]
use std::{fmt, iter::Peekable};
type Stream<'a, T> = Peekable<std::slice::Iter<'a, T>>;

#[derive(Debug, Clone)]
struct CharPos {
    chr: char,
    idx: usize,
    col: usize,
    row: usize,
    loc: String,
}

#[derive(Debug, Clone)]
struct Pos {
    idx: usize,
    col: usize,
    row: usize,
}

impl From<&CharPos> for Pos {
    fn from(char_pos: &CharPos) -> Self {
        Self {
            idx: char_pos.idx,
            col: char_pos.col,
            row: char_pos.row,
        }
    }
}

#[derive(Debug, Clone)]
struct Span {
    start: Pos,
    end: Pos,
    loc: String,
}

impl Span {
    fn new(start: &CharPos, end: &CharPos) -> Self {
        Self {
            start: start.into(),
            end: end.into(),
            loc: start.loc.to_string(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum KeyWord {
    True,
    False,
    Return,
    Let,
    And,
    Or,
    Not,
    If,
    Then,
    Else,
    Do,
    Print,
    PrintLn,
}

impl fmt::Display for KeyWord {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::True => write!(f, "True"),
            Self::False => write!(f, "False"),
            Self::Return => write!(f, "return"),
            Self::Let => write!(f, "let"),
            Self::And => write!(f, "and"),
            Self::Or => write!(f, "or"),
            Self::Not => write!(f, "not"),
            Self::If => write!(f, "if"),
            Self::Then => write!(f, "then"),
            Self::Else => write!(f, "else"),
            Self::Do => write!(f, "do"),
            Self::Print => write!(f, "print"),
            Self::PrintLn => write!(f, "println"),
        }
    }
}

impl KeyWord {
    fn lookup(name: &str) -> Option<Self> {
        use KeyWord::*;
        match name {
            "True" => Some(True),
            "False" => Some(False),
            "return" => Some(Return),
            "let" => Some(Let),
            "and" => Some(And),
            "or" => Some(Or),
            "not" => Some(Not),
            "if" => Some(If),
            "then" => Some(Then),
            "else" => Some(Else),
            "do" => Some(Do),
            "print" => Some(Print),
            "println" => Some(PrintLn),
            _ => None,
        }
    }
}

#[derive(Debug, Clone)]
enum Token {
    Int(String, Span),
    Float(String, Span),
    String(String, Span),
    Id(String, Span),
    KeyWord(KeyWord, Span),
    InDent(usize, Span),
    DeDent(Span),
    Op(&'static str, Span),
    Ctrl(char, Span),
}

struct Scanner<'a> {
    stream: Stream<'a, CharPos>,
    tokens: Vec<Token>,
    errors: Vec<String>,
}

impl<'a> Scanner<'a> {
    fn new(src: &'a [CharPos]) -> Self {
        Self {
            stream: src.iter().peekable(),
            tokens: Vec::new(),
            errors: Vec::new(),
        }
    }
    fn scan(mut self) -> Self {
        while let Some(cp) = self.stream.next() {
            match cp.chr {
                '/' if self.peek_char() == '/' => self.line_comment(),
                '{' if self.peek_char() == '-' => self.block_comment(),
                '\n' if self.peek_char() == ' ' => self.indent(),
                '\n' if self.peek_char().is_ascii_alphabetic() => self.dedent(cp),
                '=' if self.peek_char() == '=' => {
                    let end = self.stream.next().unwrap();
                    self.tokens.push(Token::Op("==", Span::new(cp, end)))
                }
                '!' if self.peek_char() == '=' => {
                    let end = self.stream.next().unwrap();
                    self.tokens.push(Token::Op("!=", Span::new(cp, end)))
                }
                '>' if self.peek_char() == '=' => {
                    let end = self.stream.next().unwrap();
                    self.tokens.push(Token::Op(">=", Span::new(cp, end)))
                }
                '<' if self.peek_char() == '=' => {
                    let end = self.stream.next().unwrap();
                    self.tokens.push(Token::Op("<=", Span::new(cp, end)))
                }
                ':' if self.peek_char() == ':' => {
                    let end = self.stream.next().unwrap();
                    self.tokens.push(Token::Op("::", Span::new(cp, end)))
                }
                '=' => self.tokens.push(Token::Op("=", Span::new(cp, cp))),
                '>' => self.tokens.push(Token::Op(">", Span::new(cp, cp))),
                '<' => self.tokens.push(Token::Op("<", Span::new(cp, cp))),
                '-' => self.tokens.push(Token::Op("-", Span::new(cp, cp))),
                '+' => self.tokens.push(Token::Op("+", Span::new(cp, cp))),
                '*' => self.tokens.push(Token::Op("*", Span::new(cp, cp))),
                '/' => self.tokens.push(Token::Op("/", Span::new(cp, cp))),
                '(' => self.tokens.push(Token::Ctrl('(', Span::new(cp, cp))),
                ')' => self.tokens.push(Token::Ctrl(')', Span::new(cp, cp))),
                '[' => self.tokens.push(Token::Ctrl('[', Span::new(cp, cp))),
                ']' => self.tokens.push(Token::Ctrl(']', Span::new(cp, cp))),
                '{' => self.tokens.push(Token::Ctrl('{', Span::new(cp, cp))),
                '}' => self.tokens.push(Token::Ctrl('}', Span::new(cp, cp))),
                '"' => self.string(cp),
                id if id.is_ascii_alphabetic() => self.identifier(cp),
                num if num.is_numeric() => self.number(cp),
                _ => self.error(cp.chr),
            }
        }
        self
    }

    fn peek_char(&mut self) -> char {
        self.stream
            .peek()
            .unwrap_or(&&CharPos {
                chr: '\0',
                idx: 0,
                row: 0,
                col: 0,
                loc: "ERROR".into(),
            })
            .chr
    }

    fn error(&mut self, c: char) {
        self.errors.push(c.to_string());
    }

    fn line_comment(&mut self) {
        while let Some(cp) = self.stream.next() {
            if cp.chr == '\n' {
                break;
            }
        }
    }

    fn block_comment(&mut self) {
        let mut last = '\0';
        while let Some(cp) = self.stream.next() {
            if last == '-' && cp.chr == '}' {
                break;
            }
            last = cp.chr;
        }
    }

    fn indent(&mut self) {
        let mut count = 0;
        let start = self.stream.next().unwrap();
        let mut end = start;
        while let Some(cp) = self.stream.next_if(|&cp| cp.chr == ' ') {
            end = cp;
            count += 1;
        }
        let span = Span::new(&start, end);
        let token = Token::InDent(count, span);
        self.tokens.push(token);
    }

    fn dedent(&mut self, start: &CharPos) {
        let span = Span::new(&start, &start);
        let token = Token::DeDent(span);
        self.tokens.push(token);
    }

    // fn character(&mut self) {}

    fn number(&mut self, start: &CharPos) {
        let mut number = start.chr.to_string();
        let mut end = start;
        while let Some(cp) = self
            .stream
            .next_if(|&cp| cp.chr.is_numeric() || (cp.chr == '.' && !number.contains('.')))
        {
            end = cp;
            number.push(cp.chr);
        }
        let span = Span::new(&start, end);
        let token = if number.contains('.') {
            Token::Float(number, span)
        } else {
            Token::Int(number, span)
        };
        self.tokens.push(token);
    }

    fn identifier(&mut self, start: &CharPos) {
        let mut idt = start.chr.to_string();
        let mut end = start;
        while let Some(cp) = self.stream.next_if(|&cp| cp.chr.is_ascii_alphanumeric()) {
            end = cp;
            idt.push(cp.chr);
        }
        let span = Span::new(start, end);
        let token_id =
            KeyWord::lookup(&idt).map_or(Token::Id(idt, span.clone()), |n| Token::KeyWord(n, span));
        self.tokens.push(token_id);
    }

    fn string(&mut self, start: &CharPos) {
        let mut string = String::new();
        while let Some(cp) = self.stream.next_if(|cp| cp.chr != '"') {
            string.push(cp.chr);
        }
        let end = self.stream.next().unwrap();
        let span = Span::new(start, end);
        self.tokens.push(Token::String(
            string
                // TODO: There are more to cover
                // This image shows a few more.
                // https://image.slidesharecdn.com/cbasics-100427070048-phpapp01/95/c-basics-9-728.jpg?cb=1272351721
                .replace("\\r", "\r")
                .replace("\\n", "\n")
                .replace("\\t", "\t")
                .replace("\\x1b", "\x1b"),
            span,
        ));
    }
}

fn pos_enum<'a>(loc: &str, src: &str) -> Vec<CharPos> {
    src.chars()
        .enumerate()
        .fold(Vec::new(), |mut acc, (idx, chr)| {
            let mut last = acc.last().map(Clone::clone).unwrap_or(CharPos {
                chr,
                idx,
                col: 1,
                row: 0,
                loc: loc.into(),
            });
            if chr == '\n' {
                last.row = 0;
                last.col += 1;
            }
            acc.push(CharPos {
                chr,
                idx,
                col: last.col,
                row: last.col,
                loc: loc.into(),
            });
            acc
        })
}

fn maybe_keyword(name: &str) -> Option<String> {
    match name {
        "True" | "False" | "return" | "let" | "and" | "or" | "not" | "if" | "then" | "else"
        | "do" | "print" | "println" => Some(name.to_string()),
        _ => None,
    }
}

#[derive(Debug, Clone, PartialEq)]
enum Value {
    Bool(bool),
    Int(i128),
    Float(f64),
    Str(String),
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
            Self::Bool(x) => write!(f, "{}", x),
            Self::Int(x) => write!(f, "{}", x),
            Self::Float(x) => write!(f, "{}", x),
            Self::Str(x) => write!(f, "{}", x),
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

#[derive(Debug, Clone)]
struct Spanned<T> {
    expr: T,
    span: Span,
}

#[derive(Debug, Clone)]
enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
    Grt,
    Les,
    Geq,
    Leq,
    Eq,
    NEq,
    Not,
}

#[derive(Debug, Clone)]
enum Expr {
    Value(Value),
    List(Vec<Spanned<Self>>),
    Local(String),
    // Let(String, Box<Spanned<Self>>, Box<Spanned<Self>>),
    If(Box<Spanned<Self>>, Box<Spanned<Self>>, Box<Spanned<Self>>),
    Then(Box<Spanned<Self>>, Box<Spanned<Self>>),
    Binary(Box<Spanned<Self>>, BinaryOp, Box<Spanned<Self>>),
    Unary(BinaryOp, Box<Spanned<Self>>),
    App(Box<Spanned<Self>>, Spanned<Vec<Spanned<Self>>>),
    Print(Box<Spanned<Self>>),
}

#[derive(Debug, Clone)]
struct Function {
    args: Vec<String>,
    body: Spanned<Expr>,
    span: Span,
}

struct Parser<'a> {
    stream: Stream<'a, Token>,
    func_expr: std::collections::HashMap<String, Function>,
}

impl<'a> Parser<'a> {
    fn new(stream: &'a [Token]) -> Self {
        Self {
            stream: stream.iter().peekable(),
            func_expr: std::collections::HashMap::new(),
        }
    }

    fn parse(&mut self) {
        self.function();
    }

    fn function(&mut self) {
        if let Some(Token::DeDent(_)) = self.stream.peek() {
            let _ = self.stream.next();
            if let Some(Token::Id(name, start)) = self.stream.next() {
                let mut args: Vec<String> = Vec::new();
                // function arguments
                while let Some(a) = self.stream.next_if(|t| !matches!(t, Token::Op("=", _))) {
                    match a {
                        Token::Id(arg_name, _) => args.push(arg_name.to_string()),
                        _ => unreachable!(),
                    }
                }
                let _ = self.stream.next();
                let body = self.expression();
                self.func_expr.insert(
                    name.to_string(),
                    Function {
                        args,
                        body,
                        span: start.clone(),
                    },
                );
                self.function();
            }
        }
    }
    fn expression(&mut self) -> Spanned<Expr> {
        self.equality()
    }

    fn equality(&mut self) -> Spanned<Expr> {
        let mut expr = self.comparison();
        while let Some(tok) = self
            .stream
            .next_if(|tok| matches!(tok, Token::Op("!=", _)) || matches!(tok, Token::Op("==", _)))
        {
            let operator = match tok {
                Token::Op("!=", _) => BinaryOp::NEq,
                Token::Op("==", _) => BinaryOp::Eq,
                _ => unreachable!(),
            };
            let right = self.comparison();
            expr = Spanned {
                span: expr.span.clone(),
                expr: Expr::Binary(Box::new(expr), operator, Box::new(right)),
            };
        }
        expr
    }
    fn comparison(&mut self) -> Spanned<Expr> {
        let mut expr = self.term();

        while let Some(tok) = self.stream.next_if(|tok| {
            matches!(tok, Token::Op(">", _))
                || matches!(tok, Token::Op(">", _))
                || matches!(tok, Token::Op(">=", _))
                || matches!(tok, Token::Op("<=", _))
        }) {
            let operator = match tok {
                Token::Op(">", _) => BinaryOp::Grt,
                Token::Op("<", _) => BinaryOp::Les,
                Token::Op(">=", _) => BinaryOp::Geq,
                Token::Op("<=", _) => BinaryOp::Leq,
                _ => unreachable!(),
            };

            let right = self.term();
            expr = Spanned {
                span: expr.span.clone(),
                expr: Expr::Binary(Box::new(expr), operator, Box::new(right)),
            };
        }

        expr
    }

    fn term(&mut self) -> Spanned<Expr> {
        let mut expr = self.factor();

        while let Some(tok) = self
            .stream
            .next_if(|tok| matches!(tok, Token::Op("+", _)) || matches!(tok, Token::Op("-", _)))
        {
            let operator = match tok {
                Token::Op("+", _) => BinaryOp::Add,
                Token::Op("-", _) => BinaryOp::Sub,
                _ => unreachable!(),
            };
            let right = self.factor();
            expr = Spanned {
                span: expr.span.clone(),
                expr: Expr::Binary(Box::new(expr), operator, Box::new(right)),
            };
        }

        expr
    }
    fn factor(&mut self) -> Spanned<Expr> {
        let mut expr = self.unary();

        while let Some(tok) = self
            .stream
            .next_if(|tok| matches!(tok, Token::Op("/", _)) || matches!(tok, Token::Op("*", _)))
        {
            let operator = match tok {
                Token::Op("/", _) => BinaryOp::Div,
                Token::Op("*", _) => BinaryOp::Mul,
                _ => unreachable!(),
            };
            let right = self.unary();
            expr = Spanned {
                span: expr.span.clone(),
                expr: Expr::Binary(Box::new(expr), operator, Box::new(right)),
            };
        }

        expr
    }
    fn unary(&mut self) -> Spanned<Expr> {
        if let Some(tok) = self
            .stream
            .next_if(|tok| matches!(tok, Token::Op("!", _)) || matches!(tok, Token::Op("-", _)))
        {
            let (operator, span) = match tok {
                Token::Op("!", span) => (BinaryOp::Not, span),
                Token::Op("-", span) => (BinaryOp::Sub, span),
                _ => unreachable!(),
            };
            let right = self.unary();
            return Spanned {
                span: span.clone(),
                expr: Expr::Unary(operator, Box::new(right)),
            };
        }

        self.app()
    }

    fn app(&mut self) -> Spanned<Expr> {
        let mut expr = self.primary();

        let mut args: Vec<Spanned<Expr>> = Vec::new();
        if let Some(tok) = self.stream.next_if(|tok| {
            matches!(expr.expr, Expr::Local(_))
                && !matches!(tok, Token::Op(_, _) | Token::DeDent(_))
        }) {
            while let Some(tok) = self.stream.next_if(|tok| !matches!(tok, Token::DeDent(_))) {
                args.push(self.expression());
            }
            println!("{:?}", args);

            let args_span = match (args.first(), args.last()) {
                (Some(start), Some(end)) => Span {
                    start: start.span.start.clone(),
                    end: end.span.end.clone(),
                    loc: start.span.loc.to_string(),
                },
                _ => expr.span.clone(),
            };
            return Spanned {
                span: expr.span.clone(),
                expr: Expr::App(
                    Box::new(expr),
                    Spanned {
                        span: args_span,
                        expr: args,
                    },
                ),
            };
        }

        expr
    }

    fn primary(&mut self) -> Spanned<Expr> {
        match self.stream.next().map(Clone::clone) {
            Some(Token::Int(int, span)) => {
                return Spanned {
                    expr: Expr::Value(Value::Int(int.parse::<i128>().unwrap())),
                    span: span.clone(),
                };
            }
            Some(Token::Float(float, span)) => {
                return Spanned {
                    expr: Expr::Value(Value::Float(float.parse::<f64>().unwrap())),
                    span: span.clone(),
                };
            }
            Some(Token::KeyWord(KeyWord::True, span)) => {
                return Spanned {
                    expr: Expr::Value(Value::Bool(true)),
                    span: span.clone(),
                };
            }
            Some(Token::KeyWord(KeyWord::False, span)) => {
                return Spanned {
                    expr: Expr::Value(Value::Bool(false)),
                    span: span.clone(),
                };
            }
            Some(Token::String(string, span)) => {
                return Spanned {
                    expr: Expr::Value(Value::Str(string.into())),
                    span: span.clone(),
                };
            }
            Some(Token::Id(string, span)) => {
                let expr = Spanned {
                    expr: Expr::Local(string.into()),
                    span: span.clone(),
                };
                return expr;
            }
            Some(Token::Ctrl('(', span)) => {
                let expr = self.expression();
                if let Some(Token::Ctrl(')', _)) = self.stream.next() {
                    return Spanned {
                        expr: expr.expr,
                        span: span.clone(),
                    };
                } else {
                    // TODO: Fix this
                    panic!("Panic in primitive method.")
                }
            }
            e => panic!("TOKEN: {:?}", e),
        }
    }
}

#[derive(Debug, Clone)]
struct Error {
    span: Span,
    msg: String,
}

fn eval_expr(
    expr: &Spanned<Expr>,
    funcs: &std::collections::HashMap<String, Function>,
    stack: &mut Vec<(String, Value)>,
) -> Result<Value, Error> {
    Ok(match &expr.expr {
        // Expr::Error => unreachable!(),
        Expr::Value(val) => val.clone(),
        // Expr::List(items) => Value::List(
        //     items
        //         .iter()
        //         .map(|item| eval_expr(item, funcs, stack))
        //         .collect::<Result<_, _>>()?,
        // ),
        Expr::Local(name) => stack
            .iter()
            .rev()
            .find(|(l, _)| l == name)
            .map(|(_, v)| v.clone())
            .or_else(|| Some(Value::Func(name.clone())).filter(|_| funcs.contains_key(name)))
            .ok_or_else(|| Error {
                span: expr.span.clone(),
                msg: format!("No such variable '{}' in scope", name),
            })?,
        // Expr::Let(local, val, body) => {
        //     let val = eval_expr(val, funcs, stack)?;
        //     stack.push((local.clone(), val));
        //     let res = eval_expr(body, funcs, stack)?;
        //     stack.pop();
        //     res
        // }
        // Expr::Then(a, b) => {
        //     eval_expr(a, funcs, stack)?;
        //     eval_expr(b, funcs, stack)?
        // }
        Expr::Binary(a, BinaryOp::Add, b) => {
            if let (Ok(lhs), Ok(rhs)) = (
                eval_expr(&a, funcs, stack)?.int(a.span.clone()),
                eval_expr(&b, funcs, stack)?.int(b.span.clone()),
            ) {
                Value::Int(lhs + rhs)
            } else {
                Value::Float(
                    eval_expr(&a, funcs, stack)?.float(a.span.clone())?
                        + eval_expr(&b, funcs, stack)?.float(b.span.clone())?,
                )
            }
        }
        Expr::Binary(a, BinaryOp::Sub, b) => {
            if let (Ok(lhs), Ok(rhs)) = (
                eval_expr(&a, funcs, stack)?.int(a.span.clone()),
                eval_expr(&b, funcs, stack)?.int(b.span.clone()),
            ) {
                Value::Int(lhs - rhs)
            } else {
                Value::Float(
                    eval_expr(&a, funcs, stack)?.float(a.span.clone())?
                        - eval_expr(&b, funcs, stack)?.float(b.span.clone())?,
                )
            }
        }
        Expr::Binary(a, BinaryOp::Mul, b) => {
            if let (Ok(lhs), Ok(rhs)) = (
                eval_expr(&a, funcs, stack)?.int(a.span.clone()),
                eval_expr(&b, funcs, stack)?.int(b.span.clone()),
            ) {
                Value::Int(lhs * rhs)
            } else {
                Value::Float(
                    eval_expr(&a, funcs, stack)?.float(a.span.clone())?
                        * eval_expr(&b, funcs, stack)?.float(b.span.clone())?,
                )
            }
        }
        Expr::Binary(a, BinaryOp::Div, b) => {
            if let (Ok(lhs), Ok(rhs)) = (
                eval_expr(&a, funcs, stack)?.int(a.span.clone()),
                eval_expr(&b, funcs, stack)?.int(b.span.clone()),
            ) {
                Value::Int(lhs / rhs)
            } else {
                Value::Float(
                    eval_expr(&a, funcs, stack)?.float(a.span.clone())?
                        / eval_expr(&b, funcs, stack)?.float(b.span.clone())?,
                )
            }
        }
        Expr::Binary(a, BinaryOp::Grt, b) => Value::Bool(
            eval_expr(&a, funcs, stack)?.int(a.span.clone())?
                > eval_expr(&b, funcs, stack)?.int(b.span.clone())?,
        ),
        Expr::Binary(a, BinaryOp::Geq, b) => Value::Bool(
            eval_expr(&a, funcs, stack)?.int(a.span.clone())?
                >= eval_expr(&b, funcs, stack)?.int(b.span.clone())?,
        ),
        Expr::Binary(a, BinaryOp::Les, b) => Value::Bool(
            eval_expr(&a, funcs, stack)?.int(a.span.clone())?
                < eval_expr(&b, funcs, stack)?.int(b.span.clone())?,
        ),
        Expr::Binary(a, BinaryOp::Leq, b) => Value::Bool(
            eval_expr(&a, funcs, stack)?.int(a.span.clone())?
                <= eval_expr(&b, funcs, stack)?.int(b.span.clone())?,
        ),
        Expr::Binary(a, BinaryOp::Eq, b) => {
            Value::Bool(eval_expr(&a, funcs, stack)? == eval_expr(&b, funcs, stack)?)
        }
        Expr::Binary(a, BinaryOp::NEq, b) => {
            Value::Bool(eval_expr(&a, funcs, stack)? != eval_expr(&b, funcs, stack)?)
        }
        Expr::App(
            func,
            Spanned {
                expr: args,
                span: args_span,
            },
        ) => {
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
                        span: func.span.clone(),
                        msg: format!("'{:?}' is not callable", f),
                    })
                }
            }
        }
        //     Expr::If(cond, a, b) => {
        //         let c = eval_expr(cond, funcs, stack)?;
        //         match c {
        //             Value::Boolean(true) => eval_expr(a, funcs, stack)?,
        //             Value::Boolean(false) => eval_expr(b, funcs, stack)?,
        //             c => {
        //                 return Err(Error {
        //                     span: cond.1.clone(),
        //                     msg: format!("Conditions must be booleans, found '{:?}'", c),
        //                 })
        //             }
        //         }
        //     }
        //     Expr::Print(a) => {
        //         let val = eval_expr(a, funcs, stack)?;
        //         print!("{}", val);
        //         val
        //     }
        //     Expr::PrintLn(a) => {
        //         let val = eval_expr(a, funcs, stack)?;
        //         println!("{}", val);
        //         val
        //     }
        e => panic!("{:?}: is not implemented yet", e),
    })
}

fn main() {
    // let src = snow_source_file(&std::env::args().nth(1).expect("Expected file argument"))
    //     .expect("Failed to open file.  Expected a Snow File.");
    let src = "
add a b = a + b

sub x y = x - y


main = add 10 23
";
    println!("{}", src);
    let chrpos = pos_enum("somefile.snow", src);
    let scanner = Scanner::new(&chrpos).scan();
    for tok in scanner.tokens.iter() {
        println!("{:?}", tok);
    }
    println!("---------------------------");
    let mut parser = Parser::new(&scanner.tokens);
    parser.parse();
    // println!("{:?}", parser.func_expr);
    for (name, func) in parser.func_expr.iter() {
        println!("{:?}: {:#?}", name, func);
    }
    if let Some(main_func) = parser.func_expr.get("main") {
        println!(
            "'{}'",
            eval_expr(&main_func.body, &parser.func_expr, &mut Vec::new()).unwrap()
        );
    }
}
