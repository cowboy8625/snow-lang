use super::Error;
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Directive {
    Ascii(String),
}

impl Directive {
    pub fn into_bytes(self) -> Vec<u8> {
        match self {
            Self::Ascii(string) => {
                let mut bytes = string.into_bytes();
                bytes.push(0);
                bytes
            }
        }
    }
}

impl FromStr for Directive {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (head, tail) = s.split_once(' ').unwrap_or((s, ""));
        match head {
            ".ascii" => {
                let Some(name) = tail.trim()
                    .strip_prefix('\"').and_then(|s| s.strip_suffix("\"")) else {
                    return Err(Error::DirectiveFormat(tail.into()));
                };
                Ok(Self::Ascii(name.into()))
            }
            _ => Err(Error::DirectiveFormat(tail.into())),
        }
    }
}
