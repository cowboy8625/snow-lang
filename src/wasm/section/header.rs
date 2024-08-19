use anyhow::Result;

#[derive(Debug, Clone, PartialEq)]
pub struct Header {
    pub magic_number: [u8; 4],
    pub version: [u8; 4],
}

impl Header {
    pub const MAGIC_NUMBER: [u8; 4] = [0x00, 0x61, 0x73, 0x6D];
    pub const VERSION: [u8; 4] = [0x01, 0x00, 0x00, 0x00];

    pub fn to_bytes(&self) -> Result<Vec<u8>> {
        let mut bytes = Vec::new();
        bytes.extend(&self.magic_number);
        bytes.extend(&self.version);
        Ok(bytes)
    }
}

impl Default for Header {
    fn default() -> Self {
        Self {
            magic_number: Header::MAGIC_NUMBER,
            version: Header::VERSION,
        }
    }
}
