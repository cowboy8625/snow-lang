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
                    eprintln!("{directive:?}, size: {size}");
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
    dbg!(assembler(src));
    assert!(false);
}
// pub struct Assembler<'a> {
//     input: &'a str,
//     cursor: usize,
//     entry_point: Option<String>,
//     symbol: SymbolTable,
//     executable: Vec<u8>,
//     errors: Option<Error>,
// }
//
// impl<'a> Assembler<'a> {
//     const TEXT_OFFSET: usize = 4;
//     const ENTRY_OFFSET: usize = 8;
//     const MAGIC_NUMBER: [u8; 4] = [0x7F, 0x6e, 0x6f, 0x77];
//     const HEADER_SIZE: usize = 64;
//     pub fn new(input: &'a str) -> Self {
//         Self {
//             input: input.trim(),
//             cursor: 0,
//             entry_point: None,
//             symbol: SymbolTable::new(),
//             executable: vec![],
//             errors: None,
//         }
//     }
//
//     fn create_header(&mut self) {
//         let Self { input, cursor, .. } = self;
//         match input[*cursor..].parse::<Entry>() {
//             Ok(entry) => {
//                 self.cursor += entry.len();
//                 let Entry { name } = entry;
//                 self.entry_point = Some(name);
//                 self.executable.extend_from_slice(&Self::MAGIC_NUMBER);
//                 while self.executable.len() < Self::HEADER_SIZE {
//                     self.executable.push(0);
//                 }
//             }
//             Err(e) => self.errors.push(e),
//         }
//     }
//
//     fn set_header_text_section(&mut self) {
//         let start = self.executable.len() as u32;
//         let [a, b, c, d] = start.to_le_bytes();
//         self.executable[Self::TEXT_OFFSET] = a;
//         self.executable[Self::TEXT_OFFSET + 1] = b;
//         self.executable[Self::TEXT_OFFSET + 2] = c;
//         self.executable[Self::TEXT_OFFSET + 3] = d;
//     }
//
//     fn set_header_entry_point(&mut self, offset: u32) {
//         let [a, b, c, d] = offset.to_le_bytes();
//         self.executable[Self::ENTRY_OFFSET] = a;
//         self.executable[Self::ENTRY_OFFSET + 1] = b;
//         self.executable[Self::ENTRY_OFFSET + 2] = c;
//         self.executable[Self::ENTRY_OFFSET + 3] = d;
//     }
//
//     // fn create_data_section(&mut self) {
//     //     // FIXME: Check to see if the offset of the data after
//     //     // the first one is aligned correctly
//     //     let Self {
//     //         input,
//     //         cursor,
//     //         executable,
//     //         ..
//     //     } = self;
//     //     let (head, _tail) = &input[*cursor..].split_once('\n').unwrap_or((input, ""));
//     //     if head != &".data" {
//     //         return;
//     //     }
//     //     *cursor += head.len() + 1;
//     //     while let Some((head, _tail)) = &input[*cursor..].split_once('\n') {
//     //         if head == &".text" {
//     //             break;
//     //         }
//     //         match head.parse::<Data>() {
//     //             Ok(data) => {
//     //                 *cursor += head.len() + 1;
//     //                 let Data { name, directive } = data;
//     //                 self.symbol.insert(name, executable.len() as u32);
//     //                 let bytes = &directive.into_bytes();
//     //                 executable.extend_from_slice(bytes);
//     //             }
//     //             Err(e) => {
//     //                 self.errors.push(e);
//     //             }
//     //         }
//     //     }
//     // }
//     //
//     // fn find_text_labels(&mut self) {
//     //     let Self {
//     //         symbol,
//     //         input,
//     //         cursor,
//     //         executable,
//     //         ..
//     //     } = self;
//     //
//     //     let (head, _) = input[*cursor..].split_once('\n').unwrap_or((input, ""));
//     //     if head != ".text" {
//     //         panic!("expected a '.text' found '{}'", head);
//     //     }
//     //     *cursor += head.len() + 1;
//     //     let mut pc = executable.len() as u32;
//     //     for line in input[*cursor..].lines() {
//     //         let line = line.trim();
//     //         if let Ok(Label(label)) = line.parse::<Label>() {
//     //             symbol.insert(label, pc);
//     //             continue;
//     //         }
//     //         pc += 4;
//     //     }
//     //     let Some(entry_point) = &self.entry_point else {
//     //         self.errors.push(Error::MissingEntryPoint);
//     //         return;
//     //     };
//     //     let Some(offset) = self.symbol.get(entry_point) else {
//     //         self.errors.push(Error::LabelNotDefined(entry_point.into()));
//     //         return;
//     //     };
//     //     self.set_header_entry_point(*offset);
//     // }
//     //
//     // fn create_text_section(&mut self) {
//     //     self.set_header_text_section();
//     //     let Self {
//     //         symbol,
//     //         input,
//     //         cursor,
//     //         executable,
//     //         ..
//     //     } = self;
//     //     for line in input[*cursor..].lines() {
//     //         let line = line.trim();
//     //         match line.parse::<TokenOp>() {
//     //             Ok(opcode) => {
//     //                 match opcode.into_bytes(symbol) {
//     //                     Ok(code) => {
//     //                         executable.extend_from_slice(&code);
//     //                     }
//     //                     Err(e) => self.errors.push(e),
//     //                 };
//     //             }
//     //             Err(e) => {
//     //                 if line.parse::<Label>().is_err() {
//     //                     self.errors.push(e);
//     //                 }
//     //             }
//     //         }
//     //     }
//     // }
//
//     pub fn assemble(mut self) -> Result<Vec<u8>, Error> {
//         self.create_header();
//         // self.create_data_section();
//         // self.find_text_labels();
//         // self.create_text_section();
//
//         if let Some(error) = self.errors {
//             return Err(error);
//         }
//         Ok(self.executable)
//     }
// }

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
