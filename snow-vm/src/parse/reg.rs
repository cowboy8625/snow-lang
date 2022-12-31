use std::str::FromStr;
use super::UnrecognizedTokenOpError;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Reg(pub u8);

impl FromStr for Reg {
    type Err = UnrecognizedTokenOpError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let "%" = &s[..1] else {
            return Err(UnrecognizedTokenOpError(s.into()));
        };
        let reg = &s[1..]
            .parse::<u8>()
            .map_err(|_| UnrecognizedTokenOpError(s.into()))?;
        Ok(Self(*reg))
    }
}
