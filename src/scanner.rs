use super::{
    error::{Error, Result},
    position::{CharPos, Span, Spanned},
};

use std::{fmt, iter::Peekable};
type Stream<'a, T> = Peekable<std::slice::Iter<'a, T>>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum KeyWord {
    True,
    False,
    Return,
    Let,
    In,
    And,
    Or,
    Not,
    If,
    Then,
    Else,
    Do,
    Print,
    PrintLn,
    DbgInt,
}

impl fmt::Display for KeyWord {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::True => write!(f, "True"),
            Self::False => write!(f, "False"),
            Self::Return => write!(f, "return"),
            Self::Let => write!(f, "let"),
            Self::In => write!(f, "in"),
            Self::And => write!(f, "and"),
            Self::Or => write!(f, "or"),
            Self::Not => write!(f, "not"),
            Self::If => write!(f, "if"),
            Self::Then => write!(f, "then"),
            Self::Else => write!(f, "else"),
            Self::Do => write!(f, "do"),
            Self::Print => write!(f, "print"),
            Self::PrintLn => write!(f, "println"),
            Self::DbgInt => write!(f, "dbg_int"),
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
            "in" => Some(In),
            "and" => Some(And),
            "or" => Some(Or),
            "not" => Some(Not),
            "if" => Some(If),
            "then" => Some(Then),
            "else" => Some(Else),
            "do" => Some(Do),
            "print" => Some(Print),
            "println" => Some(PrintLn),
            "dbg_int" => Some(DbgInt),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Int(String),
    Float(String),
    String(String),
    Id(String),
    Fn(String),
    KeyWord(KeyWord),
    InDent,
    DeDent,
    Delimiter,
    Op(&'static str),
    Ctrl(char),
}

impl Token {
    pub fn unwrap(&self) -> String {
        match self {
            Self::Int(i) => i.to_string(),
            Self::Float(i) => i.to_string(),
            Self::String(i) => i.to_string(),
            Self::Id(i) => i.to_string(),
            Self::Fn(i) => i.to_string(),
            Self::KeyWord(i) => i.to_string(),
            Self::InDent => "InDent".into(),
            Self::DeDent => "DeDent".into(),
            Self::Delimiter => ";".into(),
            Self::Op(i) => i.to_string(),
            Self::Ctrl(i) => i.to_string(),
        }
    }

    pub fn name(&self) -> String {
        match self {
            Self::Int(_) => "Int".to_string(),
            Self::Float(_) => "Float".to_string(),
            Self::String(_) => "String".to_string(),
            Self::Id(_) => "Id".to_string(),
            Self::Fn(i) => i.to_string(),
            Self::KeyWord(i) => i.to_string(),
            Self::InDent => "InDent".to_string(),
            Self::DeDent => "DeDent".to_string(),
            Self::Delimiter => ";".into(),
            Self::Op(_) => "Operator".to_string(),
            Self::Ctrl(i) => match i {
                '(' | ')' => "parenthesis".to_string(),
                '[' | ']' => "brackets".to_string(),
                '{' | '}' => "braces".to_string(),
                _ => "unknown".to_string(),
            },
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Int(i) => write!(f, "Int({})", i),
            Self::Float(i) => write!(f, "Float({})", i),
            Self::String(i) => write!(f, "String({})", i),
            Self::Id(i) => write!(f, "Id({})", i),
            Self::Fn(i) => write!(f, "Fn({})", i),
            Self::KeyWord(i) => write!(f, "KeyWord({})", i),
            Self::InDent => write!(f, "InDent"),
            Self::DeDent => write!(f, "DeDent"),
            Self::Delimiter => write!(f, ";"),
            Self::Op(i) => write!(f, "Op({})", i),
            Self::Ctrl(i) => write!(f, "Ctrl({})", i),
        }
    }
}

struct Scanner<'a> {
    stream: Stream<'a, CharPos>,
    tokens: Vec<Spanned<Token>>,
    errors: Option<Error>,
    delimiters: Vec<Spanned<Token>>,
    indent: Vec<usize>,
    current: Option<&'a CharPos>,
    previous: char,
    is_in_do_block: usize,
}

impl<'a> Scanner<'a> {
    fn new(src: &'a [CharPos]) -> Self {
        Self {
            stream: src.iter().peekable(),
            tokens: Vec::new(),
            errors: None,
            delimiters: Vec::new(),
            indent: Vec::new(),
            current: None,
            previous: '\n',
            is_in_do_block: 0,
        }
    }

    fn next(&mut self) -> Option<&'a CharPos> {
        if let Some(cp) = self.current {
            self.previous = cp.chr;
        }
        self.current = self.stream.next();
        self.current
    }

    fn next_if(&mut self, func: impl FnOnce(&&CharPos) -> bool) -> Option<&'a CharPos> {
        match (self.current, self.stream.next_if(func)) {
            (Some(current), next @ Some(_)) => {
                self.previous = current.chr;
                self.current = next;
            }
            (_, next @ Some(_)) => self.current = next,
            _ => return None,
        }
        self.current
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

    fn scan(mut self) -> Self {
        while let Some(cp) = self.next() {
            match cp.chr {
                '-' if self.peek_char() == '-' => self.line_comment(),
                '{' if self.peek_char() == '-' => self.block_comment(),
                '\n' => {
                    self.delimiter(cp);
                    self.indent();
                }
                // '\n' if self.peek_char().is_ascii_alphabetic() => self.dedent(cp),
                '=' if self.peek_char() == '=' => {
                    let end = self.next().unwrap();
                    self.push((Token::Op("=="), (cp, end).into()))
                }
                '!' if self.peek_char() == '=' => {
                    let end = self.next().unwrap();
                    self.push((Token::Op("!="), (cp, end).into()))
                }
                '>' if self.peek_char() == '=' => {
                    let end = self.next().unwrap();
                    self.push((Token::Op(">="), (cp, end).into()))
                }
                '|' if self.peek_char() == '>' => {
                    let end = self.next().unwrap();
                    self.push((Token::Op("|>"), (cp, end).into()))
                }
                '<' if self.peek_char() == '|' => {
                    let end = self.next().unwrap();
                    self.push((Token::Op("<|"), (cp, end).into()))
                }
                '<' if self.peek_char() == '=' => {
                    let end = self.next().unwrap();
                    self.push((Token::Op("<="), (cp, end).into()))
                }
                ':' if self.peek_char() == ':' => {
                    let end = self.next().unwrap();
                    self.push((Token::Op("::"), (cp, end).into()))
                }
                '!' => self.push((Token::Op("!"), (cp, cp).into())),
                '=' => self.push((Token::Op("="), (cp, cp).into())),
                '-' => self.push((Token::Op("-"), (cp, cp).into())),
                '+' => self.push((Token::Op("+"), (cp, cp).into())),
                '*' => self.push((Token::Op("*"), (cp, cp).into())),
                '/' => self.push((Token::Op("/"), (cp, cp).into())),
                '>' => self.push((Token::Op(">"), (cp, cp).into())),
                '<' => self.push((Token::Op("<"), (cp, cp).into())),
                '(' => self.push((Token::Ctrl('('), (cp, cp).into())),
                ')' => self.push((Token::Ctrl(')'), (cp, cp).into())),
                '[' => self.push((Token::Ctrl('['), (cp, cp).into())),
                ']' => self.push((Token::Ctrl(']'), (cp, cp).into())),
                '{' => self.push((Token::Ctrl('{'), (cp, cp).into())),
                '}' => self.push((Token::Ctrl('}'), (cp, cp).into())),
                ',' => self.push((Token::Ctrl(','), (cp, cp).into())),
                '"' => self.string(cp),
                id if id.is_ascii_alphabetic() && self.previous == '\n' => self.func(cp),
                id if id.is_ascii_alphabetic() => self.identifier(cp),
                num if num.is_numeric() => self.number(cp),
                ' ' => {}
                _ => {
                    self.errors = Some(Error {
                        last: self.errors.clone().map(Box::new),
                        msg: format!("unknown char '{}'", cp.chr),
                        span: Span::new(cp.into(), cp.into(), &cp.loc),
                    })
                }
            }
        }
        self.unwrap_dedent();
        if let Some(item) = self.tokens.last().map(Clone::clone) {
            if item.node != Token::DeDent {
                self.push((Token::DeDent, item.span()));
            }
        }
        for spanned in self.delimiters.iter() {
            eprintln!("{}", spanned.node);
            self.errors = Some(Error {
                last: self.errors.map(Box::new),
                msg: format!(
                    "unclosed delimiter missing {} {}",
                    spanned.node.name(),
                    spanned.node.unwrap()
                ),
                span: spanned.span(),
            });
        }
        self
    }

    fn push<T>(&mut self, spanned: T)
    where
        T: Into<Spanned<Token>>,
    {
        let spanned = spanned.into();
        match &spanned.node {
            Token::Ctrl('(' | '[' | '{') => {
                self.delimiters.push(spanned.clone());
            }
            Token::Ctrl(n @ (')' | ']' | '}')) => {
                let opsit = match n {
                    ')' => "(",
                    '}' => "{",
                    ']' => "]",
                    _ => unreachable!(),
                };
                let open = self.delimiters.pop().unwrap_or(spanned.clone());
                // let span: Span = (open.span(), spanned.span()).into();
                if opsit != open.node.unwrap() {
                    let node = &open.node;
                    self.errors = Some(Error {
                        last: self.errors.clone().map(Box::new),
                        msg: format!(
                            "unclosed delimiter missing {} {}",
                            node.name(),
                            node.unwrap()
                        ),
                        span: open.span(),
                    });
                }
            }
            _ => {}
        }
        if let Token::KeyWord(KeyWord::Do) = spanned.node {
            self.is_in_do_block += 1;
        }
        if let Token::DeDent = spanned.node {
            self.is_in_do_block -= 1;
        }
        self.tokens.push(spanned.into());
    }

    fn line_comment(&mut self) {
        while let Some(_) = self.next_if(|cp| cp.chr != '\n') {}
    }

    fn block_comment(&mut self) {
        let mut last = '\0';
        while let Some(cp) = self.next() {
            if last == '-' && cp.chr == '}' {
                break;
            }
            last = cp.chr;
        }
    }

    fn delimiter(&mut self, cp: &CharPos) {
        if self.is_in_do_block > 0
            && !matches!(
                self.tokens.last(),
                Some(Spanned {
                    node: Token::KeyWord(KeyWord::Do | KeyWord::Then | KeyWord::Else),
                    ..
                })
            )
        {
            self.push((Token::Delimiter, (cp, cp).into()));
        }
    }

    fn indent(&mut self) {
        let mut count = 0;
        // let start = self.next().unwrap();
        let mut start: Option<&'a CharPos> = None;
        let mut span: Option<Span> = None;
        while let Some(cp) = self.next_if(|&cp| cp.chr == ' ') {
            if start.is_none() {
                start = Some(cp);
            }
            count += 1;
            span = Some((start.unwrap(), cp).into());
        }

        // Code Donated by MizardX 😃
        match count.cmp(&self.indent.last().unwrap_or(&0)) {
            std::cmp::Ordering::Greater => {
                self.indent.push(count);
                self.push((Token::InDent, span.unwrap_or_default()));
            }
            std::cmp::Ordering::Less => {
                self.indent.pop();
                self.push((Token::DeDent, span.clone().unwrap_or_default()));
                while count < *self.indent.last().unwrap_or(&0) {
                    let _ = self.indent.pop();
                    self.push((Token::DeDent, span.clone().unwrap_or_default()));
                }
                if count > *self.indent.last().unwrap_or(&0) {
                    self.errors = Some(Error {
                        last: self.errors.clone().map(Box::new),
                        msg: "invalid indention".into(),
                        span: span.unwrap_or_default(),
                    });
                }
            }
            _ => (),
        }
    }

    fn unwrap_dedent(&mut self) {
        for _ in self.indent.clone().iter() {
            self.push((
                Token::DeDent,
                self.tokens.last().map(|t| t.span()).unwrap_or_default(),
            ));
        }
        self.indent.clear();
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
        let span: Span = (start, end).into();
        let token = if number.contains('.') {
            (Token::Float(number), span)
        } else {
            (Token::Int(number), span)
        };
        self.push(token);
    }

    fn func(&mut self, start: &CharPos) {
        let mut idt = start.chr.to_string();
        let mut end = start;
        while let Some(cp) = self.next_if(|&cp| cp.chr.is_ascii_alphanumeric()) {
            end = cp;
            idt.push(cp.chr);
        }
        let span: Span = (start, end).into();
        if KeyWord::lookup(&idt).is_some() {
            self.errors = Some(Error {
                last: self.errors.clone().map(Box::new),
                msg: format!("'{}' is a reserved word found in function name", idt),
                span: span.clone(),
            });
        }

        self.push((Token::Fn(idt), span));
    }

    fn identifier(&mut self, start: &CharPos) {
        let mut idt = start.chr.to_string();
        let mut end = start;
        while let Some(cp) = self.next_if(|&cp| cp.chr.is_ascii_alphanumeric() || cp.chr == '_') {
            end = cp;
            idt.push(cp.chr);
        }
        let span: Span = (start, end).into();
        let token_id = KeyWord::lookup(&idt).map_or((Token::Id(idt), span.clone()), |n| {
            (Token::KeyWord(n), span)
        });
        self.push(token_id);
    }

    fn string(&mut self, start: &CharPos) {
        let mut string = String::new();
        while let Some(cp) = self.next_if(|cp| cp.chr != '"') {
            string.push(cp.chr);
        }
        let end = self.next().unwrap();
        let span: Span = (start, end).into();
        self.push((
            Token::String(
                string
                    // NOTE: There are more to cover
                    // This image shows a few more.
                    // https://image.slidesharecdn.com/cbasics-100427070048-phpapp01/95/c-basics-9-728.jpg?cb=1272351721
                    .replace("\\r", "\r")
                    .replace("\\n", "\n")
                    .replace("\\t", "\t")
                    .replace("\\x1b", "\x1b"),
            ),
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
                row: 1,
                loc: loc.into(),
            });
            if idx != 1 {
                last.row += 1;
            }
            if chr == '\n' {
                last.row = 1;
                last.col += 1;
            }
            acc.push(CharPos {
                chr,
                idx,
                col: last.col,
                row: last.row,
                loc: loc.into(),
            });
            acc
        })
}

// FIXME: This does not need to clone.
pub fn scanner(filename: &str, src: &str) -> Result<Vec<Spanned<Token>>> {
    let chrpos = pos_enum(filename, src);
    let scanner = Scanner::new(&chrpos).scan();
    if let Some(error) = scanner.errors {
        return Err(error);
    }
    Ok(scanner.tokens.clone())
}
