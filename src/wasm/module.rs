use super::opcode::Instruction;
use super::section::Section;
use anyhow::Result;

#[derive(Debug, Default, Clone)]
pub struct Module {
    sections: Vec<Section>,
}

impl Module {
    pub fn push(&mut self, section: impl Into<Section>) {
        self.sections.push(section.into());
    }

    pub fn to_bytes(&self) -> Result<Vec<u8>> {
        let mut bytes = Vec::new();
        for section in &self.sections {
            bytes.extend(section.to_bytes()?);
        }
        Ok(bytes)
    }
}
