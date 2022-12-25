use super::Token;
#[allow(unused)]
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
pub enum Precedence {
    None,
    Primary,
    Term,       // + -
    Factor,     // * /
    Equality,   // == !=
    Comparison, // < > <= >=
    Assignment, // =
    Or,         // or
    And,        // and
    Pipe,       // |>
    Call,       // . ()
    Fn,         // fn function declaration
    Unary,      // ! -
}

impl TryFrom<Token> for Precedence {
    type Error = String;
    fn try_from(value: Token) -> Result<Self, Self::Error> {
        match value {
            Token::Int(..)
            | Token::Float(..)
            | Token::String(..)
            | Token::Char(..)
            | Token::Id(..) => Ok(Self::Primary),
            Token::KeyWord(ref b, ..) if b == "true" || b == "false" => Ok(Self::Primary),
            Token::KeyWord(..) => Ok(Self::None),
            Token::Eof(..) => Ok(Self::None),
            Token::Op(ref op, ..) => match op.as_str() {
                "+" | "-" => Ok(Precedence::Term),
                "*" | "/" => Ok(Precedence::Factor),
                ">" | "<" | ">=" | "<=" => Ok(Precedence::Comparison),
                "==" | "!=" => Ok(Precedence::Equality),
                "=" => Ok(Precedence::Assignment),
                "|>" => Ok(Precedence::Pipe),
                _ => Ok(Precedence::None),
            },
            Token::Error(c, ..) => Err(format!("Unknown char {c}")),
        }
    }
}
