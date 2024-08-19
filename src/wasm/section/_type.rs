use super::DataType;
use anyhow::Result;

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Type {
    length: usize,
    types: Vec<Kind>,
}

impl Type {
    const ID: u8 = 0x01;

    pub fn push(&mut self, type_: impl Into<Kind>) {
        let kind = type_.into();
        self.length += kind.to_bytes().unwrap_or_default().len();
        self.types.push(kind);
    }

    pub fn with(mut self, type_: impl Into<Kind>) -> Self {
        let kind = type_.into();
        self.length += kind.to_bytes().unwrap_or_default().len();
        self.types.push(kind);
        self
    }

    pub fn to_bytes(&self) -> Result<Vec<u8>> {
        let mut bytes = Vec::new();
        bytes.push(Type::ID);
        // Add 1 for the count;
        let length = self.length + 1;
        leb128::write::unsigned(&mut bytes, length as u64)?;
        leb128::write::unsigned(&mut bytes, self.types.len() as u64)?;
        for type_ in &self.types {
            bytes.extend(type_.to_bytes()?);
        }
        Ok(bytes)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Kind {
    Function(FunctionType),
}

impl Kind {
    pub fn to_bytes(&self) -> Result<Vec<u8>> {
        match self {
            Self::Function(function) => function.to_bytes(),
        }
    }
}

impl From<FunctionType> for Kind {
    fn from(function: FunctionType) -> Self {
        Self::Function(function)
    }
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct FunctionType {
    params: Vec<ValueType>,
    /// The last value that is on the stack
    results: Vec<DataType>,
}

impl FunctionType {
    const ID: u8 = 0x60;
    pub fn with_param(mut self, type_: ValueType) -> Self {
        self.params.push(type_);
        self
    }

    pub fn with_result(mut self, type_: DataType) -> Self {
        self.results.push(type_);
        self
    }

    pub fn params_to_bytes(&self) -> Result<Vec<u8>> {
        let mut bytes = Vec::new();
        let count = self.params.len();
        leb128::write::unsigned(&mut bytes, count as u64)?;
        for param in &self.params {
            bytes.push(param.to_byte());
        }
        Ok(bytes)
    }

    pub fn results_to_bytes(&self) -> Result<Vec<u8>> {
        let mut bytes = Vec::new();
        let count = self.results.len();
        leb128::write::unsigned(&mut bytes, count as u64)?;
        for result in &self.results {
            bytes.push(*result as u8);
        }
        Ok(bytes)
    }

    pub fn to_bytes(&self) -> Result<Vec<u8>> {
        let mut bytes = Vec::new();
        bytes.push(Self::ID);
        bytes.extend(self.params_to_bytes()?);
        bytes.extend(self.results_to_bytes()?);
        Ok(bytes)
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(u8)]
pub enum ValueType {
    WithName(String, DataType),
    Data(DataType),
}

impl ValueType {
    pub fn to_byte(&self) -> u8 {
        match self {
            Self::WithName(_, data) => *data as u8,
            Self::Data(data) => *data as u8,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_to_bytes_func_type() {
        let func_type = FunctionType::default()
            .with_param(ValueType::Data(DataType::I32))
            .with_param(ValueType::Data(DataType::I32))
            .with_result(DataType::I32);

        let bytes = func_type.to_bytes().unwrap();
        assert_eq!(bytes, vec![0x60, 0x02, 0x7F, 0x7F, 0x01, 0x7F]);
    }

    #[test]
    fn test_to_bytes_type() {
        let func_type = FunctionType::default()
            .with_param(ValueType::Data(DataType::I32))
            .with_param(ValueType::Data(DataType::I32))
            .with_result(DataType::I32);

        let r#type = Type::default().with(func_type);

        let bytes = r#type.to_bytes().unwrap();
        assert_eq!(
            bytes,
            vec![0x01, 0x07, 0x01, 0x60, 0x02, 0x7F, 0x7F, 0x01, 0x7F]
        );
    }
}
