use super::{
    opcode::Instruction,
    section::{
        code::{Block, Code},
        data::{Data, Segment},
        export::{Export, ExportEntry, ExportType},
        function::Function,
        header::Header,
        import::{Import, ImportEntry, ImportType},
        memory::{Memory, Page},
        start::Start,
        Section,
        _type::{FunctionType, Kind, Type, ValueType},
    },
};

use anyhow::Result;

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Module {
    pub header: Header,
    pub kind: Option<Type>,
    pub imports: Option<Import>,
    pub function: Option<Function>,
    pub memory: Option<Memory>,
    pub export: Option<Export>,
    pub start: Option<Start>,
    pub code: Option<Code>,
    pub data: Option<Data>,
    // custom: Option<Custom>,
}

impl Module {
    pub fn get_main_function_id(&self) -> Option<u32> {
        self.get_function_id("main")
    }

    pub fn get_function_id(&self, name: &str) -> Option<u32> {
        self.function
            .as_ref()
            .and_then(|function| function.get_id(name))
    }

    pub fn export(&mut self, entry: ExportEntry) {
        match self.export.as_mut() {
            Some(export) => export.push(entry),
            None => {
                let mut export = Export::default();
                export.push(entry);
                self.export = Some(export);
            }
        }
    }

    // TODO: Implement a getter to retureve a function by name to get the id of the function
    pub fn set_start(&mut self, id: u32) {
        let None = self.start else {
            panic!("Start already set");
        };
        self.start = Some(Start::new(id));
    }

    pub fn add_data(&mut self, segment: Segment) {
        match self.data.as_mut() {
            Some(data) => data.push(segment),
            None => {
                let mut data = Data::default();
                data.push(segment);
                self.data = Some(data);
            }
        }
    }

    pub fn add_string(&mut self, string: &str) -> u32 {
        match self.data.as_mut() {
            Some(data) => data.push_data(string.as_bytes().to_vec()),
            None => {
                let mut data = Data::default();
                data.push_data(string.as_bytes().to_vec());
                self.data = Some(data);
                0
            }
        }
    }

    pub fn add_memory(&mut self, page: Page) {
        match self.memory.as_mut() {
            Some(memory) => memory.push(page),
            None => {
                let mut memory = Memory::default();
                memory.push(page);
                self.memory = Some(memory);
            }
        }
    }

    pub fn add_code(&mut self, block: Block) {
        match self.code.as_mut() {
            Some(code) => {
                code.push(block);
            }
            None => {
                let mut code = Code::default();
                code.push(block);
                self.code = Some(code);
            }
        }
    }

    pub fn add_function(
        &mut self,
        name: impl Into<String>,
        definition: impl Into<Kind>,
        block: Block,
    ) {
        match self.function.as_mut() {
            Some(function) => function.add_function(name),
            None => {
                let mut function = Function::default();
                function.add_function(name);
                self.function = Some(function);
            }
        }
        self.add_type(definition);
        self.add_code(block);
    }

    // NOTE: We may need to keep better track of the impoted functions/tables/etc to know there indexes.
    fn add_type(&mut self, definition: impl Into<Kind>) {
        match self.kind.as_mut() {
            Some(kind) => {
                kind.push(definition);
            }
            None => {
                let mut kind = Type::default();
                kind.push(definition);
                self.kind = Some(kind);
            }
        }
    }

    pub fn add_imported_function(&mut self, name: impl Into<String>) {
        match self.function.as_mut() {
            Some(function) => function.add_imported_function(name),
            None => {
                let mut function = Function::default();
                function.add_imported_function(name);
                self.function = Some(function);
            }
        }
    }

    pub fn import(
        &mut self,
        module: impl Into<String>,
        name: impl Into<String>,
        kind: impl Into<Kind>,
    ) {
        let name = name.into();
        let kind = kind.into();
        let entry_type = match kind {
            Kind::Function(_) => {
                self.add_imported_function(name.clone());
                ImportType::Func
            }
        };
        let entry = ImportEntry::new(module, name, ImportType::Func);
        match self.imports.as_mut() {
            Some(imports) => {
                imports.push(entry);
                self.add_type(kind);
            }
            None => {
                let mut imports = Import::default();
                imports.push(entry);
                self.imports = Some(imports);
                self.add_type(kind);
            }
        }
    }

    // pub fn push(&mut self, section: impl Into<Section>) {
    //     self.sections.push(section.into());
    // }
    //
    // pub fn to_bytes(&self) -> Result<Vec<u8>> {
    //     let mut bytes = Vec::new();
    //     for section in &self.sections {
    //         bytes.extend(section.to_bytes()?);
    //     }
    //     Ok(bytes)
    // }
    pub fn to_bytes(&self) -> Result<Vec<u8>> {
        let mut bytes = Vec::new();

        bytes.extend(self.header.to_bytes()?);

        if let Some(kind) = &self.kind {
            bytes.extend(kind.to_bytes()?);
        }
        if let Some(imports) = &self.imports {
            bytes.extend(imports.to_bytes()?);
        }
        if let Some(function) = &self.function {
            bytes.extend(function.to_bytes()?);
        }
        if let Some(memory) = &self.memory {
            bytes.extend(memory.to_bytes()?);
        }
        if let Some(export) = &self.export {
            bytes.extend(export.to_bytes()?);
        }
        if let Some(start) = &self.start {
            bytes.extend(start.to_bytes()?);
        }
        if let Some(code) = &self.code {
            bytes.extend(code.to_bytes()?);
        }
        if let Some(data) = &self.data {
            bytes.extend(data.to_bytes()?);
        }
        Ok(bytes)
    }
}
