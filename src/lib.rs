pub use snowc_error_messages::*;
pub use snowc_errors::*;
pub use snowc_parse::*;
pub use snowc_repl::*;
pub use snowc_tree_walker_eval::*;
pub use snowc_type_checker::*;

#[derive(Debug, Default)]
pub struct CompilerBuilder {
    debug_lexer: bool,
    debug_parser: bool,
    out_of_main: bool,
}

impl CompilerBuilder {
    pub fn debug_lexer(mut self, tog: bool) -> Self {
        self.debug_lexer = tog;
        self
    }

    pub fn debug_parser(mut self, tog: bool) -> Self {
        self.debug_parser = tog;
        self
    }

    pub fn out_of_main(mut self, tog: bool) -> Self {
        self.out_of_main = tog;
        self
    }

    pub fn build(self, src: &str) -> CResult<Compiler> {
        let Self {
            debug_lexer,
            debug_parser,
            out_of_main,
        } = self;
        let ast = ParserBuilder::default()
            .out_of_main(out_of_main)
            .debug_lexer(debug_lexer)
            .debug_parser(debug_parser)
            .build(src)
            .parse()?;
        let ast = type_check(&ast);
        match ast {
            Ok(ast) => run(&ast),
            Err(e) => println!("{}", report(&src, e)),
        }
        Ok(Compiler)
    }
}

fn run(ast: &[Expr]) {
    let mut funcmap = FuncMap::new();
    for expr in ast.iter().filter(|i| i.is_func()) {
        let Expr::Func(name, body) = expr else {
            unimplemented!("here: {expr:?}");
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

pub struct Compiler;
