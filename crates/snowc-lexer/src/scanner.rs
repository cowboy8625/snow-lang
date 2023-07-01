use super::{Span, Token};
use std::iter::Peekable;
use std::str::Chars;

#[derive(Debug, Clone)]
pub struct Scanner<'a> {
    src: Peekable<Chars<'a>>,
    span: Span,
    last_chr_len: usize,
}

impl<'a> Scanner<'a> {
    pub fn new(src: &'a str) -> Self {
        Self {
            src: src.chars().peekable(),
            span: Span::default(),
            last_chr_len: 0,
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
        if number.contains('.') {
            return Some(Token::Float(number, span));
        }
        Some(Token::Int(number, span))
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
        if keywords.contains(&id.as_str()) {
            return Some(Token::KeyWord(id, span));
        }
        Some(Token::Id(id, span))
    }

    fn string(&mut self) -> Option<Token> {
        let mut string = String::new();
        while let Some(c) = self.next_char_if(|c| c != '"') {
            string.push(c);
        }
        self.next_char();
        let string = string.replace("\\n", "\n");
        Some(Token::String(string, self.span()))
    }

    fn chr(&mut self) -> Option<Token> {
        let mut string = String::new();
        while let Some(c) = self.next_char_if(|c| c != '\'') {
            string.push(c);
        }
        self.next_char();
        Some(Token::Char(string, self.span()))
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
        F: FnOnce(String, Span) -> Token,
    {
        for _ in 0..op.chars().count().saturating_sub(self.last_chr_len) {
            self.next_char();
        }
        Some(tok(op.to_string(), self.span()))
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
                let Some(ch) = self.next_char() else {
                    return None;
                };
                self.span.reset(Some(self.last_chr_len));
                self.parse(ch)
            }
            _ => Some(Token::Error(ch.to_string(), self.span())),
        }
    }

    // pub fn lex(mut self) -> Result<Vec<Token>, Vec<String>> {
    //     let mut tokens = vec![];
    //     while let Some(ch) = self.next_char() {
    //         let Some(token) = self.parse(ch) else {
    //             break;
    //         };
    //         tokens.push(token);
    //     }
    //     Ok(tokens)
    // }
}
impl<'a> Iterator for Scanner<'a> {
    type Item = Token;
    fn next(&mut self) -> Option<Self::Item> {
        self.next_char().and_then(|c| self.parse(c))
    }
}

// use super::{Span, Token};
// use std::{iter::Peekable, str::Chars};
// type Stream<'a> = Peekable<Chars<'a>>;
//
// #[derive(Debug, Clone)]
// pub struct Scanner<'a> {
//     stream: Stream<'a>,
//     span: Span,
//     current: Option<char>,
//     previous: Option<char>,
//     keywords: Vec<&'a str>,
//     line_comment: (char, Option<char>),
// }
//
// impl<'a> Scanner<'a> {
//     pub fn new(src: &'a str) -> Self {
//         let keywords = vec![
//             "enum", "data", "type", "true", "false", "return", "let", "and", "or", "not",
//             "if", "then", "else", "fn", "mod",
//         ];
//         Self {
//             stream: src.chars().peekable(),
//             span: Span::default(),
//             current: None,
//             previous: None,
//             keywords,
//             line_comment: ('-', Some('-')),
//         }
//     }
//
//     fn advance(&mut self) {
//         match self.current {
//             Some('\n') => {
//                 self.span.new_line();
//             }
//             Some(c) => {
//                 self.span.end += c.to_string().as_bytes().len();
//             }
//             None => {
//                 let Some(c) = self.previous else {
//                     self.span.end += 1;
//                     return;
//                 };
//                 self.span.end += c.to_string().as_bytes().len();
//             }
//         }
//     }
//
//     fn lookup(&self, id: &str) -> Option<String> {
//         if self.keywords.contains(&id) {
//             return Some(id.into());
//         }
//         None
//     }
//
//     fn matched(&mut self, c: char) -> bool {
//         self.peek_char() == c
//     }
//
//     fn next_char(&mut self) -> Option<char> {
//         self.previous = self.current;
//         self.current = self.stream.next();
//         if self.current.is_some() {
//             self.advance();
//         }
//         self.current
//     }
//
//     fn next_if(&mut self, func: impl FnOnce(&char) -> bool) -> Option<char> {
//         match (self.current, self.stream.next_if(func)) {
//             (Some(current), next @ Some(_)) => {
//                 self.previous = Some(current);
//                 self.current = next;
//                 self.advance();
//             }
//             (_, next @ Some(_)) => self.current = next,
//             _ => return None,
//         }
//         self.current
//     }
//
//     fn peek_char(&mut self) -> char {
//         *self.stream.peek().unwrap_or(&'\0')
//     }
//
//     fn reset_span(&mut self) {
//         self.span.start = self.span.end;
//     }
//
//     fn span(&mut self) -> Span {
//         let span = self.span;
//         self.reset_span();
//         span
//     }
//
//     fn number(&mut self) -> Token {
//         let mut number = self.current.unwrap().to_string();
//         while let Some(ch) =
//             self.next_if(|c| c.is_ascii_digit() || c == &'_' || c == &'.')
//         {
//             number.push(ch);
//         }
//         let span = self.span();
//         if number.contains('.') {
//             Token::Float(number, span)
//         } else {
//             Token::Int(number, span)
//         }
//     }
//
//     fn id(&mut self) -> Token {
//         let mut ident = self.current.unwrap().to_string();
//         while let Some(ch) = self.next_if(|c| c.is_ascii_alphanumeric() || c == &'_') {
//             ident.push(ch);
//         }
//         let span = self.span();
//         self.lookup(&ident)
//             .map_or(Token::Id(ident, span), |i| Token::KeyWord(i, span))
//     }
//
//     fn line_comment(&mut self) -> Option<Token> {
//         while self.next_if(|c| c != &'\n').is_some() {}
//         self.next()
//     }
//
//     fn string(&mut self) -> Option<Token> {
//         let mut string = "\"".to_string();
//         while let Some(ch) = self.next_if(|c| c != &'"') {
//             string.push(ch);
//         }
//         let Some('"') = self.next_char() else {
//             return Some(Token::Error(string, self.span()));
//         };
//         string.push('"');
//         let string = string.replace("\\n", "\n");
//         Some(Token::String(string, self.span()))
//     }
//
//     fn chr(&mut self) -> Option<Token> {
//         let mut c = String::new();
//         while let Some(ch) = self.next_if(|c| c != &'\'') {
//             c.push(ch);
//         }
//         self.next_char();
//         Some(Token::Char(c, self.span()))
//     }
//
//     fn op_token(&mut self, op: &str) -> Token {
//         for _ in 0..op.chars().count().saturating_sub(1) {
//             self.next_char();
//         }
//         Token::Op(op.into(), self.span())
//     }
//
//     fn err(&mut self, c: char) -> Option<Token> {
//         Some(Token::Error(c.to_string(), self.span()))
//     }
//
//     fn is_comment(&mut self, c: char) -> bool {
//         if c != self.line_comment.0 {
//             return false;
//         }
//         let Some(nc) = self.line_comment.1 else {
//             return true;
//         };
//         if self.peek_char() != nc {
//             return false;
//         }
//         true
//     }
// }
//
// impl<'a> Iterator for Scanner<'a> {
//     type Item = Token;
//     fn next(&mut self) -> Option<Self::Item> {
//         let Some(ch) = self.next_char() else {
//             return None;
//         };
//         // comment (char, Option<char>)
//         match ch {
//             // '-' if self.peek_char().is_ascii_digit() => Some(self.number()),
//             num if num.is_ascii_digit() => Some(self.number()),
//             ident if ident.is_ascii_alphabetic() => Some(self.id()),
//             c if self.is_comment(c) => self.line_comment(),
//             // '-' if self.matched('-') => self.line_comment(),
//             '-' if self.matched('>') => Some(self.op_token("->")),
//             '=' if self.matched('>') => Some(self.op_token("=>")),
//             '<' if self.matched('|') => Some(self.op_token("<|")),
//             '<' if self.matched('-') => Some(self.op_token("<-")),
//             '<' if self.matched('=') => Some(self.op_token("<=")),
//             '>' if self.matched('=') => Some(self.op_token(">=")),
//             '=' if self.matched('=') => Some(self.op_token("==")),
//             '!' if self.matched('=') => Some(self.op_token("!=")),
//             ':' if self.matched(':') => Some(self.op_token("::")),
//             '|' if self.matched('>') => Some(self.op_token("|>")),
//             '"' => self.string(),
//             '\'' => self.chr(),
//             '\\' => Some(self.op_token("\\")),
//             '%' => Some(self.op_token("%")),
//             '.' => Some(self.op_token(".")),
//             '|' => Some(self.op_token("|")),
//             '!' => Some(self.op_token("!")),
//             '<' => Some(self.op_token("<")),
//             '>' => Some(self.op_token(">")),
//             '+' => Some(self.op_token("+")),
//             '-' => Some(self.op_token("-")),
//             '=' => Some(self.op_token("=")),
//             '*' => Some(self.op_token("*")),
//             '/' => Some(self.op_token("/")),
//             ':' => Some(self.op_token(":")),
//             ';' => Some(self.op_token(";")),
//             ',' => Some(self.op_token(",")),
//             '(' => Some(self.op_token("(")),
//             ')' => Some(self.op_token(")")),
//             '[' => Some(self.op_token("[")),
//             ']' => Some(self.op_token("]")),
//             '{' => Some(self.op_token("{")),
//             '}' => Some(self.op_token("}")),
//             'λ' => Some(self.op_token("λ")),
//             ' ' | '\n' => {
//                 self.reset_span();
//                 self.next()
//             }
//             c => self.err(c),
//         }
//     }
// }
