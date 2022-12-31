#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnrecognizedTokenOpError(pub String);

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    TokenOpFormat(String),
    MissingEntryPoint,
    Unexpected(String, String),
    DirectiveFormat(String),
    LabelNotDefined(String),
}
