use snowc_parse::{parse, Expr};

pub fn snapshot_parsing(input: &str) -> String {
    let ast = match parse(input) {
        Ok(ast) => ast,
        Err(errors) => {
            return errors
                .iter()
                .map(|x| x.report("snowc", input))
                .collect::<Vec<_>>()
                .join("\n");
        }
    };
    let mut ast = std::collections::VecDeque::from(ast);
    let mut output = String::new();
    for (row, line) in input.lines().enumerate() {
        output += line;
        output += "\n";
        while let Some(node) = ast.pop_front() {
            if node.span().row_end != row {
                ast.push_front(node);
                break;
            }
            output += format_node(&input, &node).as_str();
            output += "\n";
            // output += &" ".repeat(node.span().col_start);
            // output += &"^".repeat(node.span().len());
            // output += &format!(" {node:?}");
            // output += "\n"
        }
    }
    output
}

enum ExprVisitor<'a> {
    Root,
    Unary(&'a Expr),
    Binary(&'a Expr, &'a Expr),
    IfElse(&'a Expr, &'a Expr, &'a Expr),
    Closure(&'a Expr, &'a Expr),
    Func(&'a Expr),
    App(&'a Expr, &'a [Expr]),
    Array(&'a [Expr]),
}

fn get_inner_expr<'a>(expr: &'a Expr) -> ExprVisitor<'a> {
    match expr {
        Expr::Atom(..) => ExprVisitor::Root,
        Expr::Unary(unary) => ExprVisitor::Unary(unary.expr.as_ref()),
        Expr::Binary(_, lhs, rhs, ..) => ExprVisitor::Binary(lhs.as_ref(), rhs.as_ref()),
        Expr::IfElse(condition, then, r#else, ..) => {
            ExprVisitor::IfElse(condition.as_ref(), then.as_ref(), r#else.as_ref())
        }
        Expr::Closure(head, tail, ..) => {
            ExprVisitor::Closure(head.as_ref(), tail.as_ref())
        }
        Expr::Func(_, _, node, ..) => ExprVisitor::Func(node.as_ref()),
        Expr::App(name, args, ..) => ExprVisitor::App(name.as_ref(), args),
        Expr::Array(nodes, ..) => ExprVisitor::Array(nodes),
        Expr::Enum(..) => ExprVisitor::Root,
        Expr::Error(..) => ExprVisitor::Root,
    }
}

fn format_node<'a>(src: &str, node: &'a Expr) -> String {
    let mut result = String::new();
    loop {
        match get_inner_expr(node) {
            ExprVisitor::Root => break,
            ExprVisitor::Unary(expr) => {
                result += format_node(src, expr).as_str();
                break;
            }
            ExprVisitor::Binary(lhs, rhs) => {
                result += format_node(src, lhs).as_str();
                result += format_node(src, rhs).as_str();
                break;
            }
            ExprVisitor::IfElse(cond, then, r#else) => {
                result += format_node(src, cond).as_str();
                result += format_node(src, then).as_str();
                result += format_node(src, r#else).as_str();
                break;
            }
            ExprVisitor::Closure(head, tail) => {
                result += format_node(src, head).as_str();
                result += format_node(src, tail).as_str();
                break;
            }
            ExprVisitor::Func(body) => {
                result += format_node(src, body).as_str();
                break;
            }
            ExprVisitor::App(name, args) => {
                result += format_node(src, name).as_str();
                for arg in args {
                    result += format_node(src, arg).as_str();
                }
                break;
            }
            ExprVisitor::Array(array) => {
                for item in array {
                    result += format_node(src, item).as_str();
                }
                break;
            }
        }
    }
    result += &" ".repeat(node.span().col_start);
    result += &"^".repeat(node.span().len());
    result += &format!(" {node:?}");
    result += "\n";
    result
}

macro_rules! snapshot {
    ($name:tt, $path:tt) => {
        #[test]
        fn $name() {
            let contents = include_str!($path);
            let mut settings = insta::Settings::clone_current();
            settings.set_snapshot_path("testdata/output/");
            settings.bind(|| {
                insta::assert_snapshot!(snapshot_parsing(contents));
            });
        }
    };
}

snapshot!(hello_world, "./../../../samples/hello_world.snow");
snapshot!(rule110, "./../../../samples/rule110.snow");
