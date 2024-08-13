#![allow(dead_code)]
#![allow(warnings)]
use crate::front_end::*;

#[derive(Debug, Clone, PartialEq)]
pub struct IRModule {
    pub functions: Vec<IRFunction>,
    pub globals: Vec<IRGlobal>,
    pub types: Vec<IRType>,
    pub enums: Vec<IREnum>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct IRFunction {
    pub name: String,
    pub params: Vec<(String, IRType)>,
    pub return_type: IRType,
    pub body: IRBlock,
}

#[derive(Debug, Clone, PartialEq)]
pub struct IRBlock {
    pub instructions: Vec<IRInstruction>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum IRInstruction {
    BinaryOp {
        left: IRValue,
        op: Operator,
        right: IRValue,
    },
    PrefixOp {
        op: Operator,
        right: IRValue,
    },
    // Control flow
    If {
        condition: IRValue,
        then_branch: IRBlock,
        else_branch: Option<IRBlock>,
    },
    Call {
        function_name: String,
        arguments: Vec<IRValue>,
    },
    Return(IRValue),
    // Arithmetic operations
    Add(IRValue, IRValue),
    Sub(IRValue, IRValue),
    Mul(IRValue, IRValue),
    Div(IRValue, IRValue),
    // Memory operations
    Load(IRValue),
    Store(IRValue, IRValue),
    // etc.
}

#[derive(Debug, Clone, PartialEq)]
pub enum IRType {
    Int,
    Bool,
    Custom(String), // For enums and other user-defined types
}

impl From<String> for IRType {
    fn from(name: String) -> Self {
        match name.as_str() {
            "Int" => IRType::Int,
            "Bool" => IRType::Bool,
            _ => IRType::Custom(name),
        }
    }
}

impl From<&String> for IRType {
    fn from(name: &String) -> Self {
        Self::from(name.clone())
    }
}

impl From<&str> for IRType {
    fn from(name: &str) -> Self {
        Self::from(name.clone())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct IREnum {
    pub name: String,
    pub variants: Vec<IREnumVariant>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct IREnumVariant {
    pub name: String,
    pub types: Vec<IRType>, // The types associated with the variant
}

#[derive(Debug, Clone, PartialEq)]
pub struct IRGlobal {
    pub name: String,
    pub typ: IRType,
    pub value: IRValue,
}

#[derive(Debug, Clone, PartialEq)]
pub enum IRValue {
    Int(i32),
    Bool(bool),
    Char(char),
    Variable(String), // References a variable by name
}

pub trait ExprVisitor {
    fn visit_function(
        &mut self,
        name: String,
        params: Vec<String>,
        signature: Vec<String>,
        body: &Expr,
    ) -> IRFunction;

    fn visit_enum(
        &mut self,
        name: String,
        type_args: Vec<String>,
        variants: &Vec<Vec<Expr>>,
    ) -> IREnum;

    fn visit_enum_variant(&mut self, variant: &Vec<Expr>) -> IREnumVariant;

    fn visit_binary_op(
        &mut self,
        left: &Expr,
        op: Operator,
        right: &Expr,
    ) -> IRInstruction;

    fn visit_prefix_op(&mut self, op: Operator, right: &Expr) -> IRInstruction;

    fn visit_if(
        &mut self,
        condition: &Expr,
        then_branch: &Expr,
        else_branch: &Option<Expr>,
    ) -> IRInstruction;

    fn visit_atom(&mut self, atom: &Atom) -> IRValue {
        match atom {
            Atom::Int(i) => IRValue::Int(*i),
            Atom::Bool(b) => IRValue::Bool(*b),
            Atom::Char(c) => IRValue::Char(*c),
        }
    }

    fn visit_expr(&mut self, expr: &Expr) -> IRBlock {
        let mut instructions = Vec::new();
        match expr {
            Expr::Atom(atom) => {
                let instruction = self.visit_atom(atom);
                instructions.push(instruction);
            }
            Expr::Identifier(name) => {
                let instruction = IRValue::Variable(name.to_string());
                instructions.push(instruction);
            }
            Expr::PrefixOp { op, right } => todo!(),
            Expr::BinaryOp { left, op, right } => todo!(),
            Expr::Function {
                name,
                params,
                signature,
                body,
            } => todo!(),
            Expr::Enum {
                name,
                type_args,
                variants,
            } => todo!(),
            Expr::If {
                condition,
                then_branch,
                else_branch,
            } => todo!(),
        }

        IRBlock { instructions }
    }
}
