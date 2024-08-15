use anyhow::Result;

#[derive(Debug, Default, Clone)]
pub struct Export {
    exports: Vec<ExportEntry>,
}

impl Export {
    const ID: u8 = 0x07;

    pub fn with(mut self, export: ExportEntry) -> Self {
        self.exports.push(export);
        self
    }

    pub fn len(&self) -> usize {
        let mut length = 0;
        for export in &self.exports {
            length += export.len();
        }
        length
    }

    pub fn to_bytes(&self) -> Result<Vec<u8>> {
        let mut bytes = Vec::new();
        bytes.push(Export::ID);
        // Add 1 for the count;
        let length = self.len() + 1;
        leb128::write::unsigned(&mut bytes, length as u64)?;

        // Count
        leb128::write::unsigned(&mut bytes, self.exports.len() as u64)?;

        for export in &self.exports {
            bytes.extend(export.to_bytes()?);
        }
        Ok(bytes)
    }
}

#[derive(Debug, Clone)]
pub struct ExportEntry {
    name: String,
    export_type: ExportType,
    index: u32,
}

impl ExportEntry {
    pub fn new(name: impl Into<String>, export_type: ExportType, index: u32) -> Self {
        Self {
            name: name.into(),
            export_type,
            index,
        }
    }

    pub fn len(&self) -> usize {
        self.to_bytes().unwrap_or_default().len()
    }

    pub fn to_bytes(&self) -> Result<Vec<u8>> {
        let mut bytes = Vec::new();
        let name_as_bytes = self.name.as_bytes();
        leb128::write::unsigned(&mut bytes, name_as_bytes.len() as u64)?;
        bytes.extend(name_as_bytes);
        bytes.push(self.export_type as u8);
        leb128::write::unsigned(&mut bytes, self.index as u64)?;
        Ok(bytes)
    }
}

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum ExportType {
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
    fn test_export_entry() {
        let export = ExportEntry::new("test".to_string(), ExportType::Func, 0);
        let bytes = export.to_bytes().unwrap();
        assert_eq!(bytes, vec![0x04, 0x74, 0x65, 0x73, 0x74, 0x00, 0x00]);
    }

    #[test]
    fn test_export() {
        let export = Export::default().with(ExportEntry::new(
            "test".to_string(),
            ExportType::Func,
            0,
        ));
        let bytes = export.to_bytes().unwrap();
        assert_eq!(
            bytes,
            vec![0x07, 0x08, 0x01, 0x04, 0x74, 0x65, 0x73, 0x74, 0x00, 0x00]
        );
    }
}
