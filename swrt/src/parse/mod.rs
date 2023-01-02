// TODO:[2] add comment support in swasm source files
// TODO:[4] make labels and code be on same line
// TODO:[5] Redo parser so that span can be accounted for
mod data;
mod directive;
mod entry;
mod label;
mod location;
mod reg;
mod token_op;

pub use super::{error::Error, opcode::OpCode, SymbolTable};
pub use data::Data;
pub use directive::Directive;
pub use entry::Entry;
pub use label::Label;
pub use location::Location;
pub use reg::Reg;
pub use token_op::TokenOp;
