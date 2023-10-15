use std::io::{Result, Write};
use std::time::Duration;
use crossterm::{
    QueueableCommand,
    style::Print,
    event::{read, poll, Event, KeyCode, KeyEvent, KeyModifiers},
    terminal::{enable_raw_mode, disable_raw_mode, Clear, ScrollUp},
    cursor::{SavePosition, RestorePosition, MoveTo}
};

#[derive(Debug, Clone, Copy)]
pub enum Command {
    InsertChar(char),
    Backspace,
    Return,
    Clear,
    Quit,
}


#[derive(Debug, Clone, Copy, Default)]
pub struct Size {
    pub width: u16,
    pub height: u16,
}

impl Size {
    pub fn new() -> Result<Self> {
        let (width, height) = crossterm::terminal::size()?;
        Ok(Self {
            width,
            height,
        })
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Pos<T> {
    pub x: T,
    pub y: T,
}

impl Pos<u16> {
    pub fn new() -> Result<Self> {
        let (x, y): (u16, u16) = crossterm::cursor::position()?;
        Ok(Self {
            x,
            y,
        })
    }
}

#[derive(Debug)]
pub struct Terminal {
    writer: std::io::Stdout,
    cursor: Pos<u16>,
    size: Size,
}

impl Terminal {
    pub fn new() -> Result<Self> {
        enable_raw_mode()?;
        Ok(Self {
            writer: std::io::stdout(),
            cursor: Pos::new()?,
            size: Size::new()?,
        })
    }

    pub fn y(&self) -> u16 {
        self.cursor.y
    }

    pub fn get_input(&mut self) -> Option<Command> {
        if !poll(Duration::from_millis(250)).ok()? {
            return None;
        }

        let event = read().ok()?;
        match event {
            Event::Key(KeyEvent { code: KeyCode::Char('l'), modifiers: KeyModifiers::CONTROL, .. }) => {
                self.clear_screen().ok()?;
                Some(Command::Clear)
            }
            Event::Key(KeyEvent { code: KeyCode::Char(c), .. }) => {
                Some(Command::InsertChar(c))
            },
            Event::Key(KeyEvent { code: KeyCode::Enter, .. }) => {
                self.cursor.y += 1;
                Some(Command::Return)
            },
            Event::Key(KeyEvent { code: KeyCode::Backspace, .. }) => {
                self.cursor.x = self.cursor.x.saturating_sub(1);
                Some(Command::Backspace)
            },
            Event::Key(KeyEvent { code: KeyCode::Esc, .. }) => Some(Command::Quit),
            // Event::Key(KeyEvent { code: KeyCode::Enter, .. }) => repl.state = ReplState::Eval,
            // Event::Key(KeyEvent { code: KeyCode::Char('n'), modifiers: KeyModifiers::CONTROL, .. }) => {
            //     eprintln!("next completion");
            //     repl.state = ReplState::NextCompletion;
            // }
            // Event::Key(KeyEvent { code: KeyCode::Tab, .. }) => repl.state = ReplState::AutoComplete,
            // Event::Key(KeyEvent { code: KeyCode::Backspace, .. }) => backspace(repl)?,
            // Event::Key(KeyEvent { code: KeyCode::Left, .. }) => move_cursor_left(repl)?,
            // Event::Key(KeyEvent { code: KeyCode::Right, .. }) => move_cursor_right(repl)?,
            _ => None,
        }
    }

    pub fn print(&mut self, input: &str) -> Result<()> {
        for (idx, line) in input.lines().enumerate() {
            self.writer.queue(MoveTo(self.cursor.x, self.cursor.y))?;
            self.writer.queue(Print(line))?;
            if idx > 0 {
                self.new_line()?;
            }
        }
        Ok(())
    }

    pub fn print_at(&mut self, x: u16, y: u16, input: &str) -> Result<()> {
        self.writer.queue(SavePosition)?;
        self.writer.queue(MoveTo(x, y))?;
        self.writer.queue(Print(input))?;
        self.writer.queue(RestorePosition)?;
        Ok(())
    }

    pub fn new_line(&mut self) -> Result<()> {
        self.cursor.y += 1;
        self.scroll_up_if_needed(self.cursor.y)?;
        Ok(())
    }

    pub fn scroll_up_if_needed(&mut self, y: u16) -> Result<()> {
        if y >= self.size.height {
            self.scroll_up(1)?;
            self.cursor.y -= 1;
        }
        Ok(())
    }

    pub fn scroll_up(&mut self, amount: u16) -> Result<()> {
        self.writer.queue(ScrollUp(amount))?;
        Ok(())
    }

    pub fn clear_line(&mut self) -> Result<()> {
        self.writer.queue(Clear(crossterm::terminal::ClearType::CurrentLine))?;
        Ok(())
    }

    pub fn clear_from_cursor_down(&mut self) -> Result<()> {
        self.writer.queue(Clear(crossterm::terminal::ClearType::FromCursorDown))?;
        Ok(())
    }

    pub fn clear_screen(&mut self) -> Result<()> {
        self.cursor = Pos::<u16>::default();
        self.writer.queue(Clear(crossterm::terminal::ClearType::All))?;
        Ok(())
    }

    pub fn flush(&mut self) -> Result<()> {
        self.writer.flush()
    }
}

impl Drop for Terminal {
    fn drop(&mut self) {
        disable_raw_mode().expect("Failed to disable raw mode");
    }
}

