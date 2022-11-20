use crossterm::style::{Color, Stylize};
use std::ops::Range;

pub type Span = Range<usize>;
pub fn report(src: &str, span: Span, error: &str) -> String {
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
        format!("|:   {}", &src[span]),
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