#![allow(dead_code)]
// use std::{fs::OpenOptions, io::Write};

use snowc::*;
fn main() {
    std::env::args().nth(1).map_or_else(
        || Repl::default().run().expect("failed to run repl"),
        |filename| {
            let src = std::fs::read_to_string(&filename).unwrap_or("".into());
            let mut funcmap = FuncMap::new();
            match parse(&src, false) {
                Ok(s) => {
                    for expr in s.iter() {
                        match expr {
                            Expr::Func(name, body) => {
                                let (params, body) =
                                    seperate_args_from_body(*body.clone());
                                funcmap
                                    .insert(name.to_string(), Function { params, body });
                            }
                            _ => println!("{expr}"),
                        }
                    }
                    if let Some(Function { body, .. }) = funcmap.get("main") {
                        if let Some(thing) = eval(body.clone(), &mut funcmap) {
                            println!("{thing}");
                        } else {
                            println!("Running");
                        }
                    } else {
                        panic!("no main to run");
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
