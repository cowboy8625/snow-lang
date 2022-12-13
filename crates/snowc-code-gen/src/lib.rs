// #![allow(dead_code)]
// #![allow(unused_imports)]
// // use snowc_errors;
// use snowc_parse::{Atom, Expr, Op};
// use std::fmt;
//
// trait ExprIs {
//     fn is_atom(&self) -> bool;
//     fn is_unary(&self) -> bool;
//     fn is_binary(&self) -> bool;
//     fn is_if_else(&self) -> bool;
//     fn is_closure(&self) -> bool;
//     fn is_func(&self) -> bool;
//     fn is_dec(&self) -> bool;
//     fn is_type(&self) -> bool;
// }
//
// impl ExprIs for Expr {
//     fn is_atom(&self) -> bool {
//         match self {
//             Self::Atom(_) => true,
//             _ => false,
//         }
//     }
//     fn is_unary(&self) -> bool {
//         match self {
//             Self::Unary(_, _) => true,
//             _ => false,
//         }
//     }
//     fn is_binary(&self) -> bool {
//         match self {
//             Self::Binary(_, _, _) => true,
//             _ => false,
//         }
//     }
//     fn is_if_else(&self) -> bool {
//         match self {
//             Self::IfElse(_, _, _) => true,
//             _ => false,
//         }
//     }
//     fn is_closure(&self) -> bool {
//         match self {
//             Self::Clouser(_, _) => true,
//             _ => false,
//         }
//     }
//     fn is_func(&self) -> bool {
//         match self {
//             Self::Func(_, _) => true,
//             _ => false,
//         }
//     }
//     fn is_dec(&self) -> bool {
//         match self {
//             Self::TypeDec(_, _) => true,
//             _ => false,
//         }
//     }
//     fn is_type(&self) -> bool {
//         match self {
//             Self::Type(_, _) => true,
//             _ => false,
//         }
//     }
// }
//
// fn atom(a: &Atom) -> String {
//     a.to_string()
// }
//
// fn unary(op: &Op, lhs: &Expr) -> String {
//     format!("{op}{lhs}")
// }
//
// fn binary(op: &Op, lhs: &Expr, rhs: &Expr) -> String {
//     format!("{lhs}{op}{rhs}")
// }
//
// fn if_else(condition: &Expr, branch1: &Expr, branch2: &Expr) -> String {
//     let c = code_gen(condition);
//     let b1 = code_gen(branch1);
//     let b2 = code_gen(branch2);
//     format!("if {c} {{{b1}}} else {{{b2}}}")
// }
//
// fn app(head: &Expr, tail: &[Expr]) -> String {
//     let name = code_gen(head);
//     let args = tail
//         .iter()
//         .map(|i| code_gen(i))
//         .collect::<Vec<String>>()
//         .join(",");
//     format!("{name}({args})")
// }
//
// fn clouser(head: &Expr, tail: &Expr) -> String {
//     let arg = code_gen(head);
//     let next = code_gen(tail);
//     if tail.is_atom() {
//         return format!("{arg}, {next}");
//     }
//     format!("{arg}) -> {{ {next}")
// }
//
// fn function_builder(name: &str, body: &Expr) -> String {
//     format!("fn {name}( {} }}", code_gen(body))
// }
//
// fn code_gen(expr: &Expr) -> String {
//     match expr {
//         Expr::Atom(a) => atom(a),
//         Expr::Unary(op, lhs) => unary(op, lhs),
//         Expr::Binary(op, lhs, rhs) => binary(op, lhs, rhs),
//         Expr::IfElse(condition, branch1, branch2) => if_else(condition, branch1, branch2),
//         Expr::App(head, tail) => app(head, tail),
//         Expr::Clouser(head, tail) => clouser(head, tail),
//         Expr::Func(name, body) => function_builder(name, body),
//         _ => unreachable!(),
//     }
// }
//
// pub fn code_gen_file(_filename: &str, ast: &[Expr]) -> String {
//     let mut code = String::new();
//     for expr in ast.iter() {
//         code.push_str(&code_gen(expr));
//     }
//     code
// }
//
// // struct StructBuilder;
// // struct ImplBuilder;
// // struct TraitBuilder;
// struct FunctionBuilder {
//     name: String,
//     params: Vec<String>,
//     body: String,
// }
// impl FunctionBuilder {
//     fn new(name: &str) -> Self {
//         Self {
//             name: name.to_string(),
//             params: vec![],
//             body: String::new(),
//         }
//     }
//
//     fn with_arg(mut self, name: &str) -> Self {
//         self.params.push(name.to_string());
//         self
//     }
//
//     fn with_body(mut self, body: &str) -> Self {
//         self.body = body.to_string();
//         self
//     }
// }
//
// impl fmt::Display for FunctionBuilder {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         let Self { name, params, body } = &self;
//         let args = params
//             .iter()
//             .map(|i| format!("{i}: i32"))
//             .collect::<Vec<String>>();
//         let mut args = format!("{args:?}").replace("\"", "");
//         args.pop();
//         args.remove(0);
//         let result = format!(
//             "fn {name}({args}) -> i32 {{
//     {body}
// }}"
//         );
//         write!(f, "{result}")
//     }
// }
//
// #[test]
// fn code_gen_test() {
//     use snowc_parse::parse;
//     let ast = parse("fn add x y = x + y;", true).unwrap();
//     let right = code_gen_file("", &ast);
//     let left = r#"fn add(x: i32, y: i32) -> i32 {
//     x + y
// }"#;
//     assert_eq!(left, right);
// }
// //
// // #[macro_export]
// // macro_rules! snowc {
// //     // Function Def
// //     (fn $name:ident $($args:ident )+ = $body:tt) => {
// //         {
// //             let mut args = Vec::new();
// //             $(
// //                 args.push(stringify!($args));
// //             )+
// //             let args = args.iter().map(|i| format!("{i} ")).collect::<String>();
// //             format!("fn {} {}= {};", stringify!($name), args, snowc!($body))
// //         }
// //     };
// //     // Call
// //     ($name:ident $($args:expr)+) => {
// //         {
// //             let mut args = Vec::new();
// //             $(
// //                 args.push(stringify!($args));
// //             )+
// //             let args = args.iter().map(|i| format!("{i} ")).collect::<String>();
// //             format!("{} {}", stringify!($name), args.trim_end())
// //
// //         }
// //
// //     };
// //     ($e:expr) => {
// //             format!("{}{}", stringify!($e), snowc!($tail))
// //     };
// //     (;) => {
// //         ";".to_string()
// //     };
// // }
// //
// // #[test]
// // fn snowc_call() {
// //     let right = snowc!(add 1 2);
// //     eprintln!("{right}");
// //     let left = "add 1 2 ";
// //     assert_eq!(left, right);
// // }
// // #[test]
// // fn test_func_macro_add() {
// //     let right = snowc!(fn add x y = x + y);
// //     eprintln!("{right}");
// //     let left = "fn add x y = x + y;";
// //     assert_eq!(left, right);
// //
// //     let right = snowc!(fn add x y = add x y);
// //     eprintln!("{right}");
// //     let left = "fn add x y = add x y;";
// //     assert_eq!(left, right);
// // }
// //
// // #[test]
// // fn test_func_macro_main() {
// //     let right = func!(fn main = add 1 2;);
// //     eprintln!("{right}");
// //     let left = "fn main = add 1 2;";
// //     assert_eq!(left, right);
// // }
