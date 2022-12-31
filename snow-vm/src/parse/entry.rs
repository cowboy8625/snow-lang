use std::str::FromStr;
use super::Error;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Entry {
    pub name: String,
}

impl Entry {
    pub fn len(&self) -> usize {
        let Self { name } = self;
        format!(".entry {name}\n").len()
    }
}

impl FromStr for Entry {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (head, tail) = s.split_once(' ').unwrap_or((s, ""));
        if head.trim() != ".entry" {
            return Err(Error::Unexpected(head.into(), ".entry".into()));
        }
        let (head, _tail) = tail.split_once('\n').unwrap_or((s, ""));
        Ok(Self {
            name: head.to_string(),
        })
    }
}

