#![allow(unused)]
use std::fmt;
#[derive(Debug, Clone, Default)]
pub struct CharPos {
    pub chr: char,
    pub idx: usize,
    pub col: usize,
    pub row: usize,
    pub loc: String,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Pos {
    pub idx: usize,
    pub col: usize,
    pub row: usize,
}
impl Pos {
    pub fn new(idx: usize, col: usize, row: usize) -> Self {
        Self { idx, col, row }
    }
}
impl fmt::Display for Pos {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.col, self.row)
    }
}

impl From<&CharPos> for Pos {
    fn from(char_pos: &CharPos) -> Self {
        Self {
            idx: char_pos.idx,
            col: char_pos.col,
            row: char_pos.row,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Span {
    pub start: Pos,
    pub end: Pos,
    pub loc: String,
}
impl Span {
    pub fn new(start: Pos, end: Pos, loc: &str) -> Self {
        Self {
            start,
            end,
            loc: loc.to_string(),
        }
    }
}

impl fmt::Display for Span {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}..{}", self.loc, self.start, self.end)
    }
}

impl From<(Option<Span>, Option<Span>)> for Span {
    fn from((first, last): (Option<Span>, Option<Span>)) -> Self {
        let start = first.unwrap_or_default();
        let end = last.unwrap_or(start.clone());
        Self {
            start: start.start,
            end: end.end,
            loc: start.loc.to_string(),
        }
    }
}
impl From<(Span, Span)> for Span {
    fn from((start, end): (Span, Span)) -> Self {
        Self {
            start: start.start,
            end: end.end,
            loc: start.loc.to_string(),
        }
    }
}

impl<T> From<(Option<&Spanned<T>>, Option<&Spanned<T>>)> for Span
where
    T: fmt::Debug + PartialEq + Clone + fmt::Display,
{
    fn from((first, last): (Option<&Spanned<T>>, Option<&Spanned<T>>)) -> Self {
        let start = first.map(|s| s.span()).unwrap_or_default();
        let end = last.map(|s| s.span()).unwrap_or_default();
        Self {
            start: start.start,
            end: end.end,
            loc: start.loc.to_string(),
        }
    }
}

impl From<(&CharPos, &CharPos)> for Span {
    fn from((start, end): (&CharPos, &CharPos)) -> Self {
        Self {
            start: start.into(),
            end: end.into(),
            loc: start.loc.to_string(),
        }
    }
}
// pub type Spanned<T> = (T, Span);
#[derive(Clone, PartialEq)]
pub struct Spanned<T>
where
    T: PartialEq + Clone,
{
    pub node: T,
    pub span: Span,
}
impl<T> fmt::Debug for Spanned<T>
where
    T: PartialEq + Clone + fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Spanned").field("node", &self.node).finish()
    }
}

impl<T> Spanned<T>
where
    T: PartialEq + Clone,
{
    pub fn new(node: T, span: Span) -> Self {
        Self { node, span }
    }
}

impl<T> Spanned<T>
where
    T: fmt::Debug + PartialEq + Clone + fmt::Display,
{
    pub fn span(&self) -> Span {
        self.span.clone()
    }
}

impl<T> fmt::Display for Spanned<T>
where
    T: fmt::Debug + PartialEq + Clone + fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.span, self.node)
    }
}

impl<T> From<(T, Span)> for Spanned<T>
where
    T: PartialEq + Clone,
{
    fn from((node, span): (T, Span)) -> Self {
        Spanned { node, span }
    }
}
// impl<T> From<(T, Span)> for Spanned<T>
// where
//     T: fmt::Debug + PartialEq + Clone + fmt::Display,
// {
//     fn from((node, span): (Vec<Spanned<T>>, Span)) -> Self {
//         Spanned { node, span }
//     }
// }
