// TODO:[2] add comment support in swasm source files
// TODO:[4] make labels and code be on same line
// TODO:[5] Redo parser so that span can be accounted for
mod data;
mod directive;
mod item;
mod label;
mod location;
mod reg;
mod token_op;

pub use super::{opcode::OpCode, SymbolTable};
pub use data::Data;
pub use directive::Directive;
pub use item::{Item, Text};
pub use label::Label;
pub use location::Location;
pub use reg::Reg;
use snowc_error_messages::Error;
use snowc_error_messages::ErrorCode as ErrCode;
pub use snowc_lexer::{LexerDebug, Scanner, Span, Token};
pub use token_op::TokenOp;

#[derive(Debug, Clone, PartialEq, Eq)]
struct ErrorCode {
    id: String,
    label: String,
}

impl From<(&str, &str)> for ErrorCode {
    fn from((id, label): (&str, &str)) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
        }
    }
}

impl ErrCode for ErrorCode {
    fn id(&self) -> String {
        self.id.to_string()
    }

    fn label(&self) -> String {
        self.label.to_string()
    }
}

use std::iter::Peekable;
pub struct Parser<'a> {
    lexer: Peekable<Scanner<'a>>,
    errors: Option<Error>,
    last_span: Span,
}

impl<'a> Parser<'a> {
    pub fn new(src: &'a str) -> Self {
        let keywords = vec![
            "load", "push", "pop", "inc", "dec", "prti", "aloc", "setm", "eq", "neq",
            "gt", "geq", "lt", "leq", "add", "sub", "div", "mod", "mul", "prts", "jmp",
            "jeq", "jne", "hlt", "nop",
        ];
        let line_comment = (';', None);
        Self {
            lexer: Scanner::new_with_keywords(
                src,
                LexerDebug::Off,
                keywords,
                line_comment,
            )
            .peekable(),
            errors: None,
            last_span: Span::default(),
        }
    }
    fn next(&mut self) -> Token {
        let token = self.lexer.next().unwrap();
        let span = token.span();
        self.last_span = span;
        token
    }

    fn peek(&mut self) -> Token {
        self.lexer.peek().cloned().unwrap()
    }

    fn is_end(&mut self) -> bool {
        self.peek().is_eof()
    }

    fn parse_directive(&mut self) -> Result<Directive, Error> {
        let token = self.next();
        if !token.is_op_a(".") {
            let label = format!("directives start with '.' but found '{token:?}'");
            return Err(error("E0010", &label, self.last_span));
        }
        let token = self.next();
        if !token.is_id() {
            let label = format!("expected 'ident' but found '{token:?}'");
            return Err(error("E0011", &label, self.last_span));
        }
        let value = token.value();
        match value {
            "ascii" if self.peek().is_string() => {
                let token = self.next();
                Ok(Directive::Ascii(token.value().to_string()))
            }
            _ => {
                let label = format!("unknown directive call '{token:?}'");
                Err(error("E0012", &label, self.last_span))
            }
        }
    }

    fn parse_data(&mut self) -> Result<Vec<Data>, Error> {
        let mut data = vec![];
        while !self.is_end() {
            let Label { name, span, .. } = self.parse_label()?;
            let directive = self.parse_directive()?;
            data.push(Data {
                name,
                directive,
                span,
            });
            if self.peek().is_op_a(".") {
                return Ok(data);
            }
        }
        Ok(data)
    }

    fn parse_label(&mut self) -> Result<Label, Error> {
        if !self.peek().is_id() {
            let label = format!("missing label for data'");
            return Err(error("E0009", &label, self.last_span));
        }

        let token = self.next();
        let name = token.value().to_string();
        let span = token.span();
        let def = if self.peek().is_op_a(":") {
            self.next();
            true
        } else {
            false
        };
        Ok(Label { name, span, def })
    }

    fn parse_reg(&mut self) -> Result<Reg, Error> {
        let token = self.next();
        let line = token.span().line;
        let start = token.span().start;
        if !token.is_op_a("%") {
            let label = format!("regester start with '%' but found '{token:?}'");
            return Err(error("E0000", &label, self.last_span));
        }
        let token = self.next();
        let end = token.span().end;
        if !token.is_int() {
            let label = format!("regester missing number value found '{token:?}'");
            return Err(error("E0000", &label, self.last_span));
        }
        let span = Span::new(line, start, end);
        Ok(Reg(token.value().parse().unwrap()))
    }

    fn parse_1reg_u16<F>(&mut self, top: F, name: &str) -> Result<TokenOp, Error>
    where
        F: FnOnce(u8, u8, u8) -> TokenOp,
    {
        let Reg(r1) = self
            .parse_reg()
            .map_err(|e| reg_missing_for(name, self.last_span, e))?;
        let token = self.next();
        if !token.is_int() {
            let label = format!("{name} expects a int here '{token:?}'");
            return Err(error("E0000", &label, self.last_span));
        }
        let value = token.value().parse::<u32>().unwrap();
        let [_, _, b2, b1] = value.to_be_bytes();
        Ok(top(r1, b2, b1))
    }

    fn parse_1reg<F>(&mut self, top: F, name: &str) -> Result<TokenOp, Error>
    where
        F: FnOnce(u8) -> TokenOp,
    {
        let Reg(r1) = self
            .parse_reg()
            .map_err(|e| reg_missing_for(name, self.last_span, e))?;
        Ok(top(r1))
    }

    fn parse_2reg<F>(&mut self, top: F, name: &str) -> Result<TokenOp, Error>
    where
        F: FnOnce(u8, u8) -> TokenOp,
    {
        let Reg(r1) = self
            .parse_reg()
            .map_err(|e| reg_missing_for(name, self.last_span, e))?;
        let Reg(r2) = self
            .parse_reg()
            .map_err(|e| reg_missing_for(name, self.last_span, e))?;
        Ok(top(r1, r2))
    }

    fn parse_3reg<F>(&mut self, top: F, name: &str) -> Result<TokenOp, Error>
    where
        F: FnOnce(u8, u8, u8) -> TokenOp,
    {
        let Reg(r1) = self
            .parse_reg()
            .map_err(|e| reg_missing_for(name, self.last_span, e))?;
        let Reg(r2) = self
            .parse_reg()
            .map_err(|e| reg_missing_for(name, self.last_span, e))?;
        let Reg(r3) = self
            .parse_reg()
            .map_err(|e| reg_missing_for(name, self.last_span, e))?;
        Ok(top(r1, r2, r3))
    }

    fn parse_lab<F>(&mut self, top: F, name: &str) -> Result<TokenOp, Error>
    where
        F: FnOnce(Label) -> TokenOp,
    {
        let label = self
            .parse_label()
            .map_err(|e| label_missing_for(name, self.last_span, e))?;
        Ok(top(label))
    }

    fn parse_opcode(&mut self) -> Result<TokenOp, Error> {
        let token = self.next();
        let name = token.value().to_owned();
        match name.as_str() {
            "load" => self.parse_1reg_u16(TokenOp::Load, &name),
            "push" => self.parse_1reg(TokenOp::Push, &name),
            "pop" => self.parse_1reg(TokenOp::Pop, &name),
            "inc" => self.parse_1reg(TokenOp::Inc, &name),
            "dec" => self.parse_1reg(TokenOp::Dec, &name),
            "prti" => self.parse_1reg(TokenOp::Prti, &name),
            "aloc" => self.parse_1reg(TokenOp::Aloc, &name),
            "setm" => self.parse_2reg(TokenOp::Setm, &name),
            "eq" => self.parse_2reg(TokenOp::Eq, &name),
            "neq" => self.parse_2reg(TokenOp::Neq, &name),
            "gt" => self.parse_2reg(TokenOp::Gt, &name),
            "geq" => self.parse_2reg(TokenOp::Geq, &name),
            "lt" => self.parse_2reg(TokenOp::Lt, &name),
            "leq" => self.parse_2reg(TokenOp::Leq, &name),
            "add" => self.parse_3reg(TokenOp::Add, &name),
            "sub" => self.parse_3reg(TokenOp::Sub, &name),
            "div" => self.parse_3reg(TokenOp::Div, &name),
            "mod" => self.parse_3reg(TokenOp::Mod, &name),
            "mul" => self.parse_3reg(TokenOp::Mul, &name),
            "prts" => self.parse_lab(TokenOp::Prts, &name),
            "jmp" => self.parse_lab(TokenOp::Jmp, &name),
            "jeq" => self.parse_lab(TokenOp::Jeq, &name),
            "jne" => self.parse_lab(TokenOp::Jne, &name),
            "hlt" => Ok(TokenOp::Hlt),
            "nop" => Ok(TokenOp::Nop),
            _ => unreachable!("{:?}", token),
        }
    }

    fn parse_text(&mut self) -> Result<Vec<Text>, Error> {
        let mut text = vec![];
        while !self.is_end() {
            let label = self.parse_label().ok();
            let opcode = self.parse_opcode()?;
            text.push(Text { label, opcode });
        }
        Ok(text)
    }

    pub fn parse(mut self) -> Result<Vec<Item>, Error> {
        let mut items = vec![];
        while !self.is_end() {
            let token = self.next();
            if token.is_op_a(".") && self.peek().is_id_a("entry") {
                self.next();
                items.push(Item::EntryPoint(self.next()))
            } else if token.is_op_a(".") && self.peek().is_id_a("data") {
                self.next();
                items.push(Item::Data(self.parse_data()?));
            } else if token.is_op_a(".") && self.peek().is_id_a("text") {
                self.next();
                items.push(Item::Text(self.parse_text()?));
            }
        }
        if let Some(error) = self.errors {
            return Err(error);
        };
        Ok(items)
    }
}

fn reg_missing_for(name: &str, span: Span, e: Error) -> Error {
    let label = format!("regester missing for opcode '{name:?}'");
    error_with_cause("E0000", &label, span, e)
}

fn label_missing_for(name: &str, span: Span, e: Error) -> Error {
    let label = format!("label missing for opcode '{name:?}'");
    error_with_cause("E0000", &label, span, e)
}

pub fn error(id: &str, label: &str, span: Span) -> Error {
    let errorcode = ErrorCode::from((id, label));
    Error::new(errorcode, span)
}

fn error_with_cause(id: &str, label: &str, span: Span, error: Error) -> Error {
    let errorcode = ErrorCode::from((id, label));
    Error::new_with_cause(errorcode, span, Some(error))
}

#[test]
fn parse_load() {
    let ast = Parser::new(
        r#"
.entry main
.data
name: .ascii "Hello World"
.text ;; Hey
main:
load %1 100
hlt
"#,
    )
    .parse();
    dbg!(&ast);
    assert!(false);
}
