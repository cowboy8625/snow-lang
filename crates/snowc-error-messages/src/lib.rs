use annotate_snippets::{
    display_list::{DisplayList, FormatOptions},
    snippet::{Annotation, AnnotationType, Slice, Snippet, SourceAnnotation},
};
use snowc_lexer::Span;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorCode {
    E0000,
    E0010,
    E0020,
    Unknown,
}

impl ErrorCode {
    fn id(&self) -> String {
        format!("{:?}", self)
    }

    fn label(&self) -> String {
        format!("{}", self)
    }
}

impl From<&String> for ErrorCode {
    fn from(value: &String) -> Self {
        match value.as_str() {
            "E0000"=>Self::E0000, 
            "E0010"=>Self::E0010,
            "E0020"=>Self::E0020,
            _=>Self::Unknown,
        }
    }
}

impl fmt::Display for ErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::E0000 => write!(f, "expressions not allowed in global scope"),
            Self::E0010 => write!(f, "missing deliminator"),
            Self::E0020 => write!(f, "expected a condition for if statement"),
            Self::Unknown => write!(f, "place holder for a more correct error"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Error {
    pub id: String,
    pub label: String,
    pub span: Span,
    pub help: Option<String>,
    pub cause: Option<Box<Error>>,
}

impl Error {
    pub fn new(id: ErrorCode, span: Span) -> Self {
        Self {
            id: id.id(),
            label: id.label(),
            span,
            help: None,
            cause: None,
        }
    }

    pub fn new_with_cause(id: ErrorCode, span: Span, cause: Option<Error>) -> Self {
        Self {
            id: id.id(), label: id.label(), span, help: None, cause: cause.map(Box::new),
        }
    }

    pub fn help(mut self, msg: impl Into<String>) -> Self {
        self.help = Some(msg.into());
        self
    }

    pub fn get_error_code(&self) -> ErrorCode {
        ErrorCode::from(&self.id)
    }
}

// impl Errors {
//     pub fn push(&mut self, error: Error) {
//         self.errors.push(error)
//     }
//
//     pub fn pop_then_push(&mut self, error: Error) {
//         self.errors.pop();
//         self.errors.push(error)
//     }
//
//     pub fn pop(&mut self) -> Option<Error> {
//         self.errors.pop()
//     }
//
//     pub fn len(&self) -> usize {
//         self.errors.len()
//     }
//
//     pub fn is_empty(&self) -> bool {
//         self.errors.is_empty()
//     }
//
//     pub fn is_last_err_code(&self, codes: &[&str]) -> bool {
//         let Some(code) = self.last_err_code() else {
//             return false;
//         };
//         codes.contains(&code.as_str())
//     }
//
//     pub fn last_err_code(&self) -> Option<&String> {
//         let Some(error) = self.errors.last() else {
//             return None;
//         };
//         Some(&error.id)
//     }
// }

pub fn report(filename: &str, src: &str, error: &Error) {
    let Error { cause, .. } = &error;
    let snippet = snippet_builder(filename, src, &error);
    let dl = DisplayList::from(snippet);
    eprintln!("{}", dl);
    let Some(cause) = cause else {
        return;
    };
    report(filename, src, &cause);
}

fn snippet_builder<'a>(filename: &'a str, src: &'a str, error: &'a Error) -> Snippet<'a> {
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
