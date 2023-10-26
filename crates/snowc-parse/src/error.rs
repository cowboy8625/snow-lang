use annotate_snippets::{
    display_list::{DisplayList, FormatOptions},
    snippet::{Annotation, AnnotationType, Slice, Snippet, SourceAnnotation},
};
use snowc_lexer::Span;
use thiserror::Error;

#[derive(Debug, Error, Clone, Copy)]
pub enum Error {
    #[error("missing identifier {0:?}")]
    MissingIdentifier(Span),
    #[error("unknown operator {0:?}")]
    UnknownOperator(Span),
    #[error("missing deliminator {0:?}")]
    MissingDeliminator(Span),
    #[error("missing condition for if statement {0:?}")]
    ExpectedConditionForStatement(Span),
    #[error("item not allowed in global space {0:?}")]
    ItemNotAllowedInGlobalSpace(Span),
    #[error("invalid char '{0}' {1:?}")]
    InvalidChar(char, Span),
    #[error("unexpected end of file {0:?}")]
    UnexpectedEOF(Span),
    #[error("unexpected token {0:?}")]
    UnexpectedToken(Span),
    #[error("closure arguments can only be one {0:?}")]
    ClosureArgumentsCanOnlyBeOne(Span),
    #[error("missing ']' to array at {0:?}")]
    UnclosedArray(Span),
    #[error("expected type {0:?}")]
    ExpectedType(Span),
    #[error("not a function {0:?}")]
    NotAFunction(Span),
    #[error("unexpected end of input")]
    UnexpectedEndOfInput(Span),
    #[error("unclosed parenthesis {0:?}")]
    UnclosedParen(Span),
}

impl Error {
    pub fn span(&self) -> Span {
        match self {
            Self::MissingIdentifier(s)
            | Self::ClosureArgumentsCanOnlyBeOne(s)
            | Self::ExpectedConditionForStatement(s)
            | Self::ExpectedType(s)
            | Self::InvalidChar(_, s)
            | Self::ItemNotAllowedInGlobalSpace(s)
            | Self::MissingDeliminator(s)
            | Self::NotAFunction(s)
            | Self::UnclosedArray(s)
            | Self::UnexpectedEOF(s)
            | Self::UnexpectedToken(s)
            | Self::UnexpectedEndOfInput(s)
            | Self::UnclosedParen(s)
            | Self::UnknownOperator(s) => *s,
        }
    }

    pub fn report<'a>(&'a self, filename: &'a str, src: &'a str) -> String {
        let span = match self {
            Self::UnexpectedEndOfInput(_) => {
                let idx_end = src.len();
                let idx_start = idx_end.saturating_sub(1);
                Span::new(idx_start, idx_end, 0, 0, 0, 0)
            }
            _ => self.span(),
        };
        let range = (span.idx_start, span.idx_end);
        let label = &self.to_string();
        let snippet = Snippet {
            title: Some(Annotation {
                label: Some(label),
                id: None,
                annotation_type: AnnotationType::Error,
            }),
            footer: vec![],
            slices: vec![Slice {
                source: src,
                line_start: span.row_start,
                origin: Some(filename),
                fold: true,
                annotations: vec![SourceAnnotation {
                    label,
                    annotation_type: AnnotationType::Error,
                    range,
                }],
            }],
            opt: FormatOptions {
                color: true,
                ..Default::default()
            },
        };
        DisplayList::from(snippet).to_string()
    }
}
