use anyhow::Result;

#[derive(Debug)]
pub struct WasmWalker<'a> {
    pub bytes: &'a [u8],
    index: usize,
}

impl<'a> WasmWalker<'a> {
    pub fn new(bytes: &'a [u8]) -> Self {
        Self { bytes, index: 0 }
    }

    // pub fn peek(&mut self) -> Option<&u8> {
    //     self.bytes.get(self.index)
    // }

    pub fn next(&mut self) -> Option<u8> {
        let byte = self.bytes.get(self.index)?;
        self.index += 1;
        Some(*byte)
    }

    fn has_more(byte: &u8) -> bool {
        (byte & 0b1000_0000) != 0
    }

    pub fn leb128_bytes(&self) -> Vec<u8> {
        let mut i = self.index;
        let mut bytes = Vec::new();
        while let Some(byte) = self.bytes.get(i) {
            bytes.push(*byte);
            if !Self::has_more(byte) {
                break;
            }
            i += 1;
        }
        bytes
    }

    pub fn take(&mut self, length: usize) -> Vec<u8> {
        let mut bytes = Vec::new();
        let mut count = 0;
        while let Some(byte) = self.bytes.get(self.index) {
            if count == length {
                break;
            }
            bytes.push(*byte);
            count += 1;
            self.index += 1;
        }
        bytes
    }

    pub fn get_section(&mut self) -> Result<Vec<u8>> {
        let length_bytes = self.leb128_bytes();
        let _ = self.take(length_bytes.len());
        let length =
            leb128::read::unsigned(&mut std::io::Cursor::new(length_bytes))? as usize;
        let bytes = self.take(length);
        Ok(bytes)
    }
}
