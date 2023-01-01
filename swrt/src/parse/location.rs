use super::Error;
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Location(pub String);

impl FromStr for Location {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            return Err(Error::Unexpected("<label name>".into(), s.into()));
        }
        let mut chars = s[..s.len() - 1].chars();
        let Some(c) = chars.next() else {
            return Err(Error::Unexpected("<label name>".into(), s.into()));
        };
        if !(c.is_ascii_alphabetic() || c == '_') {
            return Err(Error::Unexpected("<label name>".into(), s.into()));
        }
        for c in chars {
            if !(c.is_ascii_alphanumeric() || c == '_') {
                return Err(Error::Unexpected("<label name>".into(), s.into()));
            }
        }
        Ok(Self(s[..s.len()].to_string()))
    }
}
