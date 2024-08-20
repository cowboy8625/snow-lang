use anyhow::Result;

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Function {
    imported_functions: Vec<String>,
    functions: Vec<String>,
}

impl Function {
    const ID: u8 = 0x03;

    pub fn add_imported_function(&mut self, name: impl Into<String>) {
        self.imported_functions.push(name.into());
    }

    pub fn with_function(mut self, name: impl Into<String>) -> Self {
        self.functions.push(name.into());
        self
    }

    pub fn add_function(&mut self, name: impl Into<String>) {
        self.functions.push(name.into());
    }

    pub fn get_id(&self, name: impl Into<String>) -> Option<u32> {
        let name = name.into();
        let import_id = self
            .imported_functions
            .iter()
            .position(|function| function == &name)
            .map(|id| id as u32);

        if import_id.is_some() {
            return import_id;
        }

        let imports = self.imported_functions.len();
        self.functions
            .iter()
            .position(|function| function == &name)
            .map(|id| (id + imports) as u32)
    }

    pub fn to_bytes(&self) -> Result<Vec<u8>> {
        let mut bytes = Vec::new();
        bytes.push(Function::ID);
        // Add 1 for the count;
        let length = self.functions.len() as u64 + 1;
        leb128::write::unsigned(&mut bytes, length)?;

        // Count
        leb128::write::unsigned(&mut bytes, self.functions.len() as u64)?;

        let start = self.imported_functions.len() as u64;
        let end = self.functions.len() as u64 + start;
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
        section.add_function("foo");
        section.add_function("bar");
        let bytes = section.to_bytes()?;
        assert_eq!(bytes, vec![0x03, 0x03, 0x02, 0x00, 0x01]);
        Ok(())
    }
}
