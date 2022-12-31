use std::str::FromStr;
use super::{Directive, Error, Label};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Data {
    pub name: String,
    pub directive: Directive,
}

impl FromStr for Data {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (head, tail) = s.split_once(' ').unwrap_or((s, ""));
        let Label(name) = head
            .parse::<Label>()
            .map_err(|_| Error::Unexpected(head.into(), "damn label".into()))?;
        let directive = tail.parse::<Directive>()?;
        Ok(Self { name, directive })
    }
}
