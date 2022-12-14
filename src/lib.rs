pub use snowc_code_gen::*;
pub use snowc_error_messages::*;
pub use snowc_errors::*;
pub use snowc_parse::*;
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

    pub fn build(self, filename: impl Into<String>) -> CResult<Compiler> {
        let Self {
            debug_lexer,
            debug_parser,
            out_of_main,
        } = self;

        let filename = filename.into();
        let msg = format_compiler_message("Compiling:");
        let msg = format!("{msg} {filename}");
        println!("{}", msg);

        let src = std::fs::read_to_string(&filename)?;

        let ast = timer("Parsing:", || {
            ParserBuilder::default()
                .out_of_main(out_of_main)
                .debug_lexer(debug_lexer)
                .debug_parser(debug_parser)
                .build(&src)
                .parse()
        });

        let ast = match ast {
            Ok(ast) => ast,
            Err(e) => {
                eprintln!("{}", report(&src, e));
                std::process::exit(1);
            }
        };

        let ast = timer("Type Checking:", || type_check(&ast));

        let _ = match ast {
            Ok(ast) => ast,
            Err(e) => {
                eprintln!("{}", report(&src, e));
                std::process::exit(1);
            }
        };

        // println!("{}", format_compiler_message("Complete:"));

        Ok(Compiler)
    }
}

fn timer<T, F>(msg: impl Into<String>, func: F) -> CResult<T>
where
    F: FnOnce() -> CResult<T>,
{
    let start = std::time::Instant::now();
    let out = func()?;
    let now = std::time::Instant::now();
    let time = (now - start).as_secs_f64();
    let msg = format_compiler_message(msg);
    eprintln!("{msg} {time}s");
    Ok(out)
}

fn format_compiler_message(msg: impl Into<String>) -> String {
    let msg = msg.into();
    let w = msg.len() + (15 - msg.len());
    let msg = format!("{:>w$}", msg);
    format!("{msg}")
}
