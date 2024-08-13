mod trie;
use crossterm::{
    cursor::{position, MoveTo},
    event::{poll, read, Event, KeyCode, KeyEvent, KeyModifiers},
    style::{Color, Print, Stylize},
    terminal::{disable_raw_mode, enable_raw_mode, size, ScrollUp},
    terminal::{Clear, ClearType},
    ExecutableCommand, Result,
};
use snowc_lexer::{Scanner, Token};
use snowc_parse::{parse, parse_expr, Expr};
use snowc_tree_walker::{eval_expr_with_scope, Scope};
use std::time::Duration;
use trie::Trie;
const HELP_MESSAGE: &str = "Help Commands
:help             get this message
:exit | :quit     kill repl
:clear            clears screen
:scope            show what is in scope
:history          shows history from prompt
:load <filename>  loads a snow file
";
const WELCOME: &str = "snow-lang version 0.0.0\r\n";

#[derive(Debug, Default, Clone)]
struct History {
    inner: Vec<String>,
    index: usize,
}

impl History {
    fn push(&mut self, item: &str) {
        let Some(last) = self.inner.last() else {
            self.inner.push(item.to_string());
            self.index += 1;
            return;
        };
        if last != item {
            self.inner.push(item.to_string());
        }
    }

    fn reset(&mut self) {
        self.index = self.inner.len(); // .saturating_sub(1);
    }

    fn up(&mut self) -> Option<&String> {
        self.index = self.index.saturating_sub(1);
        self.inner.get(self.index)
    }

    fn down(&mut self) -> Option<&String> {
        let idx = self.index;
        self.index = (idx + 1).min(self.inner.len());
        self.inner.get(idx)
    }
}

#[derive(Debug, Clone, Copy, Default)]
struct Pos {
    x: u16,
    y: u16,
}

impl From<(u16, u16)> for Pos {
    fn from((x, y): (u16, u16)) -> Self {
        Self { x, y }
    }
}

pub struct Repl {
    writer: std::io::Stdout,
    cursor: Pos,
    size: Pos,
    input: String,
    is_running: bool,
    scope: Scope,
    word_dict: Trie,
    suggestion: Option<String>,
    history: History,
}

impl Repl {
    const PROMPT: &str = ":> ";
    fn quit(&mut self) {
        self.is_running = false;
        self.save_history();
    }

    fn save_history(&mut self) {
        std::fs::write(".history", self.history.inner.join("\n"))
            .expect("failed to save history");
    }

    fn prompt(&self) -> String {
        let input = syntax_hightlight(&self.input);
        let Some(suggestion) = &self.suggestion else {
            return format!("{}{input}", Self::PROMPT);
        };
        let Some(word) = self.get_current_word() else {
            return format!("{}{input}", Self::PROMPT);
        };
        let end = suggestion.len();
        let start = word.len();
        format!(
            "{}{}{}",
            Self::PROMPT,
            input,
            &suggestion
                .get(start..end)
                .unwrap_or_default()
                .to_string()
                .with(Color::DarkGreen)
        )
    }

    fn inc_cursor_y(&mut self, by: u16) -> Result<()> {
        self.cursor.y += by;
        if self.cursor.y > self.size.y {
            self.writer.execute(ScrollUp(by))?;
        }
        self.cursor.y = self.cursor.y.min(self.size.y);
        Ok(())
    }

    fn cursor_home(&mut self) {
        self.cursor.x = Self::PROMPT.len() as u16;
    }

    fn clear_line(&mut self) -> Result<()> {
        self.cursor_home();
        self.input.clear();
        Ok(())
    }

    fn backspace_word(&mut self) -> Result<()> {
        let spacing = {
            let i = self.input.trim_end().len();
            self.input.len() - i
        };
        let Some(word) = self.input.trim_end().split(' ').last() else {
            return Ok(());
        };
        let idx = self.input.len() - (word.len() + spacing);
        self.input = self.input[..idx].trim_end().to_string();
        self.cursor.x = (Self::PROMPT.len() + self.input.len()) as u16;
        self.writer.execute(Clear(ClearType::CurrentLine))?;
        Ok(())
    }

    fn clear_screen(&mut self) -> Result<()> {
        self.cursor.y = 0;
        let prompt = self.prompt();
        self.writer.execute(Clear(ClearType::All))?;
        self.writer.execute(MoveTo(0, self.cursor.y))?;
        self.writer.execute(Print(prompt))?;
        self.writer.execute(MoveTo(self.cursor.x, self.cursor.y))?;
        Ok(())
    }

    fn display_input(&mut self) -> Result<()> {
        let prompt = self.prompt();
        self.writer.execute(MoveTo(0, self.cursor.y))?;
        self.writer.execute(Clear(ClearType::CurrentLine))?;
        self.writer.execute(Print(prompt))?;
        self.writer.execute(MoveTo(self.cursor.x, self.cursor.y))?;
        Ok(())
    }

    fn _print_at(&mut self, x: u16, y: u16, msg: &str) -> Result<()> {
        self.writer.execute(MoveTo(x, y))?;
        self.writer.execute(Print(msg))?;
        self.writer.execute(MoveTo(self.cursor.x, self.cursor.y))?;
        Ok(())
    }

    fn println(&mut self, msg: &str) -> Result<()> {
        self.writer.execute(MoveTo(0, self.cursor.y))?;
        self.writer.execute(Print(msg))?;
        self.writer.execute(MoveTo(self.cursor.x, self.cursor.y))?;
        let by = (msg.lines().count() as u16).max(1);
        self.inc_cursor_y(by)?;
        Ok(())
    }

    fn display_scope(&mut self) -> Result<()> {
        self.inc_cursor_y(1)?;
        self.println("global")?;
        let g = self.scope.global.clone();
        for (name, item) in g.iter() {
            self.println(&format!("{name}: {item}"))?;
        }
        self.println("local")?;
        let l = self.scope.local.clone();
        for (name, item) in l.iter() {
            self.println(&format!("{name}: {item}"))?;
        }
        Ok(())
    }

    fn run_command(&mut self) -> Result<()> {
        let input = self.input.to_string();
        let (command, args) = input.split_once(' ').unwrap_or((&self.input, ""));

        match command {
            ":exit" | ":quit" => self.quit(),
            ":clear" => self.clear_screen()?,
            ":help" => {
                self.inc_cursor_y(1)?;
                self.println(HELP_MESSAGE.replace('\n', "\r\n").as_str())?;
            }
            ":scope" => self.display_scope()?,
            ":history" => {
                let history = self.history.inner.join("\r\n");
                self.inc_cursor_y(1)?;
                self.println(&history)?;
            }
            ":load" => {
                self.inc_cursor_y(1)?;
                let src = std::fs::read_to_string(args)?;
                self.eval(&src)?;
            }
            command if command.starts_with(':') => {
                self.inc_cursor_y(1)?;
                self.println(&format!("{} is not a command", self.input))?
            }
            _ => {
                self.inc_cursor_y(1)?;
                self.println(&self.input.to_string())?;
            }
        }
        self.input.clear();
        Ok(())
    }

    fn success_parsing(&mut self, ast: &[Expr]) -> Result<()> {
        for e in ast.iter() {
            self.println(&format!("{e}"))?;
        }
        for e in ast.iter() {
            match eval_expr_with_scope(e, &mut self.scope) {
                Ok(Some(ok)) => {
                    let output = format!(
                        "{}: {}",
                        "[OUTPUT]".with(Color::Green),
                        ok.to_string().with(Color::Cyan)
                    );
                    self.println(&output)?;
                }
                Err(e) => {
                    let msg = e.report("--repl--", &self.input).replace('\n', "\n\r");
                    self.println(&msg)?;
                }
                _ => {}
            }
        }
        self.history.push(&self.input);
        self.history.reset();
        self.input.clear();
        Ok(())
    }

    fn eval(&mut self, input: &str) -> Result<()> {
        match parse(Scanner::new(input)) {
            Ok(ast) => self.success_parsing(&ast)?,
            Err(errors) => match parse_expr(Scanner::new(&self.input)) {
                Ok(ast) => self.success_parsing(&[ast])?,
                _ => {
                    for e in errors.iter() {
                        let msg = e.report("--repl--", &self.input).replace('\n', "\n\r");
                        self.println(&msg)?;
                    }
                }
            },
        }
        Ok(())
    }

    fn submit_input(&mut self) -> Result<()> {
        self.history.push(&self.input);
        self.cursor.x = 3;
        if self.input.starts_with(':') {
            return self.run_command();
        }
        self.inc_cursor_y(1)?;
        let src = self.input.clone();
        self.eval(&src)?;
        Ok(())
    }

    fn get_current_word(&self) -> Option<String> {
        let Some(" ") = self.input.get(0..1) else {
        return self.input
            .split(' ')
            .map(ToString::to_string)
            .collect::<Vec<String>>()
            .last()
            .cloned();
        };
        None
    }

    fn keycode_event(&mut self, keycode: KeyCode) -> Result<()> {
        match keycode {
            KeyCode::Esc => self.quit(),
            KeyCode::Enter => self.submit_input()?,
            KeyCode::Up => {
                let Some(last) = self.history.up() else {
                    return Ok(());
                };
                self.input = last.to_string();
                self.cursor.x = (Self::PROMPT.len() + self.input.len()) as u16;
            }
            KeyCode::Right => {
                let len = (Self::PROMPT.len() + self.input.len()) as u16;
                self.cursor.x = self.cursor.x.saturating_add(1).min(len);
            }
            KeyCode::Left => {
                self.cursor.x = self
                    .cursor
                    .x
                    .saturating_sub(1)
                    .max(Self::PROMPT.len() as u16);
            }
            KeyCode::Down => {
                let Some(last) = self.history.down() else {
                    self.input.clear();
                    self.cursor.x = Self::PROMPT.len() as u16;
                    return Ok(());
                };
                self.input = last.to_string();
                self.cursor.x = (Self::PROMPT.len() + self.input.len()) as u16;
            }
            KeyCode::Tab => {
                let Some(suggestion) = self.suggestion.clone() else {
                    return Ok(());
                };
                let Some(word) = self.get_current_word() else {
                    return Ok(());
                };
                let idx_start = self.input.len() - word.len();
                let input = &self.input[..idx_start];
                let start_of_word = &self.input[idx_start..];
                let end_of_word = &suggestion[word.len()..];
                self.input = format!("{input}{start_of_word}{end_of_word}");
                self.cursor.x = (Self::PROMPT.len() + self.input.len()) as u16;
                self.suggestion = None;
            }
            KeyCode::Backspace => {
                if self.cursor.x == 0 {
                    return Ok(());
                }
                let x = self.cursor.x as usize;
                let idx = x.saturating_sub(Self::PROMPT.len());

                if idx == 0 {
                    return Ok(());
                }

                if idx == self.input.len() {
                    self.input.pop();
                } else {
                    self.input.remove(idx - 1);
                }

                let Some(word) = self.get_current_word() else {
                    return Ok(());
                };
                let word_list = self.word_dict.lookup(&word);
                self.suggestion = word_list.get(0).cloned();
                self.cursor.x = self
                    .cursor
                    .x
                    .saturating_sub(1)
                    .max(Self::PROMPT.len() as u16);
            }
            KeyCode::Char(c) => self.input_char(c),
            _ => {}
        }
        Ok(())
    }

    fn input_char(&mut self, c: char) {
        let idx = (self.cursor.x as usize) - Self::PROMPT.len();
        self.input.insert(idx, c);
        self.cursor.x += 1;
        let Some(word) = self.get_current_word() else {
            self.suggestion = None;
            return;
        };
        let word_list = self.word_dict.lookup(word.as_str());
        self.suggestion = word_list.get(0).cloned();
    }

    fn control_modifiers(&mut self, code: KeyCode) -> Result<()> {
        match code {
            KeyCode::Char('u') => self.clear_line()?,
            KeyCode::Char('w') => self.backspace_word()?,
            KeyCode::Char('l') => self.clear_screen()?,
            KeyCode::Char('d') => self.quit(),
            KeyCode::Char('c') => self.quit(),
            _ => {}
        }
        Ok(())
    }

    fn keyevent(&mut self, key: KeyEvent) -> Result<()> {
        match key {
            KeyEvent {
                code: KeyCode::Char(c),
                modifiers: KeyModifiers::SHIFT,
                ..
            } => self.input_char(c),
            KeyEvent {
                code,
                modifiers: KeyModifiers::NONE,
                ..
            } => self.keycode_event(code)?,
            KeyEvent {
                code,
                modifiers: KeyModifiers::CONTROL,
                ..
            } => self.control_modifiers(code)?,
            _ => {}
        }
        Ok(())
    }

    pub fn run(&mut self) -> Result<()> {
        self.cursor.x = Self::PROMPT.len() as u16;
        self.println(WELCOME)?;
        self.display_input()?;
        while self.is_running {
            if poll(Duration::from_millis(1_000))? {
                let event = read()?;
                if let Event::Key(key) = event {
                     self.keyevent(key)?;
                }
                self.display_input()?;
            }
        }
        Ok(())
    }
}

impl Default for Repl {
    fn default() -> Self {
        let word_dict = Trie::from(&vec![
            ":exit", ":quit", ":help", ":clear", ":scope", ":history", "fn", "if",
            "else", "then", "enum", "true", "false",
        ]);
        let history = std::fs::read_to_string(".history")
            .map(|s| {
                let inner = s.lines().map(ToString::to_string).collect::<Vec<String>>();
                let index = inner.len().saturating_sub(0);
                History { inner, index }
            })
            .unwrap_or_default();
        enable_raw_mode().expect("failed to enable raw mode");
        Self {
            writer: std::io::stdout(),
            cursor: position().map(Pos::from).unwrap_or_else(|_| Pos::default()),
            size: size().map(Pos::from).unwrap_or_else(|_| Pos::default()),
            input: String::default(),
            is_running: true,
            scope: Scope::default(),
            word_dict,
            suggestion: None,
            history,
        }
    }
}

impl Drop for Repl {
    fn drop(&mut self) {
        disable_raw_mode().expect("Failed to disable raw mode")
    }
}

fn syntax_hightlight(input: impl Into<String>) -> String {
    let input = input.into();
    match input.as_str() {
        ":clear" | ":help" | ":scope" | ":quit" | ":exit" | ":history" => {
            return input.green().to_string()
        }
        word if word.starts_with(':') => return input.dark_red().to_string(),
        _ => {}
    }
    Scanner::new(&input)
        .fold((0, String::new()), |(cursor, mut acc), tok| {
            let span = tok.span();
            let count = span.idx_start - cursor;
            acc += &" ".repeat(count);
            match tok {
                Token::KeyWord(..) => {
                    acc += &tok
                        .to_string()
                        .with(Color::Rgb {
                            r: 250,
                            g: 69,
                            b: 5,
                        })
                        .to_string();
                }
                Token::Id(..) => {
                    acc += &tok.to_string().dark_green().to_string();
                }
                Token::Int(..) | Token::Float(..) => {
                    acc += &tok.to_string().magenta().to_string();
                }
                Token::String(..) => {
                    acc += &tok.to_string().green().bold().to_string();
                }
                Token::Error(..) => {
                    acc += &tok.to_string().dark_red().to_string();
                }
                tok => {
                    acc += &tok.to_string();
                }
            }
            (span.idx_end, acc)
        })
        .1
}
