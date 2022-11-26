use snowc::*;
fn main() {
    std::env::args().nth(1).map_or_else(
        || Repl::default().run().expect("failed to run repl"),
        |filename| {
            let src = std::fs::read_to_string(&filename).unwrap_or("".into());
            // let dot_file = format!("{}.dot", filename.split(".").collect::<Vec<_>>()[0]);
            match parse(&src, false) {
                Ok(s) => {
                    // let node_tree = output_node_tree(&s);
                    // println!("<{dot_file}>\n{node_tree}");
                    for f in s.iter() {
                        println!("{f}");
                    }
                }
                Err(e) => {
                    let span = e
                        .downcast_ref::<snowc_parse::error::ParserError>()
                        .map(|i| i.span())
                        .unwrap_or(0..0);
                    print!("{}", report(&src, span, &e.to_string()));
                }
            }
        },
    );
}
/*
digraph {
compound=true;
rankdir="LR";
subgraph cluster_add {
label="Fn add";
add -> λx -> λy -> "x + y";
}
subgraph cluster_main {
label="Fn main";
main;
}

main -> add [rankdir=LR, lhead=cluster_add];
}
*/
// fn subgraph<'a>(name: &'a str, nodes: &[&'a str]) -> String {
//     format!(
//         "{}}}",
//         nodes.iter().enumerate().fold(
//             format!("subgraph cluster_{0} {{\nlabel=\"{0}\"", name),
//             |last, (i, new)| {
//                 if i == 0 {
//                     return format!("{new}");
//                 }
//                 format!("{last} -> {new}")
//             }
//         )
//     )
// }
//
// fn expr_to_node(expr: &Expr) -> String {
//     match expr {
//         Expr::Atom(_) => todo!(),
//         Expr::Unary(_, _) => todo!(),
//         Expr::Binary(_, _, _) => todo!(),
//         Expr::IfElse(_, _, _) => todo!(),
//         Expr::Clouser(_, _) => todo!(),
//         Expr::Func(_, _) => todo!(),
//         Expr::App(_, _) => todo!(),
//         Expr::Type(_, _) => todo!(),
//         Expr::TypeDec(_, _) => todo!(),
//     }
// }
//
// fn output_node_tree(exprs: &[snowc_parse::Expr]) -> String {
//     let list: Vec<String> = exprs.iter().map(|e| expr_to_node(e)).collect();
//     format!(
//         "digraph {{
// {}
// }}",
//         list.join("\n")
//     )
// }
