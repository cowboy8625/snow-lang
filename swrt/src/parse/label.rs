use super::Error;
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Label(pub String);

impl FromStr for Label {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            return Err(Error::Unexpected("label_name:".into(), s.into()));
        }
        let ":" = &s[s.len()-1..] else {
            return Err(Error::Unexpected(":".into(), s.into()));
        };

        let mut chars = s[..s.len() - 1].chars();
        let Some(c) = chars.next() else {
            return Err(Error::Unexpected("label has to more then just a ':'".into(), s.into()));
        };
        if !(c.is_ascii_alphabetic() || c == '_') {
            return Err(Error::Unexpected(
                format!("unsupported chars '{c}' in label name"),
                s.into(),
            ));
        }
        for c in chars {
            if !(c.is_ascii_alphanumeric() || c == '_') {
                return Err(Error::Unexpected(
                    format!("unsupported chars '{c}' in label name"),
                    s.into(),
                ));
            }
        }
        Ok(Self(s[..s.len() - 1].to_string()))
    }
}
