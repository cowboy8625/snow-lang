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

pub fn report(filename: &str, src: &str, errors: &[Error]) {
    // let lines = src.lines().enumerate().fold(vec![], |mut acc, (idx, n)| {
    //     let line_count = n.chars().count() + 1;
    //     let Some((_, span)) = acc.last() else {
    //         acc.push((idx, 0..line_count));
    //         return acc;
    //     };
    //     acc.push((idx, span.end..span.end + line_count));
    //     acc
    // });
    let snippets = errors
        .iter()
        // .map(|error| {
        //     lines
        //         .iter()
        //         .find(|(_, span)| span.contains(&error.span.start))
        //         .map_or_else(
        //             || (lines.last().unwrap().0, error),
        //             |(idx, _)| (*idx, error),
        //         )
        // })
        // .map(|(line, error)| snippet_builder(filename, line, src, error))
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
