#![allow(unused)]
use super::{Expr, Span, TypeInfo};
use std::collections::HashMap;

#[derive(Debug, Clone)]
enum Type {
    Int,
    Float,
    Bool,
    String,
    Char,
    Array(Box<Type>),
    Function(Vec<Type>, Box<Type>),
}

impl From<&TypeInfo> for Type {
    fn from(type_info: &TypeInfo) -> Self {
        match type_info {
            TypeInfo::Int => Self::Int,
            TypeInfo::Float => Self::Float,
            TypeInfo::Bool => Self::Bool,
            TypeInfo::String => Self::String,
            TypeInfo::Char => Self::Char,
            TypeInfo::Array(t) => Self::Array(Box::new(Self::from(&**t))),
            TypeInfo::Custom(_) => todo!(),
        }
    }
}

#[derive(Debug, Clone)]
struct Param {
    name: String,
    type_info: Box<Type>,
}

#[derive(Debug, Clone)]
struct Function {
    name: String,
    params: Vec<Type>,
    return_type: Box<Type>,
}

#[derive(Debug, Default)]
struct SymbolTable {
    params: HashMap<String, Param>,
    function: HashMap<String, Function>,
}

impl SymbolTable {
    fn insert_function(&mut self, name: impl Into<String>, function: Function) {
        self.function.insert(name.into(), function);
    }

    fn insert_param(&mut self, name: impl Into<String>, param: Param) {
        self.params.insert(name.into(), param);
    }

    fn lookup_function(&self, name: &str) -> Option<&Function> {
        self.function.get(name)
    }

    fn lookup_param(&self, name: &str) -> Option<&Param> {
        self.params.get(name)
    }
}

fn walk(expr: &Expr, table: &mut SymbolTable) {
    match expr {
        Expr::Atom(atom) => match atom {
            snowc_parse::Atom::Ident(name) => {}
        },
        Expr::Func(name, type_info, closure, _) => {
            table.insert_function(
                name,
                Function {
                    name: name.clone(),
                    params: type_info.iter().map(Type::from).collect(),
                    return_type: Box::new(Type::Int),
                },
            );
            walk(closure, table);
        }
        _ => unreachable!("{:?}", expr),
    }
}

#[test]
fn test_table() {
    use pretty_assertions::assert_eq;
    use snowc_parse::parse;
    let src = include_str!("./../../../samples/other.snow");
    let ast = parse(src).unwrap();
    let mut table = SymbolTable::default();
    for expr in ast.iter() {
        walk(expr, &mut table);
    }

    assert_eq!(vec![12], vec![]);
}
