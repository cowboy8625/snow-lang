fn main() {
    std::env::args().nth(1).map_or_else(
        || one_liner::repl(),
        |filename| {
            let src = std::fs::read_to_string(&filename).unwrap_or("".into());
            match parser::parse(&src, false) {
                Ok(s) => {
                    for f in s.iter() {
                        println!("{f}");
                    }
                }
                Err(e) => {
                    let span = e
                        .downcast_ref::<parser::ParserError>()
                        .map(|i| i.span())
                        .unwrap_or(0..0);
                    print!("{}", report::report(&src, span, &e.to_string()));
                }
            }
        },
    );
}
