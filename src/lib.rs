// use annotate_snippets::snippets::*;
pub use snowc_byte_code_gen::*;
pub use snowc_error_messages::*;
pub use snowc_parse::*;
pub use snowc_tree_walker::*;
pub use snowc_type_checker::*;

#[derive(Debug, Default)]
pub struct CompilerBuilder {
    debug_lexer: bool,
    debug_parser: bool,
    filename: Option<String>,
    src: Option<String>,
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

    pub fn filename(mut self, filename: impl Into<String>) -> Self {
        self.filename = Some(filename.into());
        self
    }

    pub fn src(mut self, src: impl Into<String>) -> Self {
        self.src = Some(src.into());
        self
    }

    pub fn build(mut self) -> Compiler {
        let Self {
            debug_lexer,
            debug_parser,
            ..
        } = self;

        let filename = if let Some(filename) = self.filename {
            self.src = Some(
                std::fs::read_to_string(&filename)
                    .expect("failed to read file to string"),
            );
            filename
        } else {
            "no name".into()
        };

        let msg = format_compiler_message("Compiling:");
        let msg = format!("{msg} {filename}");
        eprintln!("{}", msg);

        let Some(src) = self.src else {
            eprintln!("no src code to compile");
            std::process::exit(1);
        };

        let result_ast = timer("Parsing:", || {
            ParserBuilder::default()
                .debug_lexer(debug_lexer)
                .debug_parser(debug_parser)
                .build(&src)
                .parse()
        });

        let ast = match result_ast {
            Ok(ast) => ast,
            Err(errors) => {
                report(&filename, &src, &errors);
                std::process::exit(1);
            }
        };

        if let Err(errors) = timer("Type Checking:", || type_check(&ast)) {
            for error in errors {
                eprintln!("{}", error);
            }
            std::process::exit(1);
        }

        // let _ = match ast {
        //     Ok(ast) => ast,
        //     Err(e) => {
        //         eprintln!("{e}");
        //         std::process::exit(1);
        //     }
        // };
        //
        // println!("{}", format_compiler_message("Complete:"));

        Interpreter::new(&ast);

        Compiler
    }
}

fn timer<O, E, F>(msg: impl Into<String>, func: F) -> Result<O, E>
where
    F: FnOnce() -> Result<O, E>,
{
    let start = std::time::Instant::now();
    let out = func();
    let now = std::time::Instant::now();
    let time = (now - start).as_secs_f64();
    let msg = format_compiler_message(msg);
    eprintln!("{msg} {time}s");
    out
}

fn format_compiler_message(msg: impl Into<String>) -> String {
    let msg = msg.into();
    let w = msg.len() + (15 - msg.len());
    let msg = format!("{:>w$}", msg);
    msg
}
