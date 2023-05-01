use super::{Span, Token};
use std::{iter::Peekable, str::Chars};
type Stream<'a> = Peekable<Chars<'a>>;

#[derive(Debug, Clone)]
pub struct Scanner<'a> {
    stream: Stream<'a>,
    span: Span,
    current: Option<char>,
    previous: Option<char>,
    keywords: Vec<&'a str>,
    line_comment: (char, Option<char>),
}

impl<'a> Scanner<'a> {
    pub fn new(src: &'a str) -> Self {
        let keywords = vec![
            "enum", "data", "type", "true", "false", "return", "let", "and", "or", "not",
            "if", "then", "else", "fn", "mod",
        ];
        Self {
            stream: src.chars().peekable(),
            span: Span::default(),
            current: None,
            previous: None,
            keywords,
            line_comment: ('-', Some('-')),
        }
    }

    fn advance(&mut self) {
        match self.current {
            Some(c) => {
                self.span.end += c.to_string().as_bytes().len();
            }
            None => {
                let Some(c) = self.previous else {
                    self.span.end += 1;
                    return;
                };
                self.span.end += c.to_string().as_bytes().len();
            }
        }
    }

    fn lookup(&self, id: &str) -> Option<String> {
        if self.keywords.contains(&id) {
            return Some(id.into());
        }
        None
    }

    fn matched(&mut self, c: char) -> bool {
        self.peek_char() == c
    }

    fn next_char(&mut self) -> Option<char> {
        self.previous = self.current;
        self.current = self.stream.next();
        if self.current.is_some() {
            self.advance();
        }
        self.current
    }

    fn next_if(&mut self, func: impl FnOnce(&char) -> bool) -> Option<char> {
        match (self.current, self.stream.next_if(func)) {
            (Some(current), next @ Some(_)) => {
                self.previous = Some(current);
                self.current = next;
                self.advance();
            }
            (_, next @ Some(_)) => self.current = next,
            _ => return None,
        }
        self.current
    }

    fn peek_char(&mut self) -> char {
        *self.stream.peek().unwrap_or(&'\0')
    }

    fn reset_span(&mut self) {
        self.span.start = self.span.end;
    }

    fn span(&mut self) -> Span {
        let span = self.span;
        self.reset_span();
        span
    }

    fn number(&mut self) -> Token {
        let mut number = self.current.unwrap().to_string();
        while let Some(ch) =
            self.next_if(|c| c.is_ascii_digit() || c == &'_' || c == &'.')
        {
            number.push(ch);
        }
        let span = self.span();
        if number.contains('.') {
            Token::Float(number, span)
        } else {
            Token::Int(number, span)
        }
    }

    fn id(&mut self) -> Token {
        let mut ident = self.current.unwrap().to_string();
        while let Some(ch) = self.next_if(|c| c.is_ascii_alphanumeric() || c == &'_') {
            ident.push(ch);
        }
        let span = self.span();
        self.lookup(&ident)
            .map_or(Token::Id(ident, span), |i| Token::KeyWord(i.into(), span))
    }

    fn line_comment(&mut self) -> Option<Token> {
        while self.next_if(|c| c != &'\n').is_some() {}
        self.next()
    }

    fn string(&mut self) -> Option<Token> {
        let mut string = String::new();
        while let Some(ch) = self.next_if(|c| c != &'"') {
            string.push(ch);
        }
        self.next_char();
        let string = string.replace("\\n", "\n");
        Some(Token::String(string, self.span()))
    }

    fn chr(&mut self) -> Option<Token> {
        let mut c = String::new();
        while let Some(ch) = self.next_if(|c| c != &'\'') {
            c.push(ch);
        }
        self.next_char();
        Some(Token::Char(c, self.span()))
    }

    fn op_token(&mut self, op: &str) -> Token {
        for _ in 0..op.chars().count().saturating_sub(1) {
            self.next_char();
        }
        Token::Op(op.into(), self.span())
    }

    fn err(&mut self, c: char) -> Option<Token> {
        Some(Token::Error(c.to_string(), self.span()))
    }

    fn is_comment(&mut self, c: char) -> bool {
        if c != self.line_comment.0 {
            return false;
        }
        let Some(nc) = self.line_comment.1 else {
            return true;
        };
        if self.peek_char() != nc {
            return false;
        }
        true
    }
}

impl<'a> Iterator for Scanner<'a> {
    type Item = Token;
    fn next(&mut self) -> Option<Self::Item> {
        let Some(ch) = self.next_char() else {
            return None;
        };
        // comment (char, Option<char>)
        match ch {
            num if num.is_ascii_digit() => Some(self.number()),
            ident if ident.is_ascii_alphabetic() => Some(self.id()),
            c if self.is_comment(c) => self.line_comment(),
            // '-' if self.matched('-') => self.line_comment(),
            '-' if self.matched('>') => Some(self.op_token("->")),
            '=' if self.matched('>') => Some(self.op_token("=>")),
            '<' if self.matched('-') => Some(self.op_token("<-")),
            '<' if self.matched('=') => Some(self.op_token("<=")),
            '>' if self.matched('=') => Some(self.op_token(">=")),
            '=' if self.matched('=') => Some(self.op_token("==")),
            '!' if self.matched('=') => Some(self.op_token("!=")),
            ':' if self.matched(':') => Some(self.op_token("::")),
            '|' if self.matched('>') => Some(self.op_token("|>")),
            '"' => self.string(),
            '\'' => self.chr(),
            '\\' => Some(self.op_token("\\")),
            '%' => Some(self.op_token("%")),
            '.' => Some(self.op_token(".")),
            '|' => Some(self.op_token("|")),
            '!' => Some(self.op_token("!")),
            '<' => Some(self.op_token("<")),
            '>' => Some(self.op_token(">")),
            '+' => Some(self.op_token("+")),
            '-' => Some(self.op_token("-")),
            '=' => Some(self.op_token("=")),
            '*' => Some(self.op_token("*")),
            '/' => Some(self.op_token("/")),
            ':' => Some(self.op_token(":")),
            ';' => Some(self.op_token(";")),
            ',' => Some(self.op_token(",")),
            '(' => Some(self.op_token("(")),
            ')' => Some(self.op_token(")")),
            '[' => Some(self.op_token("[")),
            ']' => Some(self.op_token("]")),
            '{' => Some(self.op_token("{")),
            '}' => Some(self.op_token("}")),
            'λ' => Some(self.op_token("λ")),
            ' ' | '\n' => {
                self.reset_span();
                self.next()
            }
            c => self.err(c),
        }
    }
}
