use super::{
    interpreter,
    parser::{self, Expr},
};
use chumsky::Parser;
use std::borrow::Cow::{self, Borrowed, Owned};

use rustyline::completion::{Completer, FilenameCompleter, Pair};
use rustyline::config::OutputStreamType;
use rustyline::error::ReadlineError;
use rustyline::highlight::{Highlighter, MatchingBracketHighlighter};
use rustyline::hint::{Hinter, HistoryHinter};
use rustyline::validate::{self, MatchingBracketValidator, Validator};
use rustyline::{Cmd, CompletionType, Config, Context, EditMode, Editor, KeyEvent};
use rustyline_derive::Helper;

// use crate::interpreter::{interpreter_expr, Environment};
// use crate::parser::parser;
// fn run_block<'a>(
//     block: &str,
//     var: &mut Vec<(&'a String, i128)>,
//     func: &mut Vec<(&'a String, &'a [String], &'a Expr)>,
// ) {
//     let expr = parser::parser().parse(block);
//     match expr {
//         Ok(ast) => match interpreter::eval(&ast, var, func) {
//             Ok(output) => println!("{}", output),
//             Err(eval_err) => println!("Evaluation error: {}", eval_err),
//         },
//         Err(parse_errs) => parse_errs
//             .into_iter()
//             .for_each(|e| println!("Parse error: {:#?}", e)),
//     }

// , mut env: Environment) -> Environment {
// if block.is_empty() {
//     return env;
// }
// if cfg!(feature = "nom-parser") {
//     match parser(block) {
//         Ok((input, expr)) => {
//             for ex in expr {
//                 let (cons_vec, e) = interpreter_expr(ex, env);
//                 env = e;
//                 for cons in cons_vec {
//                     println!("[OUT]: {:?}", cons);
//                     if !input.is_empty() {
//                         println!("[LEFTOVER]: {:?}", input);
//                     }
//                 }
//             }
//         }
//         Err(e) => println!("[ERROR]: {:#?}", e),
//     }
// } else {
// }
// env
// }

fn shell_help() {
    println!(
        "\x1b[1m\x1b[31m[HELP]:\x1b[37m \n{}",
        format!(
            "Shell Commands start with -> {red}:
    {green}:exit{reset} ---------> {cyan}exit program.
    {green}:help{reset} ---------> {cyan}Output this message.
    {green}:clear{reset} --------> {cyan}Clear shell screen.


    Language Syntax:
        hello = {green}\"Hello\"{reset}
        space = {green}\" \"{reset}
        world = {green}\"World\"{reset}
        {cyan}print{reset} hello + space + world {reset}{reset_font}
                         ",
            green = "\x1b[32m",
            reset = "\x1b[37m",
            cyan = "\x1b[36m",
            red = "\x1b[31m",
            reset_font = "\x1b[0m"
        )
    );
}

#[derive(Helper)]
struct MyHelper {
    completer: FilenameCompleter,
    highlighter: MatchingBracketHighlighter,
    validator: MatchingBracketValidator,
    hinter: HistoryHinter,
    colored_prompt: String,
}

impl Completer for MyHelper {
    type Candidate = Pair;

    fn complete(
        &self,
        line: &str,
        pos: usize,
        ctx: &Context<'_>,
    ) -> Result<(usize, Vec<Pair>), ReadlineError> {
        self.completer.complete(line, pos, ctx)
    }
}

impl Hinter for MyHelper {
    type Hint = String;

    fn hint(&self, line: &str, pos: usize, ctx: &Context<'_>) -> Option<String> {
        self.hinter.hint(line, pos, ctx)
    }
}

impl Highlighter for MyHelper {
    fn highlight_prompt<'b, 's: 'b, 'p: 'b>(
        &'s self,
        prompt: &'p str,
        default: bool,
    ) -> Cow<'b, str> {
        if default {
            Borrowed(&self.colored_prompt)
        } else {
            Borrowed(prompt)
        }
    }

    fn highlight_hint<'h>(&self, hint: &'h str) -> Cow<'h, str> {
        Owned("\x1b[1m".to_owned() + hint + "\x1b[m")
    }

    fn highlight<'l>(&self, line: &'l str, pos: usize) -> Cow<'l, str> {
        self.highlighter.highlight(line, pos)
    }

    fn highlight_char(&self, line: &str, pos: usize) -> bool {
        self.highlighter.highlight_char(line, pos)
    }
}

impl Validator for MyHelper {
    fn validate(
        &self,
        ctx: &mut validate::ValidationContext,
    ) -> rustyline::Result<validate::ValidationResult> {
        self.validator.validate(ctx)
    }

    fn validate_while_typing(&self) -> bool {
        self.validator.validate_while_typing()
    }
}

pub fn run() -> rustyline::Result<()> {
    env_logger::init();
    let config = Config::builder()
        .history_ignore_space(true)
        .completion_type(CompletionType::List)
        .edit_mode(EditMode::Emacs)
        .output_stream(OutputStreamType::Stdout)
        .build();
    let h = MyHelper {
        completer: FilenameCompleter::new(),
        highlighter: MatchingBracketHighlighter::new(),
        hinter: HistoryHinter {},
        colored_prompt: "".to_owned(),
        validator: MatchingBracketValidator::new(),
    };
    let mut rl = Editor::with_config(config);
    rl.set_helper(Some(h));
    rl.bind_sequence(KeyEvent::alt('n'), Cmd::HistorySearchForward);
    rl.bind_sequence(KeyEvent::alt('p'), Cmd::HistorySearchBackward);
    if rl.load_history("history.txt").is_err() {
        println!("No previous history.");
    }
    // let mut env = Environment::new();
    // let mut vars = Vec::new();
    // let mut funcs = Vec::new();
    let mut count = 1;
    loop {
        let p = format!("IN [{}]: ", count);
        rl.helper_mut().expect("No helper").colored_prompt = format!("\x1b[1;32m{}\x1b[0m", p);
        let readline = rl.readline(&p);
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str());
                match line.as_str() {
                    ":exit" => break,
                    ":clear" => print!("\x1b[2J\x1b[0;0H"),
                    ":help" => shell_help(),
                    _ => {
                        match parser::parser().parse(line) {
                            Ok(ast) => {
                                match interpreter::eval(&ast, &mut Vec::new(), &mut Vec::new()) {
                                    Ok(output) => println!("{}", output),
                                    Err(eval_err) => println!("Evaluation error: {}", eval_err),
                                }
                            }
                            Err(parse_errs) => parse_errs
                                .into_iter()
                                .for_each(|e| println!("Parse error: {:#?}", e)),
                        }
                        // env = run_block(line.trim(), env);
                        // run_block(line.as_str(), &mut vars, &mut funcs);
                    }
                }
            }
            Err(ReadlineError::Interrupted) => {
                println!("Interrupted");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("Encountered Eof");
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
        count += 1;
    }
    rl.append_history("history.txt")
}
