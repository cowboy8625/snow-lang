use std::{fmt, ops::Range};

#[derive(Default, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Span {
    pub idx_start: usize,
    pub idx_end: usize,
    pub row_start: usize,
    pub col_start: usize,
    pub row_end: usize,
    pub col_end: usize,
}

impl Span {
    pub fn new(
        idx_start: usize,
        idx_end: usize,
        row_start: usize,
        col_start: usize,
        row_end: usize,
        col_end: usize,
    ) -> Self {
        Self {
            idx_start,
            idx_end,
            row_start,
            col_start,
            row_end,
            col_end,
        }
    }
    pub fn right_shift(&mut self, ch: char) {
        if ch == '\0' {
            return;
        }
        let len = ch.to_string().as_bytes().len();
        if ch == '\n' {
            self.row_end += 1;
            self.col_end = 0;
        } else {
            self.col_end += len;
        }
        self.idx_end += len;
    }

    pub fn reset(&mut self, last_chr_len: Option<usize>) {
        let len = last_chr_len.unwrap_or(0);
        self.idx_start = self.idx_end.saturating_sub(len);
        self.row_start = self.row_end;
        self.col_start = self.col_end.saturating_sub(len);
    }

    pub fn range(&self) -> Range<usize> {
        self.idx_start..self.idx_end
    }

    #[allow(unused)]
    pub fn len(&self) -> usize {
        self.idx_end - self.idx_start
    }
}

impl From<(Span, Span)> for Span {
    fn from((x, y): (Self, Self)) -> Self {
        let idx_start: usize = x.idx_start;
        let idx_end: usize = y.idx_end;
        let row_start: usize = x.row_start;
        let col_start: usize = y.col_start;
        let row_end: usize = x.row_end;
        let col_end: usize = y.col_end;
        Self::new(idx_start, idx_end, row_start, row_end, col_start, col_end)
    }
}

impl fmt::Debug for Span {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "({},{})->({},{})",
            self.row_start, self.col_start, self.row_end, self.col_end
        )
    }
}
