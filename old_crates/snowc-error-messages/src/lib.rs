use crossterm::style::{Color, Stylize};
use std::error::Error;
use std::ops::Range;

pub type Span = Range<usize>;
pub fn report(src: &str, _errors: Box<dyn Error>) -> String {
    let mut start = 0;
    let mut end = 0;
    for (_, line) in src.lines().enumerate() {
        end += line.chars().count();
        if end >= span.end {
            break;
        }
        start = end;
    }
    eprintln!("span: {span:?}, start: {start}, end: {end}");
    eprintln!("'{}'", &src[span.clone()]);
    let space = span.start + span.start.saturating_sub(20);
    let span = (span.start.saturating_sub(20))..((span.end + 20).min(src.len()));
    let underline = span
        .clone()
        .map(|_| "^")
        .collect::<String>()
        .with(Color::Red);
    format!(
        "{}\r\n{}\r\n{}\r\n",
        format!("{error}"),
        format!("|:   {}", &src[start..end]),
        format!("|:   {:>space$}{}", "", underline)
    )
}

#[cfg(test)]
mod tests {
    // use super::*;
    #[test]
    fn format_report() {
        assert_eq!("", "");
    }
}
