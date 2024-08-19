use super::section::DataType;
use anyhow::Result;

#[derive(Debug, Clone, PartialEq)]
pub enum Instruction {
    /// 0x1a is the opcode to pop the value from the stack
    Drop,
    /// 0x10 is the opcode to call a function
    Call(u32),
    // Numeric instructions
    /// 0x41 is the opcode for i32.const
    I32Const(i32),
    /// 0x42 is the opcode for i64.const
    I64Const(i64),
    F32Const(f32),
    F64Const(f64),

    // Local
    LocalGet(u32),
    // LocalSet(u32),
    // LocalTee(u32),
    // GlobalGet(u32),
    // GlobalSet(u32),

    // Arithmetic instructions
    /// 0x6a is the opcode for i32.add
    I32Add,
    /// 0x6b is the opcode for i32.sub
    I32Sub,
    /// 0x6c is the opcode for i32.mul
    I32Mul,
    /// 0x6d is the opcode for i32.div_s
    I32DivS,
    /// 0x6e is the opcode for i32.div_u
    I32DivU,
    /// 0x6f is the opcode for i32.rem_s
    I32RemS,
    /// 0x70 is the opcode for i32.rem_u
    I32RemU,
    /// 0x6a is the opcode for i32.gt_s
    I32Gt,

    // Control instructions
    /// 0x04 is the opcode for if followed by the `[DataType]`
    If(DataType),
    /// 0x05 is the opcode for else
    Else,
    /// 0x0b is the opcode for end
    End,
    Br(u32),
    BrIf(u32),
    BrTable(Vec<u32>, u32),
    Return,

    // Memory instructions
    I32Load,
    I32Store,
    // more instructions...

    // Other instructions
    Nop,
    Unreachable,
    // more instructions...
}

impl Instruction {
    pub fn to_bytes(&self) -> Result<Vec<u8>> {
        match self {
            Self::Drop => Ok(vec![0x1a]), // 0x1a is the opcode for drop
            Self::Call(index) => {
                let mut bytes = vec![0x10]; // 0x10 is the opcode for call
                leb128::write::unsigned(&mut bytes, *index as u64)?;
                Ok(bytes)
            }
            Self::LocalGet(index) => {
                let mut bytes = vec![0x20]; // 0x20 is the opcode for local.get
                leb128::write::unsigned(&mut bytes, *index as u64)?;
                Ok(bytes)
            }
            // Numeric instructions
            Self::I32Const(value) => {
                let mut bytes = vec![0x41]; // 0x41 is the opcode for i32.const
                leb128::write::unsigned(&mut bytes, *value as u64)?;
                Ok(bytes)
            }
            Self::I64Const(value) => {
                let mut bytes = vec![0x42]; // 0x42 is the opcode for i64.const
                leb128::write::unsigned(&mut bytes, *value as u64)?;
                Ok(bytes)
            }
            Self::F32Const(value) => {
                let mut bytes = vec![0x43]; // 0x43 is the opcode for f32.const
                bytes.extend(value.to_le_bytes());
                Ok(bytes)
            }
            Self::F64Const(value) => {
                let mut bytes = vec![0x44]; // 0x44 is the opcode for f64.const
                bytes.extend(value.to_le_bytes());
                Ok(bytes)
            }

            // Arithmetic instructions (all are single-byte opcodes)
            Self::I32Add => Ok(vec![0x6a]),
            Self::I32Sub => Ok(vec![0x6b]),
            Self::I32Mul => Ok(vec![0x6c]),
            Self::I32DivS => Ok(vec![0x6d]),
            Self::I32DivU => Ok(vec![0x6e]),
            Self::I32RemS => Ok(vec![0x6f]),
            Self::I32RemU => Ok(vec![0x70]),
            Self::I32Gt => Ok(vec![0x6a]),

            // Control instructions
            Self::If(data_type) => Ok(vec![0x04]),
            Self::Else => Ok(vec![0x05]),
            Self::End => Ok(vec![0x0b]),
            Self::Br(label_idx) => {
                let mut bytes = vec![0x0c]; // 0x0c is the opcode for br
                leb128::write::unsigned(&mut bytes, *label_idx as u64)?;
                Ok(bytes)
            }
            Self::BrIf(label_idx) => {
                let mut bytes = vec![0x0d]; // 0x0d is the opcode for br_if
                leb128::write::unsigned(&mut bytes, *label_idx as u64)?;
                Ok(bytes)
            }
            Self::BrTable(table, default) => {
                let mut bytes = vec![0x0e]; // 0x0e is the opcode for br_table
                leb128::write::unsigned(&mut bytes, table.len() as u64)?;
                for &label in table {
                    leb128::write::unsigned(&mut bytes, label as u64)?;
                }
                leb128::write::unsigned(&mut bytes, *default as u64)?;
                Ok(bytes)
            }
            Self::Return => Ok(vec![0x0f]), // 0x0f is the opcode for return

            // Memory instructions
            Self::I32Load => Ok(vec![0x28]), // 0x28 is the opcode for i32.load
            // HACK: 0x02 is the allignment
            // HACK: 0x00 is the offset
            Self::I32Store => Ok(vec![0x36, 0x02, 0x00]), // 0x36 is the opcode for i32.store

            // Other instructions
            Self::Nop => Ok(vec![0x01]), // 0x01 is the opcode for nop
            Self::Unreachable => Ok(vec![0x00]), // 0x00 is the opcode for unreachable
        }
    }

    pub fn len(&self) -> usize {
        self.to_bytes().unwrap_or_default().len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_to_bytes() -> Result<()> {
        assert_eq!(Instruction::I32Add.to_bytes()?, vec![0x6A]);
        assert_eq!(Instruction::I32Const(42).to_bytes()?, vec![0x41, 0x2A]);
        Ok(())
    }
}
