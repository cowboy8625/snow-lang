use ariadne::{Color, Fmt, Label, Report, ReportKind, Source};
use chumsky::{prelude::*, stream::Stream};

mod debug;
mod lexer;
mod parser;
#[cfg(test)]
mod test;
mod token;
use lexer::Lexer;
use token::Token;
use parser::eval_expr;
// use parser::Parser;
use parser::funcs_parser;

fn snow_source_file(filename: &str) -> Result<String, String> {
    if filename.ends_with(".snow") {
        match std::fs::read_to_string(filename) {
            Ok(file) => Ok(file),
            Err(e) => Err(e.to_string()),
        }
    } else {
        Err("This is not `snow` source file.".into())
    }
}

fn main() {
    let src = snow_source_file(&std::env::args().nth(1).expect("Expected file argument"))
        .expect("Failed to open file.  Expected a Snow File.");
    // println!("{:?}\n", src);
    let lexer = Lexer::new(&src).parse();
    let tokens = lexer.tokens;
    // for token in tokens.iter().map(|(t, _)| t.clone()).collect::<Vec<Token>>().iter() {
    //     println!("{:?}", token);
    // }
    let mut errs = Vec::new();
    // println!("{:?}", lexer.tokens);
    debug::_display_indent(&tokens, &src);
    // let mut parser = Parser::new(&lexer.tokens);
    // let ast = parser.parse();

    // println!("{:#?}", &ast);
    // println!("{:?}", eval_expr(&ast));
    let parse_errs = if let Some(tokens) = Some(tokens) {
        // println!("Tokens = {:?}", tokens);
        let len = src.chars().count();
        let (ast, parse_errs) =
            funcs_parser().parse_recovery(Stream::from_iter(len..len + 1, tokens.into_iter()));

        // println!("{:#?}", ast);
        if let Some(funcs) = ast.filter(|_| errs.len() + parse_errs.len() == 0) {
            if let Some(main) = funcs.get("main") {
                assert_eq!(main.args.len(), 0);
                match eval_expr(&main.body, &funcs, &mut Vec::new()) {
                    Ok(val) => println!("Return value: {}", val),
                    Err(e) => errs.push(Simple::custom(e.span, e.msg)),
                }
            } else {
                panic!("No main function!");
            }
        }

        parse_errs
    } else {
        Vec::new()
    };

    errs.into_iter()
        .map(|e: Simple<Token>| e.map(|c| c.to_string()))
        .chain(parse_errs.into_iter().map(|e| e.map(|tok| tok.to_string())))
        .for_each(|e| {
            let report = Report::build(ReportKind::Error, (), e.span().start);

            let report = match e.reason() {
                chumsky::error::SimpleReason::Unclosed { span, delimiter } => report
                    .with_message(format!(
                        "Unclosed delimiter {}",
                        delimiter.fg(Color::Yellow)
                    ))
                    .with_label(
                        Label::new(span.clone())
                            .with_message(format!(
                                "Unclosed delimiter {}",
                                delimiter.fg(Color::Yellow)
                            ))
                            .with_color(Color::Yellow),
                    )
                    .with_label(
                        Label::new(e.span())
                            .with_message(format!(
                                "Must be closed before this {}",
                                e.found()
                                    .unwrap_or(&"end of file".to_string())
                                    .fg(Color::Red)
                            ))
                            .with_color(Color::Red),
                    ),
                chumsky::error::SimpleReason::Unexpected => report
                    .with_message(format!(
                        "{}, expected {}",
                        if e.found().is_some() {
                            "Unexpected token in input"
                        } else {
                            "Unexpected end of input"
                        },
                        if e.expected().len() == 0 {
                            "something else".to_string()
                        } else {
                            e.expected()
                                .map(|expected| match expected {
                                    Some(expected) => expected.to_string(),
                                    None => "end of input".to_string(),
                                })
                                .collect::<Vec<_>>()
                                .join(", ")
                        }
                    ))
                    .with_label(
                        Label::new(e.span())
                            .with_message(format!(
                                "Unexpected token {}",
                                e.found()
                                    .unwrap_or(&"end of file".to_string())
                                    .fg(Color::Red)
                            ))
                            .with_color(Color::Red),
                    ),
                chumsky::error::SimpleReason::Custom(msg) => report.with_message(msg).with_label(
                    Label::new(e.span())
                        .with_message(format!("{}", msg.fg(Color::Red)))
                        .with_color(Color::Red),
                ),
            };

            report.finish().print(Source::from(&src)).unwrap();
        });
}
