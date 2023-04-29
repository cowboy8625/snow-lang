// use trace::trace;
// trace::init_depth_var!();

use std::collections::HashMap;

use snowc_parse::{Atom, Expr, Op};
trait Visitor {
    type Item;
    fn visit_atom(
        &mut self,
        atom: &Atom,
        local_env: &mut Env,
        global_env: &Env,
    ) -> Self::Item;
    fn visit_expr(
        &mut self,
        expr: &Expr,
        local_env: &mut Env,
        global_env: &Env,
    ) -> Self::Item;
}

type Env = HashMap<String, Expr>;

pub struct Interpreter;
impl Interpreter {
    // #[trace]
    pub fn new(ast: Vec<Expr>) {
        let mut global_env = Env::new();
        let mut interpreter = Self;
        let mut main_idx: Option<usize> = None;
        for (idx, expr) in ast.iter().enumerate() {
            match expr {
                Expr::Func(name, ..) if name == "main" => {
                    main_idx = Some(idx);
                }
                Expr::Func(name, closure, ..) => {
                    global_env.insert(name.to_string(), *closure.clone());
                }
                Expr::TypeDec(..) => {}
                _ => unreachable!(),
            }
        }

        let Some(idx) = main_idx else {
            eprintln!("missing main function");
            std::process::exit(1);
        };

        let main_function = &ast[idx];
        let Expr::Func(_, closure, ..) = main_function else {
            panic!("really bad things are happening");
        };
        let output = interpreter.visit_expr(&closure, &mut Env::new(), &global_env);
        println!("[OUTPUT]: {}", output);
    }
    // #[trace(prefix_enter="[ENTER]", prefix_exit="[EXIT]")]
    fn unary(
        &mut self,
        op: &Op,
        rhs: &Expr,
        local_env: &mut Env,
        global_env: &Env,
    ) -> Atom {
        let atom = self.visit_expr(rhs, local_env, global_env);
        match (op, atom) {
            (Op::Minus, Atom::Int(int)) => Atom::Int(-int),
            (Op::Not, Atom::Bool(b)) => Atom::Bool(!b),
            _ => unimplemented!("for operator '{op:?}'"),
        }
    }
    // #[trace(prefix_enter="[ENTER]", prefix_exit="[EXIT]")]
    fn binary(
        &mut self,
        op: &Op,
        lhs: &Expr,
        rhs: &Expr,
        local_env: &mut Env,
        global_env: &Env,
    ) -> Atom {
        let lhs_atom = self.visit_expr(lhs, local_env, global_env);
        let rhs_atom = self.visit_expr(rhs, local_env, global_env);
        match (op, lhs_atom, rhs_atom) {
            (Op::Plus, Atom::Int(lhs), Atom::Int(rhs)) => Atom::Int(lhs + rhs),
            (Op::Minus, Atom::Int(lhs), Atom::Int(rhs)) => Atom::Int(lhs - rhs),
            (Op::Mult, Atom::Int(lhs), Atom::Int(rhs)) => Atom::Int(lhs * rhs),
            (Op::Div, Atom::Int(lhs), Atom::Int(rhs)) => Atom::Int(lhs / rhs),
            (Op::Grt, Atom::Int(lhs), Atom::Int(rhs)) => Atom::Bool(lhs > rhs),
            (Op::GrtEq, Atom::Int(lhs), Atom::Int(rhs)) => Atom::Bool(lhs >= rhs),
            (Op::Les, Atom::Int(lhs), Atom::Int(rhs)) => Atom::Bool(lhs < rhs),
            (Op::LesEq, Atom::Int(lhs), Atom::Int(rhs)) => Atom::Bool(lhs <= rhs),
            (Op::Neq, Atom::Int(lhs), Atom::Int(rhs)) => Atom::Bool(lhs != rhs),
            (op, r, l) => unimplemented!("{l:?} {op:?} {r:?}"),
        }
    }

    // #[trace(prefix_enter="[ENTER]", prefix_exit="[EXIT]")]
    fn closure(
        &mut self,
        head: &Expr,
        tail: &Expr,
        local_env: &mut Env,
        global_env: &Env,
    ) -> Atom {
        self.visit_expr(tail, local_env, global_env);
        self.visit_expr(head, local_env, global_env)
    }

    // #[trace(prefix_enter="[ENTER]", prefix_exit="[EXIT]")]
    fn ifelse(
        &mut self,
        condition: &Expr,
        then: &Expr,
        r#else: &Expr,
        local_env: &mut Env,
        global_env: &Env,
    ) -> Atom {
        match self.visit_expr(condition, local_env, global_env) {
            Atom::Bool(true) => self.visit_expr(then, local_env, global_env),
            Atom::Bool(false) => self.visit_expr(r#else, local_env, global_env),
            _ => unreachable!(),
        }
    }

    // #[trace(prefix_enter="[ENTER]", prefix_exit="[EXIT]")]
    fn get_func_params(
        &self,
        closure: &Expr,
        global_env: &Env,
        local_env: &mut Env,
        params: &mut Vec<String>,
    ) {
        match closure {
            Expr::Closure(head, tail, ..) => match **head {
                Expr::Atom(Atom::Id(ref name), ..) => {
                    params.push(name.clone());
                    self.get_func_params(tail, global_env, local_env, params)
                }
                _ => {}
            },
            _ => {}
        }
    }

    // #[trace(prefix_enter="[ENTER]", prefix_exit="[EXIT]")]
    fn app(
        &mut self,
        name: &Box<Expr>,
        args: &[Expr],
        local_env: &mut Env,
        global_env: &Env,
    ) -> Atom {
        let Expr::Atom(Atom::Id(ref name), span) = **name else {
            panic!("expected a Id but found '{name:?}'");
        };
        if ["print_int", "print_bool", "print_str"].contains(&name.as_str()) {
            let e = self.visit_expr(&args[0], local_env, global_env);
            println!("{}", e);
            return e;
        }
        let Some(func) = global_env.get(name) else {
            panic!("ERROR {}:{}: {name} is not implemented yet", span.start, span.end);
        };
        let mut params = Vec::new();
        self.get_func_params(func, global_env, local_env, &mut params);
        for (param, arg) in params.iter().zip(args) {
            let span = arg.span();
            let atom = self.visit_expr(arg, local_env, global_env);
            let expr = Expr::Atom(atom, span);
            local_env.insert(param.to_string(), expr.clone());
        }
        self.visit_expr(func, local_env, global_env)
    }
}

impl Visitor for Interpreter {
    type Item = Atom;
    // #[trace(prefix_enter="[ENTER]", prefix_exit="[EXIT]")]
    fn visit_atom(
        &mut self,
        atom: &Atom,
        local_env: &mut Env,
        global_env: &Env,
    ) -> Self::Item {
        match atom {
            Atom::Id(name) => {
                let expr = global_env
                    .get(name)
                    .or(local_env.get(name))
                    .cloned()
                    .unwrap();
                self.visit_expr(&expr, local_env, global_env)
            }
            _ => atom.clone(),
        }
    }
    // #[trace(prefix_enter="[ENTER]", prefix_exit="[EXIT]")]
    fn visit_expr(
        &mut self,
        expr: &Expr,
        local_env: &mut Env,
        global_env: &Env,
    ) -> Self::Item {
        match expr {
            Expr::Atom(atom, ..) => self.visit_atom(atom, local_env, global_env),
            Expr::Unary(op, rhs, ..) => self.unary(op, rhs, local_env, global_env),
            Expr::Binary(op, lhs, rhs, ..) => {
                self.binary(op, lhs, rhs, local_env, global_env)
            }
            Expr::IfElse(condision, then, r#else, ..) => {
                self.ifelse(condision, then, r#else, local_env, global_env)
            }
            Expr::Closure(head, tail, ..) => {
                self.closure(head, tail, local_env, global_env)
            }
            Expr::App(name, args, ..) => self.app(name, args, local_env, global_env),
            Expr::Enum(..) => unimplemented!(),
            Expr::Func(..) => unreachable!(),
            Expr::TypeDec(..) => unreachable!(),
            Expr::Error(..) => unreachable!(),
        }
    }
}

// #[derive(Debug, Clone, Hash)]
// pub enum Atom {
//     Int(i32),
//     Float(String),
//     Id(String),
//     Bool(bool),
//     String(String),
//     Char(char),
// }

// #[derive(Debug, Clone, Hash)]
// pub enum Expr {
//     Atom(Atom, Span),
//     Unary(Op, Box<Self>, Span),
//     Binary(Op, Box<Self>, Box<Self>, Span),
//     IfElse(Box<Self>, Box<Self>, Box<Self>, Span),
//     Closure(Box<Self>, Box<Self>, Span),
//     Func(String, Box<Self>, Span),
//     App(Box<Self>, Vec<Self>, Span),
//     Enum(String, Vec<(String, Vec<String>)>, Span),
//     TypeDec(String, Vec<String>, Span),
//     Error(Span),
// }
