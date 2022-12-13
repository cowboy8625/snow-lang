mod trie;
use crossterm::{
    cursor::{position, MoveTo},
    event::{poll, read, Event, KeyCode, KeyEvent, KeyModifiers},
    execute,
    style::{Color, Print, Stylize},
    terminal::{disable_raw_mode, enable_raw_mode, size},
    terminal::{Clear, ClearType},
    Result,
};
use snowc_parse::{Expr, ParserBuilder};
use snowc_tree_walker_eval::FuncMap;
use std::time::Duration;
use trie::Trie;
const HELP_MESSAGE: &str = "Help Commands
:help           get this message
:exit | :quit   kill repl
:clear          clears screen
";
const WELCOME: &str = "snow-lang version 0.0.0";

fn eval(e: Expr, funcmap: &mut FuncMap) -> String {
    let Some(output) = snowc_tree_walker_eval::eval(e, funcmap) else {
        return "".into();
    };
    format!(
        "{}: {}\r\n",
        "[OUTPUT]".with(Color::Green),
        output.to_string().with(Color::Cyan)
    )
}

fn parse(line: &str) -> (String, Vec<snowc_parse::Expr>) {
    match ParserBuilder::default()
        .out_of_main(true)
        .build(line.trim())
        .parse()
    {
        Ok(s) => (
            s.iter()
                .map(|f| {
                    format!(
                        "{}: {}\r\n",
                        "[AST]".with(Color::Green),
                        f.to_string().with(Color::Cyan)
                    )
                })
                .collect(),
            s,
        ),
        Err(e) => (snowc_error_messages::report(line.trim(), e), vec![]),
    }
}

#[derive(Debug, Default, Clone)]
struct History {
    inner: Vec<String>,
    index: usize,
}

impl History {
    fn push(&mut self, item: &str) {
        let Some(last) = self.inner.last() else {
            self.inner.push(item.to_string());
            return;
        };
        if last != item {
            self.inner.push(item.to_string());
        }
    }

    fn reset(&mut self) {
        self.index = self.inner.len().saturating_sub(1);
    }

    fn up(&mut self) -> Option<&String> {
        let idx = self.index;
        self.index = self.index.saturating_sub(1);
        self.inner.get(idx)
    }

    fn down(&mut self) -> Option<&String> {
        let idx = self.index;
        self.index = (self.index + 1).max(self.inner.len());
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
    funcmap: FuncMap,
    word_dict: Trie,
    suggestion: Option<String>,
    history: History,
}

impl Repl {
    const PROMPT: &str = ":> ";
    fn quit(&mut self) {
        self.is_running = false;
    }

    fn prompt(&self) -> String {
        let Some(suggestion) = &self.suggestion else {
            return format!(":> {}", self.input);
        };
        let Some(word) = self.get_current_word() else {
            return format!(":> {}", self.input);
        };
        let end = suggestion.len();
        let start = word.len();
        format!(
            "{}{}{}",
            Self::PROMPT,
            self.input,
            &suggestion
                .get(start..end)
                .unwrap_or("")
                .to_string()
                .with(Color::DarkGreen)
        )
    }

    fn inc_cursor_y(&mut self, by: u16) -> Result<()> {
        self.cursor.y += by;
        self.cursor.y = self.cursor.y.min(self.size.y);
        Ok(())
    }

    fn clear_screen(&mut self) -> Result<()> {
        self.cursor.y = 0;
        let prompt = self.prompt();
        execute!(
            self.writer,
            Clear(ClearType::All),
            MoveTo(0, self.cursor.y),
            Print(prompt),
            MoveTo(self.cursor.x, self.cursor.y),
        )?;
        Ok(())
    }

    fn welcome_message(&mut self) -> Result<()> {
        execute!(
            self.writer,
            MoveTo(0, self.cursor.y),
            Clear(ClearType::CurrentLine),
            Print(WELCOME),
            MoveTo(self.cursor.x, self.cursor.y),
        )?;
        let by = WELCOME.lines().count() as u16;
        self.inc_cursor_y(by)?;
        Ok(())
    }

    fn display_input(&mut self) -> Result<()> {
        let prompt = self.prompt();
        execute!(
            self.writer,
            MoveTo(0, self.cursor.y),
            Clear(ClearType::CurrentLine),
            Print(prompt),
            MoveTo(self.cursor.x, self.cursor.y),
        )?;
        Ok(())
    }

    fn _print_at(&mut self, x: u16, y: u16, msg: &str) -> Result<()> {
        execute!(
            self.writer,
            MoveTo(x, y),
            Print(msg),
            MoveTo(self.cursor.x, self.cursor.y),
        )?;
        let by = msg.lines().count() as u16;
        self.inc_cursor_y(by)?;
        Ok(())
    }

    fn println(&mut self, msg: &str) -> Result<()> {
        execute!(
            self.writer,
            MoveTo(0, self.cursor.y),
            Print(msg),
            MoveTo(self.cursor.x, self.cursor.y),
        )?;
        let by = msg.lines().count() as u16;
        self.inc_cursor_y(by)?;
        Ok(())
    }

    fn run_command(&mut self) -> Result<()> {
        match self.input.as_str() {
            ":exit" | ":quit" => self.quit(),
            ":clear" => self.clear_screen()?,
            ":help" => self.println(HELP_MESSAGE.replace("\n", "\r\n").as_str())?,
            ":func-list" => self.println(&format!("{:?}", self.funcmap))?,
            unknown_command => self.println(&format!("{unknown_command}"))?,
        }
        self.history.push(&self.input);
        self.input.clear();
        Ok(())
    }

    fn sumbit_input(&mut self) -> Result<()> {
        self.cursor.x = 3;
        if self.input.starts_with(":") {
            return self.run_command();
        }
        self.inc_cursor_y(1)?;
        let (ast, exprs) = parse(&self.input);
        self.println(&ast)?;
        for e in exprs.iter() {
            let output = eval(e.clone(), &mut self.funcmap);
            self.println(&output)?;
        }
        self.history.push(&self.input);
        self.history.reset();
        self.input.clear();
        Ok(())
    }

    fn get_current_word(&self) -> Option<String> {
        let Some(" ") = self.input.get(0..1) else {
        return self.input
            .split(" ")
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
            KeyCode::Enter => self.sumbit_input()?,
            KeyCode::Up => {
                let Some(last) = self.history.up() else {
                    return Ok(());
                };
                self.input = last.to_string();
            }
            KeyCode::Down => {
                let Some(last) = self.history.down() else {
                    self.input.clear();
                    return Ok(());
                };
                self.input = last.to_string();
            }
            KeyCode::Tab => {
                let Some(suggestion) = &self.suggestion else {
                    return Ok(());
                };
                let Some(word) = self.get_current_word() else {
                    return Ok(());
                };
                let end = self.input.get(0..1).map(|_| 1).unwrap_or(0);
                let start = self.input.len() - word.len() + end;
                if word == &self.input[start..] {
                    return Ok(());
                }
                self.input =
                    format!("{}{}", &self.input[..start], &suggestion[word.len()..]);
                self.cursor.x = 3 + self.input.len() as u16;
                self.suggestion = None;
            }
            KeyCode::Backspace => {
                self.input.pop();
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
        self.cursor.x += 1;
        self.input.push(c);
        let Some(word) = self.get_current_word() else {
            self.suggestion = None;
            return;
        };
        let word_list = self.word_dict.lookup(word.as_str());
        self.suggestion = word_list.get(0).cloned();
    }

    fn control_modifiers(&mut self, code: KeyCode) -> Result<()> {
        match code {
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
        self.welcome_message()?;
        self.display_input()?;
        while self.is_running {
            if poll(Duration::from_millis(1_000))? {
                let event = read()?;
                match event {
                    Event::Key(key) => self.keyevent(key)?,
                    _ => {}
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
            ":exit", ":quit", ":help", ":clear", "fn", "if", "else", "then", "type",
            "true", "false",
        ]);
        enable_raw_mode().expect("failed to enable raw mode");
        Self {
            writer: std::io::stdout(),
            cursor: position().map(Pos::from).unwrap_or_else(|_| Pos::default()),
            size: size().map(Pos::from).unwrap_or_else(|_| Pos::default()),
            input: String::default(),
            is_running: true,
            funcmap: FuncMap::default(),
            word_dict,
            suggestion: None,
            history: History::default(),
        }
    }
}

impl Drop for Repl {
    fn drop(&mut self) {
        disable_raw_mode().expect("Failed to disable raw mode")
    }
}
