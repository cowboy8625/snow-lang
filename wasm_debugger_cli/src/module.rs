use crate::type_section::TypeSection;
use crate::utils::into_hex;
use crate::wasm_walker::WasmWalker;
use anyhow::Result;

// Next step:
// - Implement a decoder for each section
// - use ratitui to display the sectionsew
// - implement a cli interface to output information about the module
#[derive(Debug, Clone, Default)]
pub struct Module {
    pub header: Vec<u8>,
    pub custom: Vec<u8>,
    pub types: TypeSection,
    pub imports: Vec<u8>,
    pub functions: Vec<u8>,
    pub tables: Vec<u8>,
    pub memories: Vec<u8>,
    pub globals: Vec<u8>,
    pub exports: Vec<u8>,
    pub start: Vec<u8>,
    pub elements: Vec<u8>,
    pub code: Vec<u8>,
    pub data: Vec<u8>,
    pub data_count: Vec<u8>,
}

impl Module {
    pub fn new(bytes: Vec<u8>) -> Result<Self> {
        let mut wasm_walker = WasmWalker::new(&bytes);
        let mut module = Module::default();
        module.header = wasm_walker.take(8);
        while let Some(byte) = wasm_walker.next() {
            match byte {
                id @ (0..=12) => {
                    let length = wasm_walker.leb128_bytes();
                    let section = wasm_walker.get_section()?;
                    match &id {
                        0 => module.custom = section,
                        1 => module.types = TypeSection::try_from((length, section))?,
                        2 => module.imports = section,
                        3 => module.functions = section,
                        4 => module.tables = section,
                        5 => module.memories = section,
                        6 => module.globals = section,
                        7 => module.exports = section,
                        8 => module.start = section,
                        9 => module.elements = section,
                        10 => module.code = section,
                        11 => module.data = section,
                        12 => module.data_count = section,
                        _ => {}
                    }
                }
                _ => {
                    panic!("Unknown section {:02X}", byte);
                }
            }
        }
        Ok(module)
    }
}

impl std::fmt::Display for Module {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Header:\n{}", into_hex(&self.header))?;
        write!(f, "Custom:\n{}", into_hex(&self.custom))?;
        write!(f, "Types:\n{:#?}", self.types)?;
        write!(f, "Imports:\n{}", into_hex(&self.imports))?;
        write!(f, "Functions:\n{}", into_hex(&self.functions))?;
        write!(f, "Tables:\n{}", into_hex(&self.tables))?;
        write!(f, "Memeories:\n{}", into_hex(&self.memories))?;
        write!(f, "Globals:\n{}", into_hex(&self.globals))?;
        write!(f, "Exports:\n{}", into_hex(&self.exports))?;
        write!(f, "Start:\n{}", into_hex(&self.start))?;
        write!(f, "Elements:\n{}", into_hex(&self.elements))?;
        write!(f, "Code:\n{}", into_hex(&self.code))?;
        write!(f, "Data:\n{}", into_hex(&self.data))?;
        write!(f, "Data Count:\n{}", into_hex(&self.data_count))
    }
}
