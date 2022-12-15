use super::style::{Field, Label, Shape, Style};
use std::fmt;

#[derive(Debug, Clone)]
pub struct StructBuilder {
    name: String,
    label: Label,
    shape: Shape,
    style: Style,
}

impl StructBuilder {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.into(),
            label: Label(vec![]),
            shape: Shape::None,
            style: Style::None,
        }
    }

    pub fn named_field(mut self, id: &str, field: &str) -> Self {
        let field = if field
            .chars()
            .nth(0)
            .map(|c| c.is_ascii_punctuation())
            .unwrap_or(false)
        {
            format!("\\{field}")
        } else {
            field.to_string()
        };
        self.label.0.push(Field::Named(id.into(), field.into()));
        self.shape(Shape::Record)
    }
    pub fn field(mut self, field: &str) -> Self {
        self.label.0.push(Field::Name(field.into()));
        self
    }

    pub fn shape(mut self, shape: Shape) -> Self {
        self.shape = shape;
        self
    }

    pub fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }
}

impl fmt::Display for StructBuilder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self {
            name,
            shape,
            style,
            label,
        } = &self;
        write!(f, "{name} [ {shape}{style}{label} ];\n")
    }
}

#[test]
fn graphviz_struct_record_function() {
    let left = StructBuilder::new("name")
        .field("1")
        .field("2")
        .named_field("field", "3")
        .shape(Shape::Record)
        .style(Style::Rounded);
    let right = r#"name [ shape=record,
style=rounded,
label="1|2|<field> 3" ];"#;
    assert_eq!(left.to_string(), right);
}
