use std::fmt;

#[derive(Debug, Clone)]
pub enum Shape {
    Record,
    None,
}
impl fmt::Display for Shape {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Record => write!(f, "shape=record,\n"),
            Self::None => write!(f, ""),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Style {
    Rounded,
    None,
}

impl fmt::Display for Style {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Rounded => write!(f, "style=rounded,\n"),
            Self::None => write!(f, ""),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Field {
    Named(String, String),
    Name(String),
}

impl fmt::Display for Field {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Named(id, name) => write!(f, "<{id}> {name}"),
            Self::Name(name) => write!(f, "{name}"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Label(pub Vec<Field>);

impl fmt::Display for Label {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let fields = self.0.iter().fold("".into(), |last: String, field| {
            if last.is_empty() {
                format!("{field}")
            } else {
                format!("{last}|{field}")
            }
        });
        write!(f, "label=\"{fields}\"")
    }
}
