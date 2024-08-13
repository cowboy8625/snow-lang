pub enum WasmSection {
    Custom(Vec<u8>),   // 0x00: Custom section with name and data
    Type(Vec<u8>),     // 0x01: Type section with function signatures
    Import(Vec<u8>),   // 0x02: Import section with imported functions, tables, etc.
    Function(Vec<u8>), // 0x03: Function section with function indices
    Table(Vec<u8>),    // 0x04: Table section with table definitions
    Memory(Vec<u8>),   // 0x05: Memory section with memory definitions
    Global(Vec<u8>),   // 0x06: Global section with global variables
    Export(Vec<u8>),   // 0x07: Export section with exported functions, tables, etc.
    Start(Vec<u8>),    // 0x08: Start section with the index of the start function
    Element(Vec<u8>),  // 0x09: Element section with function table elements
    Code(Vec<u8>),     // 0x0A: Code section with function bodies
    Data(Vec<u8>),     // 0x0B: Data section with initialization data for memory
}

impl WasmSection {
    /// Converts the section to bytes, including the section ID and its size.
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();

        match self {
            WasmSection::Custom(data) => {
                bytes.push(0x00); // Section ID for Custom
                bytes.extend(
                    leb128::write::unsigned(&mut Vec::new(), data.len() as u64).unwrap(),
                );
                bytes.extend(data);
            }
            WasmSection::Type(data) => {
                bytes.push(0x01); // Section ID for Type
                bytes.extend(
                    leb128::write::unsigned(&mut Vec::new(), data.len() as u64).unwrap(),
                );
                bytes.extend(data);
            }
            WasmSection::Import(data) => {
                bytes.push(0x02); // Section ID for Import
                bytes.extend(
                    leb128::write::unsigned(&mut Vec::new(), data.len() as u64).unwrap(),
                );
                bytes.extend(data);
            }
            WasmSection::Function(data) => {
                bytes.push(0x03); // Section ID for Function
                bytes.extend(
                    leb128::write::unsigned(&mut Vec::new(), data.len() as u64).unwrap(),
                );
                bytes.extend(data);
            }
            WasmSection::Table(data) => {
                bytes.push(0x04); // Section ID for Table
                bytes.extend(
                    leb128::write::unsigned(&mut Vec::new(), data.len() as u64).unwrap(),
                );
                bytes.extend(data);
            }
            WasmSection::Memory(data) => {
                bytes.push(0x05); // Section ID for Memory
                bytes.extend(
                    leb128::write::unsigned(&mut Vec::new(), data.len() as u64).unwrap(),
                );
                bytes.extend(data);
            }
            WasmSection::Global(data) => {
                bytes.push(0x06); // Section ID for Global
                bytes.extend(
                    leb128::write::unsigned(&mut Vec::new(), data.len() as u64).unwrap(),
                );
                bytes.extend(data);
            }
            WasmSection::Export(data) => {
                bytes.push(0x07); // Section ID for Export
                bytes.extend(
                    leb128::write::unsigned(&mut Vec::new(), data.len() as u64).unwrap(),
                );
                bytes.extend(data);
            }
            WasmSection::Start(data) => {
                bytes.push(0x08); // Section ID for Start
                bytes.extend(
                    leb128::write::unsigned(&mut Vec::new(), data.len() as u64).unwrap(),
                );
                bytes.extend(data);
            }
            WasmSection::Element(data) => {
                bytes.push(0x09); // Section ID for Element
                bytes.extend(
                    leb128::write::unsigned(&mut Vec::new(), data.len() as u64).unwrap(),
                );
                bytes.extend(data);
            }
            WasmSection::Code(data) => {
                bytes.push(0x0A); // Section ID for Code
                bytes.extend(
                    leb128::write::unsigned(&mut Vec::new(), data.len() as u64).unwrap(),
                );
                bytes.extend(data);
            }
            WasmSection::Data(data) => {
                bytes.push(0x0B); // Section ID for Data
                bytes.extend(
                    leb128::write::unsigned(&mut Vec::new(), data.len() as u64).unwrap(),
                );
                bytes.extend(data);
            }
        }

        bytes
    }

    /// Constructs a WasmSection from raw bytes.
    pub fn from_bytes(section_id: u8, bytes: Vec<u8>) -> Result<Self, &'static str> {
        match section_id {
            0x00 => Ok(WasmSection::Custom(bytes)),
            0x01 => Ok(WasmSection::Type(bytes)),
            0x02 => Ok(WasmSection::Import(bytes)),
            0x03 => Ok(WasmSection::Function(bytes)),
            0x04 => Ok(WasmSection::Table(bytes)),
            0x05 => Ok(WasmSection::Memory(bytes)),
            0x06 => Ok(WasmSection::Global(bytes)),
            0x07 => Ok(WasmSection::Export(bytes)),
            0x08 => Ok(WasmSection::Start(bytes)),
            0x09 => Ok(WasmSection::Element(bytes)),
            0x0A => Ok(WasmSection::Code(bytes)),
            0x0B => Ok(WasmSection::Data(bytes)),
            _ => Err("Unknown section ID"),
        }
    }
}
