use super::position::{CharPos, Span, Spanned};
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
    KeyWord(KeyWord),
    InDent(usize),
    DeDent,
    Op(&'static str),
    Ctrl(char),
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Int(i) => write!(f, "Int({})", i),
            Self::Float(i) => write!(f, "Float({})", i),
            Self::String(i) => write!(f, "String({})", i),
            Self::Id(i) => write!(f, "Id({})", i),
            Self::KeyWord(i) => write!(f, "KeyWord({})", i),
            Self::InDent(i) => write!(f, "InDent({})", i),
            Self::DeDent => write!(f, "DeDent"),
            Self::Op(i) => write!(f, "Op({})", i),
            Self::Ctrl(i) => write!(f, "Ctrl({})", i),
        }
    }
}

// impl From<(Token, Span)> for Spanned<Token> {
//     fn from((node, span): (Token, Span)) -> Self {
//         Self { node, span }
//     }
// }

struct Scanner<'a> {
    stream: Stream<'a, CharPos>,
    tokens: Vec<Spanned<Token>>,
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
                '-' if self.peek_char() == '-' => self.line_comment(),
                '{' if self.peek_char() == '-' => self.block_comment(),
                '\n' if self.peek_char() == ' ' => self.indent(),
                '\n' if self.peek_char().is_ascii_alphabetic() => self.dedent(cp),
                '=' if self.peek_char() == '=' => {
                    let end = self.stream.next().unwrap();
                    self.push((Token::Op("=="), (cp, end).into()))
                }
                '!' if self.peek_char() == '=' => {
                    let end = self.stream.next().unwrap();
                    self.push((Token::Op("!="), (cp, end).into()))
                }
                '>' if self.peek_char() == '=' => {
                    let end = self.stream.next().unwrap();
                    self.push((Token::Op(">="), (cp, end).into()))
                }
                '<' if self.peek_char() == '=' => {
                    let end = self.stream.next().unwrap();
                    self.push((Token::Op("<="), (cp, end).into()))
                }
                ':' if self.peek_char() == ':' => {
                    let end = self.stream.next().unwrap();
                    self.push((Token::Op("::"), (cp, end).into()))
                }
                '!' => self.push((Token::Op("!"), (cp, cp).into())),
                '=' => self.push((Token::Op("="), (cp, cp).into())),
                '>' => self.push((Token::Op(">"), (cp, cp).into())),
                '<' => self.push((Token::Op("<"), (cp, cp).into())),
                '-' => self.push((Token::Op("-"), (cp, cp).into())),
                '+' => self.push((Token::Op("+"), (cp, cp).into())),
                '*' => self.push((Token::Op("*"), (cp, cp).into())),
                '/' => self.push((Token::Op("/"), (cp, cp).into())),
                '(' => self.push((Token::Ctrl('('), (cp, cp).into())),
                ')' => self.push((Token::Ctrl(')'), (cp, cp).into())),
                '[' => self.push((Token::Ctrl('['), (cp, cp).into())),
                ']' => self.push((Token::Ctrl(']'), (cp, cp).into())),
                '{' => self.push((Token::Ctrl('{'), (cp, cp).into())),
                '}' => self.push((Token::Ctrl('}'), (cp, cp).into())),
                ',' => self.push((Token::Ctrl(','), (cp, cp).into())),
                '"' => self.string(cp),
                id if id.is_ascii_alphabetic() => self.identifier(cp),
                num if num.is_numeric() => self.number(cp),
                ' ' => {}
                '\n' => {}
                _ => {
                    println!("{:?}:{}:{}:{}", cp.chr, cp.idx, cp.row, cp.col);
                    self.error(cp.chr);
                }
            }
        }
        self
    }

    fn push<T>(&mut self, spanned: T)
    where
        T: Into<Spanned<Token>>,
    {
        self.tokens.push(spanned.into());
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
        while let Some(_) = self.stream.next_if(|cp| cp.chr != '\n') {}
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
        let mut count = 1;
        let start = self.stream.next().unwrap();
        let mut end = start;
        while let Some(cp) = self.stream.next_if(|&cp| cp.chr == ' ') {
            end = cp;
            count += 1;
        }
        let span: Span = (start, end).into();
        let token = (Token::InDent(count), span);
        self.push(token);
    }

    fn dedent(&mut self, start: &CharPos) {
        let span: Span = (start, start).into();
        let token = (Token::DeDent, span);
        self.push(token);
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

    fn identifier(&mut self, start: &CharPos) {
        let mut idt = start.chr.to_string();
        let mut end = start;
        while let Some(cp) = self.stream.next_if(|&cp| cp.chr.is_ascii_alphanumeric()) {
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
        while let Some(cp) = self.stream.next_if(|cp| cp.chr != '"') {
            string.push(cp.chr);
        }
        let end = self.stream.next().unwrap();
        let span: Span = (start, end).into();
        self.push((
            Token::String(
                string
                    // TODO: There are more to cover
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

// FIXME: This does not need to clone.
pub fn scanner(
    filename: &str,
    src: &str,
) -> Result<Vec<Spanned<Token>>, (Vec<Spanned<Token>>, Vec<String>)> {
    // NOTE: We insert a '\n' at the begening of a file
    // do to how functions are parsed.  The `Token::DeDent`
    // is triggered when a `\n` is followed by a 'Alphabetic` `char`.
    //
    // EXAMPLE: "main = print (+ 1 100)"
    //
    // This would created a error in the parse.
    //
    let src = if src
        // Some times we dont want to add a '\n' if we are using a shell.
        .chars()
        .nth(0)
        .map(|c| c.is_ascii_alphabetic())
        .unwrap_or(false)
    {
        format!("\n{}", src)
    } else {
        src.to_string()
    };
    let chrpos = pos_enum(filename, &src);
    let scanner = Scanner::new(&chrpos).scan();
    if !scanner.errors.is_empty() {
        return Err((scanner.tokens.clone(), scanner.errors.clone()));
    }
    Ok(scanner.tokens.clone())
}
