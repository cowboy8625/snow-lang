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
                let mut b = vec![];
                let mut chars = string.chars().peekable();
                // FIXME: Not a so happy about this.
                while let Some(c) = chars.next() {
                    match c {
                        '\\' if chars.peek() == Some(&'n') => {
                            chars.next();
                            b.push(10);
                        }
                        _ => b.push(c as u8),
                    }
                }
                b.push(0);
                b
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
                let Some(item) = tail.trim()
                    .strip_prefix('\"').and_then(|s| s.strip_suffix("\"")) else {
                    return Err(Error::DirectiveFormat(tail.into()));
                };
                Ok(Self::Ascii(item.into()))
            }
            _ => Err(Error::DirectiveFormat(tail.into())),
        }
    }
}
