use super::Instruction;
use anyhow::Result;

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Code {
    length: usize,
    blocks: Vec<Block>,
}

impl Code {
    const ID: u8 = 0x0A;

    pub fn push(&mut self, block: Block) {
        self.length += block.len();
        self.blocks.push(block);
    }

    pub fn with(mut self, block: Block) -> Self {
        self.length += block.len();
        self.blocks.push(block);
        self
    }

    pub fn to_bytes(&self) -> Result<Vec<u8>> {
        let mut bytes = Vec::new();
        bytes.push(Code::ID);
        // Add 1 for the count;
        let length = self.length + 1 + self.blocks.len();
        leb128::write::unsigned(&mut bytes, length as u64)?;
        let count = self.blocks.len();
        leb128::write::unsigned(&mut bytes, count as u64)?;
        for block in &self.blocks {
            bytes.extend(block.to_bytes()?);
        }
        Ok(bytes)
    }
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Block {
    instructions: Vec<Instruction>,
}

impl Block {
    pub fn new(instructions: Vec<Instruction>) -> Self {
        Self { instructions }
    }
    fn len(&self) -> usize {
        let mut length = 0;
        for instruction in &self.instructions {
            length += instruction.len();
        }
        // Byte for 0x00 after the length
        length += 1;
        // Byte for 0x0B end of block
        length += 1;
        length
    }

    pub fn with(mut self, instruction: Instruction) -> Self {
        self.instructions.push(instruction);
        self
    }

    fn to_bytes(&self) -> Result<Vec<u8>> {
        let mut bytes = Vec::new();
        let length = self.len();
        leb128::write::unsigned(&mut bytes, length as u64)?;
        bytes.push(0x00);
        for instruction in &self.instructions {
            bytes.extend(instruction.to_bytes()?);
        }
        bytes.push(0x0B);
        Ok(bytes)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_code_block() {
        let block = Block::default()
            .with(Instruction::I32Const(1))
            .with(Instruction::I32Const(2))
            .with(Instruction::I32Add);
        eprintln!("{:?}", block);
        let bytes = match block.to_bytes() {
            Ok(bytes) => bytes,
            Err(err) => panic!("ERROR: {}", err),
        };
        assert_eq!(bytes, vec![0x07, 0x00, 0x41, 0x01, 0x41, 0x02, 0x6A, 0x0B]);
    }

    #[test]
    fn test_code_section() {
        let code_section = Code::default().with(
            Block::default()
                .with(Instruction::I32Const(1))
                .with(Instruction::I32Const(2))
                .with(Instruction::I32Add),
        );

        let bytes = code_section.to_bytes().unwrap();
        assert_eq!(
            bytes,
            vec![0x0A, 0x09, 0x01, 0x07, 0x00, 0x41, 0x01, 0x41, 0x02, 0x6A, 0x0B]
        );
    }
}
