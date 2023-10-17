use crate::token::TokenPosition;

use super::{Char, Ctrl, Error, Float, Ident, Int, KeyWord, Op, Span, Str, Token};
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
        let mut lexme = c.to_string();
        while let Some(c) = self.next_char_if(|c| c.is_ascii_digit() || c == '_') {
            lexme.push(c);
        }
        let span = self.span();
        let pos = self.get_token_position();
        if lexme.contains('.') {
            return Some(Token::Float(Float { lexme, pos, span }));
        }
        Some(Token::Int(Int { lexme, pos, span }))
    }

    fn ident(&mut self, c: char) -> Option<Token> {
        let mut lexme = c.to_string();
        while let Some(c) = self.next_char_if(|c| c.is_ascii_alphanumeric() || c == '_') {
            lexme.push(c);
        }
        let span = self.span();
        let keywords = [
            "enum", "data", "type", "true", "false", "return", "let", "and", "or", "not",
            "if", "then", "else", "fn", "mod",
        ];
        let pos = self.get_token_position();
        if keywords.contains(&lexme.as_str()) {
            return Some(Token::KeyWord(KeyWord { lexme, pos, span }));
        }
        Some(Token::Ident(Ident { lexme, pos, span }))
    }

    fn string(&mut self) -> Option<Token> {
        let mut lexme = String::new();
        while let Some(c) = self.next_char_if(|c| c != '"') {
            lexme.push(c);
        }
        self.next_char();
        let lexme = lexme.replace("\\n", "\n");
        let pos = self.get_token_position();
        let span = self.span();
        Some(Token::Str(Str { lexme, pos, span }))
    }

    fn chr(&mut self) -> Option<Token> {
        let mut lexme = String::new();
        while let Some(c) = self.next_char_if(|c| c != '\'') {
            lexme.push(c);
        }
        self.next_char();
        let pos = self.get_token_position();
        let span = self.span();
        Some(Token::Char(Char { lexme, pos, span }))
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
        let lexme = op.to_string();
        let span = self.span();
        Some(tok(lexme, pos, span))
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
            '-' if self.matched('>') => {
                self.token("->", |lexme, pos, span| Token::Op(Op { lexme, pos, span }))
            }
            '>' if self.matched('=') => {
                self.token(">=", |lexme, pos, span| Token::Op(Op { lexme, pos, span }))
            }
            '<' if self.matched('=') => {
                self.token("<=", |lexme, pos, span| Token::Op(Op { lexme, pos, span }))
            }
            '=' if self.matched('=') => {
                self.token("==", |lexme, pos, span| Token::Op(Op { lexme, pos, span }))
            }
            '!' if self.matched('=') => {
                self.token("!=", |lexme, pos, span| Token::Op(Op { lexme, pos, span }))
            }
            ':' if self.matched(':') => self.token("::", |lexme, pos, span| {
                Token::Ctrl(Ctrl { lexme, pos, span })
            }),
            '|' => self.token("|", |lexme, pos, span| Token::Op(Op { lexme, pos, span })),
            '-' => self.token("-", |lexme, pos, span| Token::Op(Op { lexme, pos, span })),
            '+' => self.token("+", |lexme, pos, span| Token::Op(Op { lexme, pos, span })),
            '*' => self.token("*", |lexme, pos, span| Token::Op(Op { lexme, pos, span })),
            '/' => self.token("/", |lexme, pos, span| Token::Op(Op { lexme, pos, span })),
            '>' => self.token(">", |lexme, pos, span| Token::Op(Op { lexme, pos, span })),
            '<' => self.token("<", |lexme, pos, span| Token::Op(Op { lexme, pos, span })),
            '=' => self.token("=", |lexme, pos, span| Token::Op(Op { lexme, pos, span })),
            '!' => self.token("!", |lexme, pos, span| Token::Op(Op { lexme, pos, span })),
            '%' => self.token("%", |lexme, pos, span| Token::Op(Op { lexme, pos, span })),
            '.' => self.token(".", |lexme, pos, span| Token::Op(Op { lexme, pos, span })),
            ',' => self.token(",", |lexme, pos, span| {
                Token::Ctrl(Ctrl { lexme, pos, span })
            }),
            '(' => self.token("(", |lexme, pos, span| {
                Token::Ctrl(Ctrl { lexme, pos, span })
            }),
            ')' => self.token(")", |lexme, pos, span| {
                Token::Ctrl(Ctrl { lexme, pos, span })
            }),
            '{' => self.token("{", |lexme, pos, span| {
                Token::Ctrl(Ctrl { lexme, pos, span })
            }),
            '}' => self.token("}", |lexme, pos, span| {
                Token::Ctrl(Ctrl { lexme, pos, span })
            }),
            '[' => self.token("[", |lexme, pos, span| {
                Token::Ctrl(Ctrl { lexme, pos, span })
            }),
            ']' => self.token("]", |lexme, pos, span| {
                Token::Ctrl(Ctrl { lexme, pos, span })
            }),
            ':' => self.token(":", |lexme, pos, span| {
                Token::Ctrl(Ctrl { lexme, pos, span })
            }),
            ';' => self.token(";", |lexme, pos, span| {
                Token::Ctrl(Ctrl { lexme, pos, span })
            }),
            'λ' => self.token("λ", |lexme, pos, span| {
                Token::Ctrl(Ctrl { lexme, pos, span })
            }),
            '\\' => self.token("\\", |lexme, pos, span| {
                Token::Ctrl(Ctrl { lexme, pos, span })
            }),
            '\n' | '\r' | ' ' | '\0' => {
                self.last_char = ch;
                let Some(ch) = self.next_char() else {
                    return None;
                };
                self.span.reset(Some(self.last_chr_len));
                self.parse(ch)
            }
            _ => Some(Token::Error(Error {
                lexme: ch.to_string(),
                pos: self.get_token_position(),
                span: self.span(),
            })),
        }
    }
}

impl<'a> Iterator for Scanner<'a> {
    type Item = Token;
    fn next(&mut self) -> Option<Self::Item> {
        self.next_char().and_then(|c| self.parse(c))
    }
}
