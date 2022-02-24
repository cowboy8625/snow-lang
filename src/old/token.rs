use std::fmt;
use std::ops::Range;

pub type Span = Range<usize>;
pub type Spanned<T> = (T, Span);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum KeyWord {
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
    pub fn lookup(name: &str) -> Option<Self> {
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

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BlockType {
    Fn,
    Do,
    Let,
    Arg,
    Pram,
    Paren,
    FnBlock,
}

impl fmt::Display for BlockType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Fn => write!(f, "Fn"),
            Self::Do => write!(f, "Do"),
            Self::Let => write!(f, "Let"),
            Self::Arg => write!(f, "Arg"),
            Self::Pram => write!(f, "Pram"),
            Self::Paren => write!(f, "Paren"),
            Self::FnBlock => write!(f, "FnBlock"),
        }
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Token {
    Int(String),
    Float(String),
    String(String),
    Char(char),
    Op(String),
    Ctrl(char),
    KeyWord(KeyWord),
    Id(String),
    OpenBlock(BlockType),
    CloseBlock(BlockType),
    Error(String),
    Eof,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Int(i) => write!(f, "Int({})", i),
            Self::Float(i) => write!(f, "Float({})", i),
            Self::String(i) => write!(f, "String({})", i),
            Self::Char(i) => write!(f, "Char({})", i),
            Self::Op(i) => write!(f, "Op({})", i),
            Self::Ctrl(i) => write!(f, "Ctrl({})", i),
            Self::KeyWord(i) => write!(f, "KeyWord({})", i),
            Self::Id(i) => write!(f, "Id({})", i),
            Self::OpenBlock(i) => write!(f, "OpenBlock({})", i),
            Self::CloseBlock(i) => write!(f, "CloseBlock({})", i),
            Self::Error(i) => write!(f, "Error({})", i),
            Self::Eof => write!(f, "EOF"),
        }
    }
}
