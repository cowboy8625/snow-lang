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

impl From<Token> for Precedence {
    fn from(token: Token) -> Self {
        match token {
            Token::Int(_) | Token::Float(_) | Token::String(_) | Token::Char(_) | Token::Id(_) => {
                Self::Primary
            }
            Token::KeyWord(ref b) if b == "true" || b == "false" => Self::Primary,
            Token::KeyWord(_) => Self::None,
            Token::Eof => Self::None,
            Token::Op(ref op) => match op.as_str() {
                "+" | "-" => Precedence::Term,
                "*" | "/" => Precedence::Factor,
                ">" | "<" | ">=" | "<=" => Precedence::Comparison,
                "==" | "!=" => Precedence::Equality,
                "=" => Precedence::Assignment,
                "|>" => Precedence::Pipe,
                _ => Precedence::None,
            },
        }
    }
}
