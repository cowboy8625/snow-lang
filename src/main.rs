use snowc::*;
fn main() {
    std::env::args().nth(1).map_or_else(
        || Repl::default().run().expect("failed to run repl"),
        |filename| compile(&filename),
    );
}

fn compile(filename: &str) {
    let src = std::fs::read_to_string(&filename).unwrap_or("".into());
    match parse(&src, false) {
        Ok(ast) => run(&ast),
        Err(e) => error_message(&src, e),
    }
}

fn run(ast: &[Expr]) {
    let mut funcmap = FuncMap::new();
    for expr in ast.iter() {
        let Expr::Func(name, body) = expr else {
            unimplemented!("{expr:?}");
        };
        let (params, body) = seperate_args_from_body(*body.clone());
        funcmap.insert(name.to_string(), Function { params, body });
    }
    let Some(Function { body, .. }) = funcmap.get("main") else {
        eprintln!("ERROR: file is missing main function");
        std::process::exit(1);
    };
    let Some(thing) = eval(body.clone(), &mut funcmap) else {
        return;
    };
    println!("{thing}");
}

fn error_message(src: &str, error: Box<dyn std::error::Error>) {
    let span = error
        .downcast_ref::<snowc_parse::error::ParserError>()
        .map(|i| i.span())
        .unwrap_or(0..0);
    print!("{}", report(&src, span, &error.to_string()));
}
