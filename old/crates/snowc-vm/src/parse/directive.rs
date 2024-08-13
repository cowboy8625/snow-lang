#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Directive {
    Ascii(String),
}

impl Directive {
    pub fn size(&self) -> usize {
        match self {
            Self::Ascii(_) => self.as_bytes().len(),
        }
    }
    pub fn as_bytes(&self) -> Vec<u8> {
        match self {
            Self::Ascii(string) => {
                let mut b = vec![];
                let mut chars = string.chars().peekable();
                // FIXME: Not a so happy about this.
                while let Some(c) = chars.next() {
                    match c {
                        '\\' if chars.peek() == Some(&'n') => {
                            chars.next();
                            b.push(10);
                        }
                        _ => b.push(c as u8),
                    }
                }
                b.push(0);
                b
            }
        }
    }
}
