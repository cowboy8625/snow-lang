use anyhow::Result;

#[derive(Debug, Default, Clone)]
pub struct Memory {
    length: usize,
    pages: Vec<Page>,
}

impl Memory {
    const ID: u8 = 0x05;

    pub fn push(&mut self, page: Page) {
        self.pages.push(page);
    }

    pub fn with(mut self, page: Page) -> Self {
        self.pages.push(page);
        self
    }

    pub fn to_bytes(&self) -> Result<Vec<u8>> {
        let mut bytes = Vec::new();
        bytes.push(Memory::ID);
        // Add 1 for the count;
        let mut length = self.length + 1;
        let mut compiled_pages = Vec::new();
        for page in &self.pages {
            let page_bytes = page.to_bytes()?;
            compiled_pages.extend(page_bytes);
        }

        length += compiled_pages.len();
        leb128::write::unsigned(&mut bytes, length as u64)?;
        leb128::write::unsigned(&mut bytes, self.pages.len() as u64)?;
        bytes.extend(compiled_pages);
        Ok(bytes)
    }
}

#[derive(Debug, Clone)]
pub enum Page {
    WithMinAndMax(u32, u32),
    WithNoMinimun(u32),
}

impl Page {
    fn to_bytes(&self) -> Result<Vec<u8>> {
        let mut bytes = Vec::new();
        match self {
            Self::WithMinAndMax(min, max) => {
                bytes.push(0x01);
                leb128::write::unsigned(&mut bytes, *min as u64)?;
                leb128::write::unsigned(&mut bytes, *max as u64)?;
            }
            Self::WithNoMinimun(min) => {
                bytes.push(0x00);
                leb128::write::unsigned(&mut bytes, *min as u64)?;
            }
        }
        Ok(bytes)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_memory() -> Result<()> {
        let mut section = Memory::default().with(Page::WithNoMinimun(1));
        let bytes = section.to_bytes()?;
        assert_eq!(bytes, vec![0x05, 0x03, 0x01, 0x00, 0x01]);
        Ok(())
    }
}
