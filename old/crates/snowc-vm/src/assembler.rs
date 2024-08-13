use super::parse::{Data, Item, Label, Parser, Text};
use snowc_error_messages::Error;

pub type SymbolTable = std::collections::HashMap<String, u32>;

#[derive(Debug)]
struct Header([u8; Header::SIZE]);
impl Header {
    const MAGIC_NUMBER: [u8; 4] = [0x7F, 0x6e, 0x6f, 0x77];
    const TEXT_OFFSET: usize = 4;
    const ENTRY_OFFSET: usize = 8;
    const SIZE: usize = 64;

    fn set_header_text_section(&mut self, offset: u32) {
        let [a, b, c, d] = offset.to_le_bytes();
        self.0[Self::TEXT_OFFSET] = a;
        self.0[Self::TEXT_OFFSET + 1] = b;
        self.0[Self::TEXT_OFFSET + 2] = c;
        self.0[Self::TEXT_OFFSET + 3] = d;
    }

    fn set_header_entry_point(&mut self, offset: u32) {
        let [a, b, c, d] = offset.to_le_bytes();
        self.0[Self::ENTRY_OFFSET] = a;
        self.0[Self::ENTRY_OFFSET + 1] = b;
        self.0[Self::ENTRY_OFFSET + 2] = c;
        self.0[Self::ENTRY_OFFSET + 3] = d;
    }

    fn into_bytes(mut self) -> [u8; Header::SIZE] {
        self.0
    }
}

impl Default for Header {
    fn default() -> Self {
        let mut bytes = [0; Header::SIZE];
        bytes[0] = Header::MAGIC_NUMBER[0];
        bytes[1] = Header::MAGIC_NUMBER[1];
        bytes[2] = Header::MAGIC_NUMBER[2];
        bytes[3] = Header::MAGIC_NUMBER[3];
        Self(bytes)
    }
}

fn create_symbol_table(ast: &[Item]) -> SymbolTable {
    let mut st = SymbolTable::new();
    let mut pc = Header::SIZE;
    for item in ast.iter() {
        match item {
            Item::Data(data) => {
                for Data {
                    name, directive, ..
                } in data.iter()
                {
                    let size = directive.size();
                    st.insert(name.into(), pc as u32);
                    pc += size;
                }
            }
            Item::Text(text) => {
                for t in text.iter() {
                    if let Text {
                        label:
                            Some(Label {
                                name, def: true, ..
                            }),
                        ..
                    } = t
                    {
                        st.insert(name.into(), pc as u32);
                    }
                    pc += 4;
                }
            }
            _ => {}
        }
    }
    st
}

pub fn assembler(input: &str) -> Result<Vec<u8>, Error> {
    //TODO: combined bytes with header
    let error = None;
    let mut bytes = vec![];
    let ast = Parser::new(input).parse()?;
    let mut header = Header::default();

    let symbol_table = create_symbol_table(&ast);

    for item in ast {
        match item {
            Item::EntryPoint(entry) => {
                //FIXME: return error here
                let offset = symbol_table.get(entry.value()).unwrap();
                header.set_header_entry_point(*offset);
            }
            Item::Data(data) => {
                for Data { directive, .. } in data.iter() {
                    bytes.extend_from_slice(&directive.as_bytes())
                }
                let data_size = (bytes.len() + Header::SIZE) as u32;
                header.set_header_text_section(data_size);
            }
            Item::Text(text) => {
                for Text { opcode, .. } in text.iter() {
                    bytes.extend_from_slice(&opcode.as_bytes(&symbol_table)?);
                }
            }
        }
    }

    if let Some(error) = error {
        return Err(error);
    }
    let mut header = header.into_bytes().to_vec();
    header.extend_from_slice(&bytes);
    Ok(header)
}

pub fn assemble_from_ast(ast: &Vec<Item>) -> Result<Vec<u8>, Error> {
    //TODO: combined bytes with header
    let error = None;
    let mut bytes = vec![];
    let mut header = Header::default();

    let symbol_table = create_symbol_table(&ast);

    for item in ast {
        match item {
            Item::EntryPoint(entry) => {
                //FIXME: return error here
                let offset = symbol_table.get(entry.value()).unwrap();
                header.set_header_entry_point(*offset);
            }
            Item::Data(data) => {
                for Data { directive, .. } in data.iter() {
                    bytes.extend_from_slice(&directive.as_bytes())
                }
                let data_size = (bytes.len() + Header::SIZE) as u32;
                header.set_header_text_section(data_size);
            }
            Item::Text(text) => {
                for Text { opcode, .. } in text.iter() {
                    bytes.extend_from_slice(&opcode.as_bytes(&symbol_table)?);
                }
            }
        }
    }

    if let Some(error) = error {
        return Err(error);
    }
    let mut header = header.into_bytes().to_vec();
    header.extend_from_slice(&bytes);
    Ok(header)
}
