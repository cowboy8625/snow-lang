use super::{Directive, Span};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Data {
    pub name: String,
    pub directive: Directive,
    pub span: Span,
}
