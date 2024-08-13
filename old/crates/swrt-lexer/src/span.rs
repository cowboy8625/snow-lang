#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct Span {
    pub line: usize,
    pub start: usize,
    pub end: usize,
}

impl Span {
    pub fn new(line: usize, start: usize, end: usize) -> Self {
        Self { line, start, end }
    }

    pub fn shift_right(&mut self) {
        self.start += 1;
    }

    pub fn new_line(&mut self) {
        self.line += 1;
        self.start += 1;
    }

    pub fn range(&self) -> std::ops::Range<usize> {
        self.start..self.end
    }
}
