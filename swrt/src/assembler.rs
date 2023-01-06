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

#[test]
fn assembler_test() {
    let src = r#"
.entry main
.data
name .ascii "hello"
.text
main: hlt
"#;
    // dbg!(assembler(src));
    assert!(false);
}
// TODO:[1] Add more test to assembler.rs
// #[cfg(test)]
// mod test {
//     use super::*;
//     fn check_header(program: &[u8]) -> bool {
//         if program.len() < Assembler::HEADER_SIZE {
//             return false;
//         }
//         let end = Assembler::MAGIC_NUMBER.len();
//         let &[0x7F, 0x6e, 0x6f, 0x77] = &program[..end] else {
//             return false;
//         };
//         false
//     }
//
//     fn get_text_section_loc(program: &[u8]) -> usize {
//         let start = Assembler::TEXT_OFFSET;
//         let end = start + 4;
//         let &[a, b, c, d] = &program[start..end] else {
//             panic!("program head incorrect format");
//         };
//         u32::from_le_bytes([a, b, c, d]) as usize
//     }
//
//     fn get_data_section<'a>(program: &'a [u8]) -> &'a [u8] {
//         let data_section_start = Assembler::HEADER_SIZE;
//         let text_section_start = get_text_section_loc(program);
//         &program[data_section_start..text_section_start]
//     }
//
//     fn get_text_section<'a>(program: &'a [u8]) -> &'a [u8] {
//         let start = get_text_section_loc(program);
//         &program[start..]
//     }
//
//     #[test]
//     fn test_assembler() {
//         let src = r#"
// .entry main
// .data
// name: .ascii "Hello World!"
// .text
// main:
// hlt
//     "#;
//         let program = Assembler::new(src).assemble().unwrap();
//         assert!(check_header(&program));
//         assert_eq!(
//             get_data_section(&program),
//             &[
//                 0x48, 0x65, 0x6c, 0x6c, 0x6f, 0x20, 0x57, 0x6f, 0x72, 0x6c, 0x64, 0x21,
//                 0x00
//             ]
//         );
//         let hlt = OpCode::Hlt as u8;
//         assert_eq!(get_text_section(&program), &[hlt, 0x00, 0x00, 0x00]);
//     }
// }
