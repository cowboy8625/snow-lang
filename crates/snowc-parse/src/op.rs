use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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
    LRPipe,
    RLPipe,
    And,
    Or,
    Mod,
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
            ">=" => Ok(Self::GrtEq),
            "<=" => Ok(Self::LesEq),
            "==" => Ok(Self::Eq),
            "!=" => Ok(Self::Neq),
            "not" | "!" => Ok(Self::Not),
            // "=" => Ok(Self::Equals),
            "|>" => Ok(Self::LRPipe),
            "<|" => Ok(Self::RLPipe),
            "and" | "&&" => Ok(Self::And),
            "or" | "||" => Ok(Self::Or),
            "mod" | "%" => Ok(Self::Mod),
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
            Self::LRPipe => write!(f, "|>"),
            Self::RLPipe => write!(f, "<|"),
            Self::And => write!(f, "and"),
            Self::Or => write!(f, "or"),
            Self::Mod => write!(f, "mod"),
        }
    }
}
