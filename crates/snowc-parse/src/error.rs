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
    #[error("who knows what you did, gezzz {0:?}")]
    Unknown(Span),
}

impl Error {
    pub fn span(&self) -> Span {
        match self {
            Self::MissingIdentifier(s) => *s,
            Self::UnknownOperator(s) => *s,
            Self::MissingDeliminator(s) => *s,
            Self::ExpectedConditionForStatement(s) => *s,
            Self::ItemNotAllowedInGlobalSpace(s) => *s,
            Self::InvalidChar(_, s) => *s,
            Self::Unknown(s) => *s,
        }
    }

    pub fn report<'a>(&'a self, filename: &'a str, src: &'a str) -> String {
        let span = self.span();
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
                annotations: vec![
                    SourceAnnotation {
                        label,
                        annotation_type: AnnotationType::Error,
                        range,
                    },
                    // SourceAnnotation {
                    //     label: "while parsing this struct",
                    //     annotation_type: AnnotationType::Info,
                    //     range: (34, 50),
                    // },
                ],
            }],
            opt: FormatOptions {
                color: true,
                ..Default::default()
            },
        };
        DisplayList::from(snippet).to_string()
    }
}
