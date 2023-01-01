#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    TokenOpFormat(String),
    MissingEntryPoint,
    Unexpected(String, String),
    DirectiveFormat(String),
    LabelNotDefined(String),
    MissingReg(String),
}
