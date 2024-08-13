#[derive(Debug, Clone)]
pub enum WasmInstruction {
    // Numeric instructions
    I32Const(i32),
    I64Const(i64),
    F32Const(f32),
    F64Const(f64),

    // Arithmetic instructions
    I32Add,
    I32Sub,
    I32Mul,
    I32DivS,
    I32DivU,
    I32RemS,
    I32RemU,

    // Control instructions
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

impl WasmInstruction {
    pub fn to_bytes(&self) -> Vec<u8> {
        match self {
            // Numeric instructions
            Self::I32Const(value) => {
                let mut bytes = vec![0x41]; // 0x41 is the opcode for i32.const
                bytes.extend(i32::to_le_bytes(*value));
                bytes
            }
            Self::I64Const(value) => {
                let mut bytes = vec![0x42]; // 0x42 is the opcode for i64.const
                bytes.extend(i64::to_le_bytes(*value));
                bytes
            }
            Self::F32Const(value) => {
                let mut bytes = vec![0x43]; // 0x43 is the opcode for f32.const
                bytes.extend(f32::to_le_bytes(*value));
                bytes
            }
            Self::F64Const(value) => {
                let mut bytes = vec![0x44]; // 0x44 is the opcode for f64.const
                bytes.extend(f64::to_le_bytes(*value));
                bytes
            }

            // Arithmetic instructions (all are single-byte opcodes)
            Self::I32Add => vec![0x6a],
            Self::I32Sub => vec![0x6b],
            Self::I32Mul => vec![0x6c],
            Self::I32DivS => vec![0x6d],
            Self::I32DivU => vec![0x6e],
            Self::I32RemS => vec![0x6f],
            Self::I32RemU => vec![0x70],

            // Control instructions
            Self::Br(label_idx) => {
                let mut bytes = vec![0x0c]; // 0x0c is the opcode for br
                bytes.extend(u32::to_le_bytes(*label_idx));
                bytes
            }
            Self::BrIf(label_idx) => {
                let mut bytes = vec![0x0d]; // 0x0d is the opcode for br_if
                bytes.extend(u32::to_le_bytes(*label_idx));
                bytes
            }
            Self::BrTable(table, default) => {
                let mut bytes = vec![0x0e]; // 0x0e is the opcode for br_table
                bytes.extend(u32::to_le_bytes(table.len() as u32));
                for &label in table {
                    bytes.extend(u32::to_le_bytes(label));
                }
                bytes.extend(u64::to_le_bytes(*default as u64));
                bytes
            }
            Self::Return => vec![0x0f], // 0x0f is the opcode for return

            // Memory instructions
            Self::I32Load => vec![0x28], // 0x28 is the opcode for i32.load
            Self::I32Store => vec![0x36], // 0x36 is the opcode for i32.store

            // Other instructions
            Self::Nop => vec![0x01], // 0x01 is the opcode for nop
            Self::Unreachable => vec![0x00], // 0x00 is the opcode for unreachable
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_to_bytes() {
        assert_eq!(WasmInstruction::I32Add.to_bytes(), vec![0x6A]);
        assert_eq!(
            WasmInstruction::I32Const(42).to_bytes(),
            vec![0x41, 0x2A, 0x00, 0x00, 0x00]
        );
    }
}
