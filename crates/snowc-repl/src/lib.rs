mod terminal;
use crossterm::style::Stylize;
use terminal::{Command, Pos, Terminal};

use anyhow::Result;

const PROMPT: &str = ":> ";
const _WELCOME: &str = "snow-lang version 0.0.0\r\n";
const _HELP_MESSAGE: &str = "Help Commands
:help             get this message
:exit | :quit     kill repl
:clear            clears screen
:scope            show what is in scope
:history          shows history from prompt
:load <filename>  loads a snow file
";

use snowc_tree_walker::{eval_expr_with_scope, Scope, Value};

pub fn repl() -> Result<()> {
    let mut repl = Repl::new();
    let mut terminal = Terminal::new()?;
    let mut scope = Scope::default();

    terminal.print("Welcome to snow repl")?;
    terminal.new_line()?;
    // terminal.print(PROMPT)?;
    terminal.prompt(PROMPT, &repl.input, &repl.pos)?;
    terminal.flush()?;

    while repl.is_running() {
        let Some(command) = terminal.get_input() else {
            continue;
        };

        match command {
            Command::InsertChar(c) => repl.insert_char(c),
            Command::CursorLeft => repl.cursor_left(),
            Command::CursorRight => repl.cursor_right(),
            Command::DeleteFromCursorBackward => {
                repl.delete_from_cursor_backward();
                terminal.clear_line()?;
            }
            Command::Backspace => backspace(&mut terminal, &mut repl)?,
            Command::Return => {
                execute_return_command(&mut terminal, &mut repl, &mut scope)?
            }
            Command::Clear => repl.pos.y = 0,
            Command::Quit => repl.quit(),
        }

        if !matches!(command, Command::Return) {
            let mut s = scope.clone();
            match compile(&repl, &mut s) {
                Ok(Some(v)) => {
                    let y = terminal.y() + 1;
                    terminal.scroll_up_if_needed(y)?;
                    terminal.print_at(0, y, &v.to_string().grey().to_string())?;
                }
                _ => {
                    terminal.clear_from_cursor_down()?;
                }
            }
        }

        terminal.prompt(PROMPT, &repl.input, &repl.pos)?;
        // terminal.print(&format!("{PROMPT}{}", &repl.input))?;
        terminal.flush()?;
    }
    Ok(())
}

// Not sure what to return here
fn compile(
    repl: &Repl,
    scope: &mut Scope,
) -> std::result::Result<Option<Value>, Vec<String>> {
    let input = &repl.input;
    let (filename, src) = repl
        .loaded_file
        .clone()
        .unwrap_or(("snowc repl".into(), String::new()));
    let i = &(src + &repl.compiled_lines + input);
    let ast = match snowc_parse::parse(input) {
        Ok(ast) => ast,
        Err(_) => match snowc_parse::expression(input) {
            Ok(ast) => vec![ast],
            Err(err) => return Err(vec![err.report(&filename, i)]),
        },
    };
    if ast.iter().any(|x| x.is_error()) {
        return Ok(None);
    }

    let mut results = vec![];
    for node in ast {
        match eval_expr_with_scope(&node, scope) {
            Ok(v) => results.push(v),
            Err(err) => return Err(vec![err.report("snowc", i)]),
        }
    }
    Ok(results.pop().flatten())
}

fn execute_return_command(
    terminal: &mut Terminal,
    repl: &mut Repl,
    scope: &mut Scope,
) -> Result<()> {
    if execute_builtin_repl_command(terminal, repl, scope)? {
        repl.clear_input();
        return Ok(());
    }
    match compile(&repl, scope) {
        Ok(Some(v)) => {
            terminal.print(&v.to_string().yellow().to_string())?;
            terminal.new_line()?;
            repl.successful_compiled_line();
        }
        Err(errors) => {
            terminal.print(&errors.join("\n"))?;
        }
        _ => {
            let y = terminal.y();
            terminal.scroll_up_if_needed(y)?;
            repl.successful_compiled_line();
        }
    }
    repl.clear_input();
    Ok(())
}

fn execute_builtin_repl_command(
    terminal: &mut Terminal,
    repl: &mut Repl,
    scope: &mut Scope,
) -> Result<bool> {
    match repl.input.clone().trim() {
        i if i.starts_with(":load") => {
            // FIXME: properly handle filename if it doesn't exist
            let filename = &i[6..];
            let src = std::fs::read_to_string(filename).unwrap();
            repl.input = src.clone();
            let result = compile(&repl, scope);
            if result.is_err() {
                terminal.print(&format!("failed to compile {filename}"))?;
                terminal.new_line()?;
                terminal.print(&result.unwrap_err().join("\n"))?;
                terminal.new_line()?;
                return Ok(true);
            }
            repl.loaded_file = Some((filename.to_string(), src));
            terminal.print(&format!("loaded file {}", &i[6..]))?;
            terminal.new_line()?;
            Ok(true)
        }
        ":scope" => {
            terminal.print("LOCALS")?;
            terminal.new_line()?;
            for (k, v) in scope.local.iter() {
                terminal.print(&format!("{k}: {v}"))?;
                terminal.new_line()?;
            }
            terminal.print("GLOBALS")?;
            terminal.new_line()?;
            for (k, v) in scope.global.iter() {
                terminal.print(&format!("{k}: {v}"))?;
                terminal.new_line()?;
            }
            terminal.print(&format!("COMPILED LINES: {}", &repl.compiled_lines))?;
            terminal.new_line()?;
            Ok(true)
        }
        ":exit" | ":quit" => {
            repl.quit();
            Ok(true)
        }
        ":clear" => {
            terminal.clear_screen()?;
            repl.pos.y = 0;
            Ok(true)
        }
        _ => Ok(false),
    }
}

fn backspace(terminal: &mut Terminal, repl: &mut Repl) -> Result<()> {
    repl.backspace();
    terminal.clear_line()?;
    Ok(())
}

struct Repl {
    input: String,
    running: bool,
    pos: Pos<usize>,
    compiled_lines: String,
    loaded_file: Option<(String, String)>,
    // history: History,
}

impl Repl {
    fn new() -> Self {
        Self {
            input: String::new(),
            running: true,
            pos: Pos::default(),
            compiled_lines: String::new(),
            loaded_file: None,
        }
    }

    fn successful_compiled_line(&mut self) {
        self.compiled_lines += &self.input;
        self.compiled_lines += "\n";
    }

    fn insert_char(&mut self, c: char) {
        self.input.insert(self.pos.x, c);
        self.pos.x += 1;
    }

    fn clear_input(&mut self) {
        self.pos.x = 0;
        self.input.clear();
    }

    fn backspace(&mut self) {
        if self.pos.x == 0 {
            return;
        }
        self.pos.x -= 1;
        self.input.remove(self.pos.x);
    }

    fn delete_from_cursor_backward(&mut self) {
        if self.pos.x == 0 {
            return;
        }
        self.input = self.input[self.pos.x..].to_string();
        self.pos.x = 0;
    }

    fn cursor_right(&mut self) {
        if self.pos.x == self.input.len() {
            return;
        }
        self.pos.x += 1;
    }

    fn cursor_left(&mut self) {
        if self.pos.x == 0 {
            return;
        }
        self.pos.x -= 1;
    }

    fn is_running(&self) -> bool {
        self.running
    }

    fn quit(&mut self) {
        self.running = false;
    }
}
