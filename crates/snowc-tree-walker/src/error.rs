use annotate_snippets::{
    display_list::{DisplayList, FormatOptions},
    snippet::{Annotation, AnnotationType, Slice, Snippet, SourceAnnotation},
};
use snowc_parse::Span;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum RuntimeError {
    #[error("undefined identifier {0} {1:?}")]
    Undefined(String, Span),
    #[error("missing main function")]
    MissingMainFunction,
    #[error("invalid args to function")]
    InvalidArguments(Span),
    #[error("index out of bounds")]
    IdxOutOfBounds(Span),
}

impl RuntimeError {
    pub fn report(&self, filename: &str, src: &str) -> String {
        match self {
            Self::Undefined(name, span) => {
                let label = format!("undefined identifier '{name}'");
                let snippet = snippet_builder(filename, src, &label, *span);
                DisplayList::from(snippet).to_string()
            }
            Self::MissingMainFunction => {
                let label = "missing main function";
                let span = Span::default();
                let snippet = snippet_builder(filename, src, label, span);
                DisplayList::from(snippet).to_string()
            }
            Self::InvalidArguments(span) => {
                let label = "invalid args to function";
                let snippet = snippet_builder(filename, src, label, *span);
                DisplayList::from(snippet).to_string()
            }
            Self::IdxOutOfBounds(span) => {
                let label = "index out of bounds";
                let snippet = snippet_builder(filename, src, label, *span);
                DisplayList::from(snippet).to_string()
            }
        }
    }
}

fn snippet_builder<'a>(
    filename: &'a str,
    src: &'a str,
    label: &'a str,
    span: Span,
) -> Snippet<'a> {
    // assert!(span.start < span.end, "{span:?}");

    // let range = if span.start < span.end {
    //     (span.start, span.end)
    // } else {
    //     (span.end, span.start)
    // };
    let range = (span.idx_start, span.idx_end);
    Snippet {
        title: Some(Annotation {
            label: Some(label),
            id: None,
            annotation_type: AnnotationType::Error,
        }),
        footer: vec![],
        slices: vec![Slice {
            source: src,
            line_start: span.col_start,
            origin: Some(filename),
            fold: true,
            annotations: vec![SourceAnnotation {
                label: "",
                annotation_type: AnnotationType::Error,
                range,
            }],
        }],
        opt: FormatOptions {
            color: true,
            ..Default::default()
        },
    }
}
