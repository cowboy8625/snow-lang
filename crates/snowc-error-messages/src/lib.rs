use annotate_snippets::{
    display_list::{DisplayList, FormatOptions},
    snippet::{Annotation, AnnotationType, Slice, Snippet, SourceAnnotation},
};

#[derive(Debug)]
pub struct Error {
    pub id: String,
    pub label: String,
    pub span: std::ops::Range<usize>,
}

pub fn report(filename: &str, src: &str, errors: &[Error]) {
    let lines = src.lines().enumerate().fold(vec![], |mut acc, (idx, n)| {
        let line_count = n.chars().count() + 1;
        let Some((_, span)) = acc.last() else {
            acc.push((idx, 0..line_count));
            return acc;
        };
        acc.push((idx, span.end..span.end + line_count));
        acc
    });
    let snippets = errors
        .iter()
        .filter_map(|error| {
            lines
                .iter()
                .find(|(_, span)| span.contains(&error.span.start))
                .map(|(idx, _)| (idx, error))
        })
        .map(|(line, error)| snippet_builder(filename, *line, src, error))
        .collect::<Vec<Snippet>>();

    for snippet in snippets {
        let dl = DisplayList::from(snippet);
        eprintln!("{}", dl);
    }
}

fn snippet_builder<'a>(
    filename: &'a str,
    line: usize,
    src: &'a str,
    error: &'a Error,
) -> Snippet<'a> {
    Snippet {
        title: Some(Annotation {
            label: Some(&error.label),
            id: Some(&error.id),
            annotation_type: AnnotationType::Error,
        }),
        footer: vec![],
        slices: vec![Slice {
            source: src,
            line_start: line,
            origin: Some(filename),
            fold: true,
            annotations: vec![
                SourceAnnotation {
                    label: "",
                    annotation_type: AnnotationType::Error,
                    range: (error.span.start, error.span.end),
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
