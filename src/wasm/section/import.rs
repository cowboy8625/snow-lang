use anyhow::Result;

/// Import Section Only holds names of functions, tables, memories, and globals
/// If there is a Function import that needs to be inserted into the `[Type]` `[Section]` before
/// any module level function are define.
#[derive(Debug, Default, Clone, PartialEq)]
pub struct Import {
    imports: Vec<ImportEntry>,
}

impl Import {
    const ID: u8 = 0x02;

    pub fn push(&mut self, import: ImportEntry) {
        self.imports.push(import)
    }

    pub fn with(mut self, import: ImportEntry) -> Self {
        self.imports.push(import);
        self
    }

    pub fn to_bytes(&self) -> Result<Vec<u8>> {
        let mut bytes = Vec::new();
        bytes.push(Import::ID);
        // Add 1 for the count;
        let mut length = 1;

        let mut import_bytes = Vec::new();
        for (index, import) in self.imports.iter().enumerate() {
            import_bytes.extend(import.to_bytes(index)?);
        }

        length += import_bytes.len();

        leb128::write::unsigned(&mut bytes, length as u64)?;
        leb128::write::unsigned(&mut bytes, self.imports.len() as u64)?;
        bytes.extend(import_bytes);

        Ok(bytes)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ImportEntry {
    module: String,
    name: String,
    import_type: ImportType,
}

impl ImportEntry {
    pub fn new(
        module: impl Into<String>,
        name: impl Into<String>,
        import_type: ImportType,
    ) -> Self {
        Self {
            module: module.into(),
            name: name.into(),
            import_type,
        }
    }

    pub fn to_bytes(&self, index: usize) -> Result<Vec<u8>> {
        let mut bytes = Vec::new();
        leb128::write::unsigned(&mut bytes, self.module.len() as u64)?;
        bytes.extend(self.module.as_bytes());
        leb128::write::unsigned(&mut bytes, self.name.len() as u64)?;
        bytes.extend(self.name.as_bytes());
        bytes.push(self.import_type as u8);
        leb128::write::unsigned(&mut bytes, index as u64)?;
        Ok(bytes)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(u8)]
pub enum ImportType {
    Func = 0x00,
    Table = 0x01,
    Memory = 0x02,
    Global = 0x03,
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_import_entry() {
        let section = ImportEntry::new("module", "name", ImportType::Func);
        let bytes = section.to_bytes(0).unwrap();
        assert_eq!(
            bytes,
            vec![
                0x06, 0x6d, 0x6f, 0x64, 0x75, 0x6c, 0x65, 0x04, 0x6e, 0x61, 0x6d, 0x65,
                0x00, 0x00
            ]
        );
    }

    #[test]
    fn test_import() {
        let section = ImportEntry::new("module", "name", ImportType::Func);
        let section = Import::default().with(section);
        let bytes = section.to_bytes().unwrap();
        assert_eq!(
            bytes,
            vec![
                0x02, 0x0F, 0x01, 0x06, 0x6d, 0x6f, 0x64, 0x75, 0x6c, 0x65, 0x04, 0x6e,
                0x61, 0x6d, 0x65, 0x00, 0x00
            ]
        );
    }
}
