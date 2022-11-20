use super::Token;
use std::fmt;

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
