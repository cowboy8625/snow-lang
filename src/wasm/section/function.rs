use anyhow::Result;

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Function {
    imported_functions: u64,
    functions: u64,
}

impl Function {
    const ID: u8 = 0x03;

    pub fn add_imported_function(&mut self) {
        self.imported_functions += 1;
    }

    pub fn with_function(mut self) -> Self {
        self.functions += 1;
        self
    }

    pub fn add_function(&mut self) {
        self.functions += 1;
    }

    pub fn to_bytes(&self) -> Result<Vec<u8>> {
        let mut bytes = Vec::new();
        bytes.push(Function::ID);
        // Add 1 for the count;
        let length = self.functions + 1;
        leb128::write::unsigned(&mut bytes, length)?;

        // Count
        leb128::write::unsigned(&mut bytes, self.functions)?;

        let start = self.imported_functions;
        let end = self.functions + start;
        for function_id in start..end {
            leb128::write::unsigned(&mut bytes, function_id)?;
        }
        Ok(bytes)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_function() -> Result<()> {
        let mut section = Function::default();
        section.add_function();
        section.add_function();
        let bytes = section.to_bytes()?;
        assert_eq!(bytes, vec![0x03, 0x03, 0x02, 0x00, 0x01]);
        Ok(())
    }
}
