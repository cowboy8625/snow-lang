use snowc_error_messages::ErrorCode as ErrCode;
use std::fmt;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorCode {
    E0000,
    E0010,
    E0020,
    Unknown,
}

impl ErrCode for ErrorCode {
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
            "E0000" => Self::E0000,
            "E0010" => Self::E0010,
            "E0020" => Self::E0020,
            _ => Self::Unknown,
        }
    }
}

// impl From<( String, String, Span, Error)> for Error {
//     fn from((id, label, span, cause): (String, String, Span, Error)) -> Self {
//         Self::new_with_cause(id, label, span, Some(Error))
//     }
// }

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
