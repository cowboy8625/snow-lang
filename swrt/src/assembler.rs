use super::{error::Error, parse::*};

pub type SymbolTable = std::collections::HashMap<String, u32>;
pub struct Assembler<'a> {
    input: &'a str,
    cursor: usize,
    entry_point: Option<String>,
    symbol: SymbolTable,
    executable: Vec<u8>,
    errors: Vec<Error>,
}

impl<'a> Assembler<'a> {
    const TEXT_OFFSET: usize = 4;
    const ENTRY_OFFSET: usize = 8;
    const MAGIC_NUMBER: [u8; 4] = [0x7F, 0x6e, 0x6f, 0x77];
    const HEADER_SIZE: usize = 64;
    pub fn new(input: &'a str) -> Self {
        Self {
            input: input.trim(),
            cursor: 0,
            entry_point: None,
            symbol: SymbolTable::new(),
            executable: vec![],
            errors: vec![],
        }
    }

    fn create_header(&mut self) {
        let Self { input, cursor, .. } = self;
        match input[*cursor..].parse::<Entry>() {
            Ok(entry) => {
                self.cursor += entry.len();
                let Entry { name } = entry;
                self.entry_point = Some(name);
                self.executable.extend_from_slice(&Self::MAGIC_NUMBER);
                while self.executable.len() < Self::HEADER_SIZE {
                    self.executable.push(0);
                }
            }
            Err(e) => self.errors.push(e),
        }
    }

    fn set_header_text_section(&mut self) {
        let start = self.executable.len() as u32;
        let [a, b, c, d] = start.to_le_bytes();
        self.executable[Self::TEXT_OFFSET] = a;
        self.executable[Self::TEXT_OFFSET + 1] = b;
        self.executable[Self::TEXT_OFFSET + 2] = c;
        self.executable[Self::TEXT_OFFSET + 3] = d;
    }

    fn set_header_entry_point(&mut self, offset: u32) {
        let [a, b, c, d] = offset.to_le_bytes();
        self.executable[Self::ENTRY_OFFSET] = a;
        self.executable[Self::ENTRY_OFFSET + 1] = b;
        self.executable[Self::ENTRY_OFFSET + 2] = c;
        self.executable[Self::ENTRY_OFFSET + 3] = d;
    }

    fn create_data_section(&mut self) {
        // FIXME: Check to see if the offset of the data after
        // the first one is aligned correctly
        let Self {
            input,
            cursor,
            executable,
            ..
        } = self;
        let (head, _tail) = &input[*cursor..].split_once('\n').unwrap_or((input, ""));
        if head != &".data" {
            return;
        }
        *cursor += head.len() + 1;
        while let Some((head, _tail)) = &input[*cursor..].split_once('\n') {
            if head == &".text" {
                break;
            }
            match head.parse::<Data>() {
                Ok(data) => {
                    *cursor += head.len() + 1;
                    let Data { name, directive } = data;
                    self.symbol.insert(name, executable.len() as u32);
                    let bytes = &directive.into_bytes();
                    executable.extend_from_slice(bytes);
                }
                Err(e) => {
                    self.errors.push(e);
                }
            }
        }
    }

    fn find_text_labels(&mut self) {
        let Self {
            symbol,
            input,
            cursor,
            executable,
            ..
        } = self;

        let (head, _) = input[*cursor..].split_once('\n').unwrap_or((input, ""));
        if head != ".text" {
            panic!("expected a '.text' found '{}'", head);
        }
        *cursor += head.len() + 1;
        let mut pc = executable.len() as u32;
        for line in input[*cursor..].lines() {
            let line = line.trim();
            if let Ok(Label(label)) = line.parse::<Label>() {
                symbol.insert(label, pc);
                continue;
            }
            pc += 4;
        }
        let Some(entry_point) = &self.entry_point else {
            self.errors.push(Error::MissingEntryPoint);
            return;
        };
        let Some(offset) = self.symbol.get(entry_point) else {
            self.errors.push(Error::LabelNotDefined(entry_point.into()));
            return;
        };
        self.set_header_entry_point(*offset);
    }

    fn create_text_section(&mut self) {
        self.set_header_text_section();
        let Self {
            symbol,
            input,
            cursor,
            executable,
            ..
        } = self;
        for line in input[*cursor..].lines() {
            let line = line.trim();
            match line.parse::<TokenOp>() {
                Ok(opcode) => {
                    match opcode.into_bytes(symbol) {
                        Ok(code) => {
                            executable.extend_from_slice(&code);
                        }
                        Err(e) => self.errors.push(e),
                    };
                }
                Err(e) => {
                    if line.parse::<Label>().is_err() {
                        self.errors.push(e);
                    }
                }
            }
        }
    }

    pub fn assemble(mut self) -> Result<Vec<u8>, Vec<Error>> {
        self.create_header();
        self.create_data_section();
        self.find_text_labels();
        self.create_text_section();

        if !self.errors.is_empty() {
            return Err(self.errors);
        }
        Ok(self.executable)
    }
}

// TODO:[1] Add more test to assembler.rs
#[cfg(test)]
mod test {
    use super::*;
    fn check_header(program: &[u8]) -> bool {
        if program.len() < Assembler::HEADER_SIZE {
            return false;
        }
        let end = Assembler::MAGIC_NUMBER.len();
        let &[0x7F, 0x6e, 0x6f, 0x77] = &program[..end] else {
            return false;
        };
        false
    }

    fn get_text_section_loc(program: &[u8]) -> usize {
        let start = Assembler::TEXT_OFFSET;
        let end = start + 4;
        let &[a, b, c, d] = &program[start..end] else {
            panic!("program head incorrect format");
        };
        u32::from_le_bytes([a, b, c, d]) as usize
    }

    fn get_data_section<'a>(program: &'a[u8]) -> &'a[u8] {
        let data_section_start = Assembler::HEADER_SIZE;
        let text_section_start = get_text_section_loc(program);
        &program[data_section_start..text_section_start]
    }

    fn get_text_section<'a>(program: &'a[u8]) -> &'a[u8] {
        let start = get_text_section_loc(program);
        &program[start..]
    }

    #[test]
    fn test_assembler() {
        let src = r#"
.entry main
.data
name: .ascii "Hello World!"
.text
main:
hlt
    "#;
        let program = Assembler::new(src).assemble().unwrap();
        assert!(check_header(&program));
        assert_eq!(get_data_section(&program), &[
                   0x48, 0x65, 0x6c, 0x6c, 0x6f, 0x20, 0x57, 0x6f, 0x72, 0x6c, 0x64, 0x21, 0x00
        ]);
        let hlt = OpCode::Hlt as u8;
        assert_eq!(get_text_section(&program), &[
                   hlt, 0x00, 0x00, 0x00
        ]);
    }
}
