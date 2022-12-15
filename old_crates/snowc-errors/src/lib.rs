use std::error::Error;
use std::fmt;
use std::ops::Range;
pub type CResult<T> = Result<T, Box<dyn std::error::Error>>;

#[derive(Debug, Clone)]
pub struct CompilerError {
    msg: String,
    span: Range<usize>,
}

impl CompilerError {
    pub fn new(msg: String, span: Range<usize>) -> Self {
        let msg = format!("{}:{} {}", span.start, span.end, msg);
        Self { msg, span }
    }

    pub fn span(&self) -> Range<usize> {
        self.span.clone()
    }
}

impl fmt::Display for CompilerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.msg)
    }
}

impl Error for CompilerError {
    fn description(&self) -> &str {
        &self.msg
    }
}

#[macro_export]
macro_rules! bail {
    ($span:expr $(, $arg:expr)* $(,)?) => {{
        let msg = format!($($arg, )*);
        return Err(Box::new(CompilerError::new(msg, $span)));
    }};
}
