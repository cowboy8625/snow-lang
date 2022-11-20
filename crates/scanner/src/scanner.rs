use super::{Span, Token};
use std::{iter::Peekable, str::Chars};
type Stream<'a> = Peekable<Chars<'a>>;

#[derive(Debug, Clone)]
pub struct Scanner<'a> {
    stream: Stream<'a>,
    pos: usize,
    current: Option<char>,
    previous: Option<char>,
}

impl<'a> Scanner<'a> {
    pub fn new(src: &'a str) -> Self {
        Self {
            stream: src.chars().peekable(),
            pos: 0,
            current: None,
            previous: None,
        }
    }
    fn advance(&mut self) {
        self.pos += 1;
    }

    fn next_char(&mut self) -> Option<char> {
        self.previous = self.current;
        self.current = self.stream.next();
        self.advance();
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

    fn span(&self, start: usize) -> Span {
        start - 1..self.pos
    }

    fn span_one(&self) -> Span {
        self.pos - 1..self.pos
    }

    fn number(&mut self) -> Option<(Token, Span)> {
        let mut number = self.current.unwrap().to_string();
        let start = self.pos;
        while let Some(ch) = self.next_if(|c| c.is_ascii_digit() || c == &'_' || c == &'.') {
            number.push(ch);
        }
        let span = self.span(start);
        let token = if number.contains('.') {
            Token::Float(number)
        } else {
            Token::Int(number)
        };
        Some((token, span))
    }

    fn id(&mut self) -> Option<(Token, Span)> {
        let mut ident = self.current.unwrap().to_string();
        let start = self.pos;
        while let Some(ch) = self.next_if(|c| c.is_ascii_alphanumeric() || c == &'_') {
            ident.push(ch);
        }
        let span = self.span(start);
        let token = Token::lookup(&ident).map_or(Token::Id(ident), |i| i);
        Some((token, span))
    }

    fn rarrow(&mut self) -> Option<(Token, Span)> {
        let start = self.pos;
        let _ = self.next_char();
        Some((Token::Op("->".into()), self.span(start)))
    }
    fn larrow(&mut self) -> Option<(Token, Span)> {
        let start = self.pos;
        let _ = self.next_char();
        Some((Token::Op("<-".into()), self.span(start)))
    }
    fn fatrarrow(&mut self) -> Option<(Token, Span)> {
        let start = self.pos;
        let _ = self.next_char();
        Some((Token::Op("=>".into()), self.span(start)))
    }

    fn line_comment(&mut self) -> Option<(Token, Span)> {
        while let Some(_) = self.next_if(|c| c != &'\n') {}
        self.next()
    }
}

impl<'a> Iterator for Scanner<'a> {
    type Item = (Token, Span);
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(ch) = self.next_char() {
            match ch {
                num if num.is_ascii_digit() => return self.number(),
                ident if ident.is_ascii_alphabetic() => return self.id(),
                '-' if self.peek_char() == '-' => return self.line_comment(),
                '-' if self.peek_char() == '>' => return self.rarrow(),
                '=' if self.peek_char() == '>' => return self.fatrarrow(),
                '<' if self.peek_char() == '-' => return self.larrow(),
                '<' if self.peek_char() == '=' => {
                    let start = self.pos;
                    let _ = self.next_char();
                    return Some((Token::Op("<=".into()), self.span(start)));
                }
                '>' if self.peek_char() == '=' => {
                    let start = self.pos;
                    let _ = self.next_char();
                    return Some((Token::Op(">=".into()), self.span(start)));
                }
                '=' if self.peek_char() == '=' => {
                    let start = self.pos;
                    let _ = self.next_char();
                    return Some((Token::Op("==".into()), self.span(start)));
                }
                '!' if self.peek_char() == '=' => {
                    let start = self.pos;
                    let _ = self.next_char();
                    return Some((Token::Op("!=".into()), self.span(start)));
                }
                ':' if self.peek_char() == ':' => {
                    let start = self.pos;
                    let _ = self.next_char();
                    return Some((Token::Op("::".into()), self.span(start)));
                }
                '|' if self.peek_char() == '>' => {
                    let start = self.pos;
                    let _ = self.next_char();
                    return Some((Token::Op("|>".into()), self.span(start)));
                }
                '|' => return Some((Token::Op("|".into()), self.span_one())),
                '!' => return Some((Token::Op("!".into()), self.span_one())),
                '<' => return Some((Token::Op("<".into()), self.span_one())),
                '>' => return Some((Token::Op(">".into()), self.span_one())),
                '+' => return Some((Token::Op("+".into()), self.span_one())),
                '-' => return Some((Token::Op("-".into()), self.span_one())),
                '=' => return Some((Token::Op("=".into()), self.span_one())),
                '*' => return Some((Token::Op("*".into()), self.span_one())),
                '/' => return Some((Token::Op("/".into()), self.span_one())),
                ':' => return Some((Token::Op(":".into()), self.span_one())),
                ';' => return Some((Token::Op(";".into()), self.span_one())),
                ',' => return Some((Token::Op(",".into()), self.span_one())),
                '(' => return Some((Token::Op("(".into()), self.span_one())),
                ')' => return Some((Token::Op(")".into()), self.span_one())),
                '[' => return Some((Token::Op("[".into()), self.span_one())),
                ']' => return Some((Token::Op("]".into()), self.span_one())),
                '{' => return Some((Token::Op("{".into()), self.span_one())),
                '}' => return Some((Token::Op("}".into()), self.span_one())),
                'λ' => return Some((Token::Op("λ".into()), self.span_one())),
                ' ' | '\n' => return self.next(),
                c => panic!("Unknown char: {c}"),
            }
        }
        None
    }
}
