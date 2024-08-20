#![allow(dead_code)]
#![allow(warnings)]
use crate::front_end::{Atom, Expr, Operator};

#[derive(Debug, Clone, PartialEq)]
pub struct Module {
    pub functions: Vec<Function>,
    pub globals: Vec<Global>,
    pub enums: Vec<Enum>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Function {
    pub name: String,
    pub params: Vec<(String, Type)>,
    pub return_type: Option<Type>,
    pub body: Block,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Block {
    pub instructions: Vec<Instruction>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Instruction {
    // Control flow
    If {
        condition: Block,
        then_branch: Block,
        else_branch: Option<Block>,
    },
    Call {
        function_name: String,
        arguments: Vec<Value>,
    },
    Return(Value),
    // Arithmetic operations
    Add,
    Sub,
    Mul,
    Div,
    GreaterThan,
    // Memory operations
    Load(Value),
    Store(Value, Value),
    // Stack operations
    Push(Value),
    Pop(Value),
    // etc.....?
}

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Int,
    Bool,
    /// For enums and other user-defined types
    Custom(String),
}

impl From<String> for Type {
    fn from(name: String) -> Self {
        match name.as_str() {
            "Int" => Type::Int,
            "Bool" => Type::Bool,
            _ => Type::Custom(name),
        }
    }
}

impl From<&String> for Type {
    fn from(name: &String) -> Self {
        Self::from(name.clone())
    }
}

impl From<&str> for Type {
    fn from(name: &str) -> Self {
        Self::from(name.clone())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Enum {
    pub name: String,
    pub variants: Vec<EnumVariant>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct EnumVariant {
    pub name: String,
    /// The types associated with the variant
    pub types: Vec<Type>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Global {
    pub name: String,
    pub typ: Type,
    pub value: Value,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Int(i32),
    Bool(bool),
    Char(char),
    /// References a variable by name
    Variable(String),
    String(String),
}
