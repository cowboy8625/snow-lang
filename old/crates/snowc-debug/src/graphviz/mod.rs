#![allow(dead_code)]
mod struct_builder;
mod style;
use super::snowc_parse::{Atom, Expr};
use std::{collections::HashMap, fmt};
use struct_builder::StructBuilder;

use uuid::Uuid;
type GlobalDec = HashMap<String, Node>;
type LocalDec = HashMap<String, Node>;

#[derive(Debug, Default)]
pub struct GraphaBuilder {
    nodes: Vec<Node>,
    builders: Vec<StructBuilder>,
}

impl GraphaBuilder {
    fn with(mut self, node: Node) -> Self {
        self.nodes.push(node);
        self
    }

    fn push(&mut self, node: Node) {
        self.nodes.push(node);
    }

    fn builders(mut self, builders: Vec<StructBuilder>) -> Self {
        self.builders = builders;
        self
    }
}

impl fmt::Display for GraphaBuilder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut graph = String::new();
        graph.push_str("digraph AST {\n    compound=true;\n");
        for builder in self.builders.iter() {
            graph.push_str("    ");
            graph.push_str(&builder.to_string());
        }
        for node in self.nodes.iter() {
            if let Some(names) = node.pointing_to() {
                graph.push_str(&names);
            } else {
                graph.push_str(&node.name());
            }
        }
        graph.push_str("\n}");
        write!(f, "{graph}")
    }
}

#[derive(Debug, Clone, Hash)]
pub enum Node {
    Atom(String),
    Unary {
        op: String,
        rhs: Box<Self>,
    },
    Binary {
        op: String,
        rhs: Box<Self>,
        lhs: Box<Self>,
    },
    IfElse {
        condition: Box<Self>,
        branch1: Box<Self>,
        branch2: Box<Self>,
    },
    App {
        head: Box<Self>,
        tail: Vec<Node>,
    },
    Clouser(Box<Self>, Box<Self>),
}

impl Node {
    fn name(&self) -> String {
        match self {
            Self::Atom(name) => name.to_string(),
            Self::Unary { op, .. } => op.to_string(),
            Self::Binary { op, .. } => op.to_string(),
            Self::IfElse { condition, .. } => condition.name(),
            Self::App { head, .. } => head.name(),
            Self::Clouser(head, _) => head.name(),
        }
    }

    fn pointing_to(&self) -> Option<String> {
        match self {
            Self::Atom(_) => None,
            Self::Unary { op, rhs } => {
                let mut result = format!("    {} -> {op};\n", rhs.name());
                if let Some(r) = rhs.pointing_to() {
                    result.push_str(&r);
                    result.push_str("\n");
                }
                Some(result)
            }
            Self::Binary { op, lhs, rhs } => {
                let mut result =
                    format!("    {{{}, {}}} -> {op};\n", lhs.name(), rhs.name());
                if let Some(l) = lhs.pointing_to() {
                    result.push_str(&l);
                    result.push_str("\n");
                }
                if let Some(r) = rhs.pointing_to() {
                    result.push_str(&r);
                    result.push_str("\n");
                }
                Some(result)
            }
            Self::IfElse {
                condition,
                branch1,
                branch2,
            } => {
                let c = condition.name();
                let b1 = branch1.name();
                let b2 = branch2.name();
                let mut result = format!(
                    "{c} -> {b1} [label=\"then\"];\n{c} -> {b2} [label=\"else\"]\n"
                );
                if let Some(c) = condition.pointing_to() {
                    result.push_str(&c);
                    result.push_str("\n");
                }
                if let Some(b1) = branch1.pointing_to() {
                    result.push_str(&b1);
                    result.push_str("\n");
                }
                if let Some(b2) = branch2.pointing_to() {
                    result.push_str(&b2);
                    result.push_str("\n");
                }
                Some(result)
            }
            Self::App { head, tail } => {
                let h = head.name();
                let mut t = tail.iter().fold("{".to_string(), |last, next| {
                    if last.contains("{") {
                        format!("{} {}", last, next.name())
                    } else {
                        format!("{}, {}", last, next.name())
                    }
                });
                t.push_str("}");
                let result = format!("{h} -> {t};\n");
                Some(result)
            }
            Self::Clouser(tail, _) => tail.pointing_to(),
        }
    }
}

fn id() -> String {
    format!("\"{}\"", Uuid::new_v4().to_string().replace("-", ""))
}

pub fn node_str(
    expr: &Expr,
    builders: &mut Vec<StructBuilder>,
    local: &mut LocalDec,
) -> Node {
    match expr {
        Expr::Atom(Atom::Id(atom_name)) => {
            let Some(node) = local.get(atom_name) else {
                let name = id();
                builders.push(StructBuilder::new(&name).field(atom_name));
                let node = Node::Atom(name);
                local.insert(atom_name.into(), node.clone());
                return node;
            };
            node.clone()
        }
        Expr::Atom(a) => {
            let name = id();
            builders.push(StructBuilder::new(&name).field(&a.to_string()));
            Node::Atom(name)
        }
        Expr::Unary(op, rhs) => {
            let rhs_node = node_str(rhs, builders, local);
            let name = id();
            builders.push(StructBuilder::new(&name).field(&op.to_string()));
            Node::Unary {
                op: name,
                rhs: Box::new(rhs_node),
            }
        }
        Expr::Binary(op, lhs, rhs) => {
            let lhs_node = node_str(lhs, builders, local);
            let rhs_node = node_str(rhs, builders, local);
            let name = id();
            builders.push(StructBuilder::new(&name).field(&op.to_string()));
            Node::Binary {
                op: name,
                lhs: Box::new(lhs_node),
                rhs: Box::new(rhs_node),
            }
        }
        Expr::IfElse(condition, branch1, branch2) => {
            let condition_node = node_str(condition, builders, local);
            let branch1_node = node_str(branch1, builders, local);
            let branch2_node = node_str(branch2, builders, local);
            Node::IfElse {
                condition: Box::new(condition_node),
                branch1: Box::new(branch1_node),
                branch2: Box::new(branch2_node),
            }
        }
        Expr::App(head, tail) => {
            let head_node = node_str(head, builders, local);
            let tail_nodes = tail
                .iter()
                .map(|arg| node_str(arg, builders, local))
                .collect::<Vec<Node>>();
            Node::App {
                head: Box::new(head_node),
                tail: tail_nodes,
            }
        }
        Expr::Clouser(head, tail) => {
            let head_node = node_str(tail, builders, local);
            let tail_node = node_str(head, builders, local);
            Node::Clouser(Box::new(head_node), Box::new(tail_node))
        }
        _ => unimplemented!(),
    }
}

pub fn output_node_tree(ast: &[Expr]) -> String {
    let mut graph = GraphaBuilder::default();
    let mut builders = vec![];
    let mut globals = GlobalDec::new();
    for dec in ast.iter() {
        match dec {
            Expr::Func(name, body) => {
                let mut local = LocalDec::new();
                let node = node_str(body, &mut builders, &mut local);
                globals.insert(name.into(), node.clone());
                graph.push(node);
                //             graph.push_str(&format!(
                //                 "\nsubgraph cluster_{name}{id} {{
                // label=\"Fn {name}\";
                // {var}
                // {name} -> {call};
                // }}"
                //             ));
            }
            _ => unreachable!(),
        }
    }
    graph.builders(builders).to_string()
}

#[test]
fn graphviz_simple() {
    use super::snowc_parse::parse;
    use std::fs::OpenOptions;
    use std::io::Write;
    let ast_vec = parse("fn max x y = if x > y then x else y;", true).unwrap();
    let right = output_node_tree(&ast_vec);
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open("test3.dot")
        .unwrap();
    file.write_all(right.as_bytes()).unwrap();
    eprintln!("{right}");
    let left = r#"digraph AST {
    compound=true;
    subgraph cluster_main0 {
        label=\"Fn main\";
        binary0 [ label=\"1|<if0> /+|3\" ];
        main -> binary0;
    }
}"#;
    assert_eq!(left, right);
}
