use super::Error;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Reg(pub u8);

impl FromStr for Reg {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let Some("%") = &s.get(..1) else {
            return Err(Error::Unexpected("%".into(), s.into()));
        };
        let reg = &s[1..]
            .parse::<u8>()
            .map_err(|_| Error::Unexpected("<num>".into(), s.into()))?;
        Ok(Self(*reg))
    }
}
