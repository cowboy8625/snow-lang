#![allow(dead_code)]
use crate::wasm_walker::WasmWalker;

#[derive(Debug, Clone, Default)]
pub struct TypeSection {
    pub length: Vec<u8>,
    pub function_types: Vec<FunctionType>,
    pub table_types: Vec<u8>,
    pub memory_types: Vec<u8>,
    pub global_types: Vec<u8>,
}

impl TypeSection {
    pub const ID: u8 = 0x01;
    pub fn count(&self) -> usize {
        self.function_types.len()
            + self.table_types.len()
            + self.memory_types.len()
            + self.global_types.len()
    }
}

impl TryFrom<(Vec<u8>, Vec<u8>)> for TypeSection {
    type Error = anyhow::Error;
    fn try_from(
        (length, bytes): (Vec<u8>, Vec<u8>),
    ) -> std::result::Result<Self, Self::Error> {
        let mut walker = WasmWalker::new(&bytes);

        // --- Count of Types ---
        let type_count_bytes = walker.leb128_bytes();
        let _ = walker.take(type_count_bytes.len());
        let _type_count =
            leb128::read::unsigned(&mut std::io::Cursor::new(type_count_bytes));

        let mut this = Self::default();
        this.length = length;

        while let Some(byte) = walker.next() {
            match byte {
                FunctionType::ID => {
                    let mut function_type = FunctionType::default();
                    // --- Count of Param Types ---
                    let params_count_bytes = walker.leb128_bytes();
                    let _ = walker.take(params_count_bytes.len());
                    let params_count = leb128::read::unsigned(
                        &mut std::io::Cursor::new(params_count_bytes),
                    )?;

                    // Params
                    for _ in 0..params_count {
                        let Some(byte) = walker.next() else {
                            // Maybe return an error here
                            panic!("Invalid Function Type");
                        };
                        function_type.params.push(Type::try_from(byte)?);
                    }

                    // --- Count of Return Types ---
                    let return_count_bytes = walker.leb128_bytes();
                    let _ = walker.take(return_count_bytes.len());
                    let return_count = leb128::read::unsigned(
                        &mut std::io::Cursor::new(return_count_bytes),
                    )?;

                    // Return
                    for _ in 0..return_count {
                        let Some(byte) = walker.next() else {
                            // Maybe return an error here
                            panic!("Invalid Function Type");
                        };
                        function_type.results.push(Type::try_from(byte)?);
                    }

                    this.function_types.push(function_type);
                }
                _ => {
                    println!("Input: {:?}", crate::utils::into_hex(&bytes));
                    println!("Unknown Type: {:02X}", byte);
                    println!("remaining: {}", crate::utils::into_hex(&walker.bytes));
                    std::process::exit(1);
                }
            }
        }

        Ok(this)
    }
}

#[derive(Debug, Clone, Default)]
pub struct FunctionType {
    pub length: Vec<u8>,
    pub params: Vec<Type>,
    pub results: Vec<Type>,
}

impl FunctionType {
    pub const ID: u8 = 0x60;
}

#[derive(Debug, Clone)]
pub enum Type {
    I32,
}

impl TryFrom<u8> for Type {
    type Error = anyhow::Error;
    fn try_from(byte: u8) -> std::result::Result<Self, Self::Error> {
        match byte {
            0x7F => Ok(Self::I32),
            _ => Err(anyhow::anyhow!("Invalid Function Param")),
        }
    }
}
