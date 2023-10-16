use crate::token::TokenPosition;

use super::{Span, Token};
use std::iter::Peekable;
use std::str::Chars;

#[derive(Debug, Clone)]
pub struct Scanner<'a> {
    src: Peekable<Chars<'a>>,
    span: Span,
    last_chr_len: usize,
    last_char: char,
}

impl<'a> Scanner<'a> {
    pub fn new(src: &'a str) -> Self {
        Self {
            src: src.chars().peekable(),
            span: Span::default(),
            last_chr_len: 0,
            last_char: '\n',
        }
    }

    fn peek_char(&mut self) -> Option<&char> {
        self.src.peek()
    }

    fn next_char(&mut self) -> Option<char> {
        let Some(ch) = self.src.next() else {
            return None;
        };
        self.span.right_shift(ch);
        self.last_chr_len = ch.to_string().as_bytes().len();
        Some(ch)
    }

    fn next_char_if<F>(&mut self, func: F) -> Option<char>
    where
        F: FnOnce(char) -> bool,
    {
        let Some(c) = self.peek_char() else {
            return None;
        };
        if func(*c) {
            return self.next_char();
        }
        None
    }

    fn get_token_position(&mut self) -> TokenPosition {
        let next = self.peek_char().copied().unwrap_or('\n');
        match (self.last_char, next) {
            ('\n', '\n') => TokenPosition::FullSpan,
            ('\n', c) if c != '\n' => TokenPosition::Start,
            (c, '\n') if c != '\n' => TokenPosition::End,
            _ => TokenPosition::Middle,
        }
    }

    fn span(&mut self) -> Span {
        let span = self.span;
        self.span.reset(None);
        span
    }

    fn number(&mut self, c: char) -> Option<Token> {
        let mut number = c.to_string();
        while let Some(c) = self.next_char_if(|c| c.is_ascii_digit() || c == '_') {
            number.push(c);
        }
        let span = self.span();
        let pos = self.get_token_position();
        if number.contains('.') {
            return Some(Token::Float(number, pos, span));
        }
        Some(Token::Int(number, pos, span))
    }

    fn ident(&mut self, c: char) -> Option<Token> {
        let mut id = c.to_string();
        while let Some(c) = self.next_char_if(|c| c.is_ascii_alphanumeric() || c == '_') {
            id.push(c);
        }
        let span = self.span();
        let keywords = [
            "enum", "data", "type", "true", "false", "return", "let", "and", "or", "not",
            "if", "then", "else", "fn", "mod",
        ];
        let pos = self.get_token_position();
        if keywords.contains(&id.as_str()) {
            return Some(Token::KeyWord(id, pos, span));
        }
        Some(Token::Id(id, pos, span))
    }

    fn string(&mut self) -> Option<Token> {
        let mut string = String::new();
        while let Some(c) = self.next_char_if(|c| c != '"') {
            string.push(c);
        }
        self.next_char();
        let string = string.replace("\\n", "\n");
        let pos = self.get_token_position();
        Some(Token::String(string, pos, self.span()))
    }

    fn chr(&mut self) -> Option<Token> {
        let mut string = String::new();
        while let Some(c) = self.next_char_if(|c| c != '\'') {
            string.push(c);
        }
        self.next_char();
        let pos = self.get_token_position();
        Some(Token::Char(string, pos, self.span()))
    }

    fn take_while(&mut self, expected: char) {
        while self.next_char_if(|c| c != expected).is_some() {}
    }

    fn comment(&mut self) -> Option<Token> {
        self.take_while('\n');
        let Some(ch) = self.next_char() else {
            return None;
        };
        self.parse(ch)
    }

    fn token<F>(&mut self, op: &str, tok: F) -> Option<Token>
    where
        F: FnOnce(String, TokenPosition, Span) -> Token,
    {
        for _ in 0..op.chars().count().saturating_sub(self.last_chr_len) {
            self.next_char();
        }
        let pos = self.get_token_position();
        Some(tok(op.to_string(), pos, self.span()))
    }

    fn matched(&mut self, ch: char) -> bool {
        matches!(self.peek_char(), Some(c) if c == &ch)
    }

    fn parse(&mut self, ch: char) -> Option<Token> {
        match ch {
            n @ '0'..='9' => self.number(n),
            i @ ('a'..='z' | 'A'..='Z') => self.ident(i),
            '"' => self.string(),
            '\'' => self.chr(),
            '-' if self.matched('-') => self.comment(),
            '-' if self.matched('>') => self.token("->", Token::Op),
            '>' if self.matched('=') => self.token(">=", Token::Op),
            '<' if self.matched('=') => self.token("<=", Token::Op),
            '=' if self.matched('=') => self.token("==", Token::Op),
            '!' if self.matched('=') => self.token("!=", Token::Op),
            ':' if self.matched(':') => self.token("::", Token::Op),
            '|' => self.token("|", Token::Op),
            '-' => self.token("-", Token::Op),
            '+' => self.token("+", Token::Op),
            '*' => self.token("*", Token::Op),
            '/' => self.token("/", Token::Op),
            '>' => self.token(">", Token::Op),
            '<' => self.token("<", Token::Op),
            '=' => self.token("=", Token::Op),
            '!' => self.token("!", Token::Op),
            '%' => self.token("%", Token::Op),
            '.' => self.token(".", Token::Op),
            ',' => self.token(",", Token::Op),
            '(' => self.token("(", Token::Op),
            ')' => self.token(")", Token::Op),
            '{' => self.token("{", Token::Op),
            '}' => self.token("}", Token::Op),
            '[' => self.token("[", Token::Op),
            ']' => self.token("]", Token::Op),
            ':' => self.token(":", Token::Op),
            ';' => self.token(";", Token::Op),
            'λ' => self.token("λ", Token::Op),
            '\\' => self.token("\\", Token::Op),
            '\n' | '\r' | ' ' | '\0' => {
                self.last_char = ch;
                let Some(ch) = self.next_char() else {
                    return None;
                };
                self.span.reset(Some(self.last_chr_len));
                self.parse(ch)
            }
            _ => Some(Token::Error(ch.to_string(), self.span())),
        }
    }
}

impl<'a> Iterator for Scanner<'a> {
    type Item = Token;
    fn next(&mut self) -> Option<Self::Item> {
        self.next_char().and_then(|c| self.parse(c))
    }
}
