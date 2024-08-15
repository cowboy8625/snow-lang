// Id | Section
// 0  | Custom Section
// 1  | Type Section
// 2  | Import Section
// 3  | Function Section
// 4  | Table Section
// 5  | Memory Section
// 6  | Global Section
// 7  | Export Section
// 8  | Start Section
// 9  | Element Section
// 10 | Code Section
// 11 | Data Section

pub mod _type;
pub mod code;
pub mod export;
pub mod function;
pub mod header;
pub mod start;
use super::opcode::Instruction;
use _type::Type;
use anyhow::Result;
use code::Code;
use export::Export;
use function::Function;
use header::Header as Custom;
use start::Start;

macro_rules! into_section {
    ($($section:ident),*) => {
        $(
            impl From<$section> for Section {
                fn from(section: $section) -> Self {
                    Section::$section(section)
                }
            }
        )*
    };
}

#[derive(Debug, Clone)]
pub enum Section {
    Custom(Custom), // 0x00: Custom section with name and data
    Type(Type),     // 0x01: Type section with function signatures
    // Import(Vec<u8>),           // 0x02: Import section with imported functions, tables, etc.
    Function(Function), // 0x03: Function section with function indices
    // Table(Vec<u8>),            // 0x04: Table section with table definitions
    // Memory(Vec<u8>),           // 0x05: Memory section with memory definitions
    // Global(Vec<u8>),           // 0x06: Global section with global variables
    Export(Export), // 0x07: Export section with exported functions, tables, etc.
    Start(Start),   // 0x08: Start section with the index of the start function
    // Element(Vec<u8>),          // 0x09: Element section with function table elements
    Code(Code), // 0x0A: Code section with function bodies
                // Data(Vec<u8>),             // 0x0B: Data section with initialization data for memory
}

impl Section {
    pub fn to_bytes(&self) -> Result<Vec<u8>> {
        match self {
            Section::Custom(data) => data.to_bytes(),
            Section::Type(data) => data.to_bytes(),
            Section::Function(data) => data.to_bytes(),
            Section::Export(data) => data.to_bytes(),
            Section::Start(data) => data.to_bytes(),
            Section::Code(data) => data.to_bytes(),
        }
    }
}

into_section!(Custom, Type, Function, Export, Start, Code);
