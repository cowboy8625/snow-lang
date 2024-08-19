use anyhow::Result;

#[derive(Debug, Clone, PartialEq)]
pub struct Start {
    length: u32,
    index: u32,
}

impl Start {
    pub const ID: u8 = 0x08;
    pub fn new(index: u32) -> Self {
        Self { length: 1, index }
    }

    pub fn to_bytes(&self) -> Result<Vec<u8>> {
        let mut bytes = vec![Self::ID];
        leb128::write::unsigned(&mut bytes, self.length as u64)?;
        leb128::write::unsigned(&mut bytes, self.index as u64)?;
        Ok(bytes)
    }
}
