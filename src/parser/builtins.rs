use super::{KeyWord, Parser, Spanned, Token};
use std::fmt;
pub(crate) fn builtin<'a>() -> impl Parser<'a, Token, Spanned<BuiltIn>> {
    move |input: &'a [Spanned<Token>]| match &input.get(0) {
        Some(node) => match node.node {
            Token::Op("+") => Ok((&input[1..], (BuiltIn::Plus, input[0].span()).into())),
            Token::Op("-") => Ok((&input[1..], (BuiltIn::Mins, input[0].span()).into())),
            Token::Op("*") => Ok((&input[1..], (BuiltIn::Mult, input[0].span()).into())),
            Token::Op("/") => Ok((&input[1..], (BuiltIn::Div, input[0].span()).into())),
            Token::Op("==") => Ok((&input[1..], (BuiltIn::Eq, input[0].span()).into())),
            Token::Op("!=") => Ok((&input[1..], (BuiltIn::NEq, input[0].span()).into())),
            Token::Op("!") => Ok((&input[1..], (BuiltIn::Not, input[0].span()).into())),
            Token::KeyWord(kw) => match kw {
                KeyWord::Print => Ok((&input[1..], (BuiltIn::Print, input[0].span()).into())),
                KeyWord::PrintLn => Ok((&input[1..], (BuiltIn::PrintLn, input[0].span()).into())),
                KeyWord::DbgInt => Ok((&input[1..], (BuiltIn::DbgInt, input[0].span()).into())),
                _ => Err(input),
            },
            _ => Err(input),
        },
        _ => Err(input),
    }
}

#[derive(Debug, PartialEq, Clone, Copy, Eq)]
pub enum BuiltIn {
    Plus,
    Mins,
    Mult,
    Div,
    Eq,
    NEq,
    Not,
    Print,
    PrintLn,
    DbgInt,
}

impl BuiltIn {
    pub fn display(&self) -> String {
        match self {
            Self::Plus => "+".into(),
            Self::Mins => "-".into(),
            Self::Mult => "*".into(),
            Self::Div => "/".into(),
            Self::Eq => "==".into(),
            Self::NEq => "/=".into(),
            Self::Not => "not".into(),
            Self::Print => "putStr".into(),
            Self::PrintLn => "putStr".into(),
            Self::DbgInt => "putStr".into(),
        }
    }
}

impl fmt::Display for BuiltIn {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Plus => write!(f, "<+>"),
            Self::Mins => write!(f, "<->"),
            Self::Mult => write!(f, "<*>"),
            Self::Div => write!(f, "</>"),
            Self::Eq => write!(f, "<==>"),
            Self::NEq => write!(f, "<!=>"),
            Self::Not => write!(f, "<!>"),
            Self::Print => write!(f, "<Print>"),
            Self::PrintLn => write!(f, "<Println>"),
            Self::DbgInt => write!(f, "<DbgInt>"),
        }
    }
}
