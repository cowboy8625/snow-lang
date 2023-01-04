use crate::parse::Span;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Label {
    pub name: String,
    pub span: Span,
    pub def: bool,
}
