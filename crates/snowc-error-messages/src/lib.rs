use annotate_snippets::{
    display_list::{DisplayList, FormatOptions},
    snippet::{Annotation, AnnotationType, Slice, Snippet, SourceAnnotation},
};
use snowc_lexer::Span;

#[derive(Debug)]
pub struct Error {
    pub id: String,
    pub label: String,
    pub span: Span,
}

#[derive(Debug, Default)]
pub struct Errors {
    errors: Vec<Error>,
}

impl Errors {
    pub fn push(&mut self, error: Error) {
        self.errors.push(error)
    }

    pub fn pop_then_push(&mut self, error: Error) {
        self.errors.pop();
        self.errors.push(error)
    }

    pub fn pop(&mut self) -> Option<Error> {
        self.errors.pop()
    }

    pub fn len(&self) -> usize {
        self.errors.len()
    }

    pub fn is_empty(&self) -> bool {
        self.errors.is_empty()
    }

    pub fn is_last_err_code(&self, codes: &[&str]) -> bool {
        let Some(code) = self.last_err_code() else {
            return false;
        };
        codes.contains(&code.as_str())
    }

    pub fn last_err_code(&self) -> Option<&String> {
        let Some(error) = self.errors.last() else {
            return None;
        };
        Some(&error.id)
    }
}

pub fn report(filename: &str, src: &str, errors: &Errors) {
    let snippets = errors.errors
        .iter()
        .map(|error| snippet_builder(filename, src, error))
        .collect::<Vec<Snippet>>();

    for snippet in snippets {
        let dl = DisplayList::from(snippet);
        eprintln!("{}", dl);
    }
}

fn snippet_builder<'a>(
    filename: &'a str,
    src: &'a str,
    error: &'a Error,
) -> Snippet<'a> {
    let span = if error.span.end > src.len() {
        (
            error.span.start.saturating_sub(1),
            error.span.end.saturating_sub(1),
        )
    } else {
        (error.span.start, error.span.end)
    };
    Snippet {
        title: Some(Annotation {
            label: Some(&error.label),
            id: Some(&error.id),
            annotation_type: AnnotationType::Error,
        }),
        footer: vec![],
        slices: vec![Slice {
            source: src,
            line_start: error.span.line,
            origin: Some(filename),
            fold: true,
            annotations: vec![
                SourceAnnotation {
                    label: "",
                    annotation_type: AnnotationType::Error,
                    range: span,
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
    }
}
