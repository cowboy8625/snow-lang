use super::{Data, Label, Span, Token, TokenOp};
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Item {
    EntryPoint(Token),
    Data(Vec<Data>),
    Text(Vec<Text>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Text {
    pub label: Option<Label>,
    pub opcode: TokenOp,
}

impl Text {
    pub fn new_opcode(opcode: TokenOp) -> Self {
        Self {
            label: None,
            opcode,
        }
    }
    pub fn new_opcode_with_label(label: impl Into<String>, opcode: TokenOp) -> Self {
        Self {
            label: Some(Label {
                name: label.into(),
                span: Span::default(),
                def: true,
            }),
            opcode,
        }
    }
}
