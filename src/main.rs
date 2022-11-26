use snowc::*;
fn main() {
    std::env::args().nth(1).map_or_else(
        || Repl::default().run().expect("failed to run repl"),
        |filename| {
            let src = std::fs::read_to_string(&filename).unwrap_or("".into());
            match parse(&src, false) {
                Ok(s) => {
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
