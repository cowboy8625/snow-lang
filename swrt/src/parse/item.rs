use super::{Data, Label, Token, TokenOp};
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
