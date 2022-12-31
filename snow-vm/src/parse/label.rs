use std::str::FromStr;
use super::UnrecognizedTokenOpError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Label(pub String);

impl FromStr for Label {
    type Err = UnrecognizedTokenOpError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            return Err(UnrecognizedTokenOpError(s.into()));
        }
        let ":" = &s[s.len()-1..] else {
            return Err(UnrecognizedTokenOpError(s.into()));
        };

        let mut chars = s[..s.len() - 1].chars();
        let Some(c) = chars.next() else {
            return Err(UnrecognizedTokenOpError(s.into()));
        };
        if !(c.is_ascii_alphabetic() || c == '_') {
            return Err(UnrecognizedTokenOpError(s.into()));
        }
        for c in chars {
            if !(c.is_ascii_alphanumeric() || c == '_') {
                return Err(UnrecognizedTokenOpError(s.into()));
            }
        }
        Ok(Self(s[..s.len() - 1].to_string()))
    }
}

