// use error::CResult;
#![allow(dead_code)]
use crossterm::{
    cursor::{position, MoveLeft, MoveTo},
    event::{poll, read, Event, KeyCode, KeyEvent, KeyModifiers},
    execute,
    style::{Color, Print, Stylize},
    terminal::{disable_raw_mode, enable_raw_mode, size},
    terminal::{Clear, ClearType},
};
use std::fs;
use std::io::stdout;
use std::time::Duration;

fn eval(line: &str) -> String {
    match parser::parse(line.trim(), true) {
        Ok(s) => s
            .iter()
            .map(|f| {
                format!(
                    "{}: {}\r\n",
                    "[OUT]".with(Color::Green),
                    f.to_string().with(Color::Cyan)
                )
            })
            .collect(),
        Err(e) => {
            let span = e
                .downcast_ref::<parser::ParserError>()
                .map(|i| i.span())
                .unwrap_or(0..0);
            report::report(line.trim(), span, &e.to_string())
        }
    }
}

fn prompt(msg: &str) -> String {
    format!(":> {msg} ")
}

pub fn repl() {
    size()
        .and_then(|(_w, _h)| {
            position().and_then(|(mut x, mut y)| {
                enable_raw_mode().and_then(|_| {
                    let mut history = fs::read_to_string("history.txt")
                        .unwrap_or("".into())
                        .lines()
                        .map(|line| line.trim().to_string())
                        .collect::<Vec<String>>();
                    let mut history_number = 0usize;
                    let mut line = String::new();
                    let mut writer = stdout();
                    loop {
                        execute!(
                            writer,
                            MoveTo(x, y),
                            Print(prompt(&line)),
                            MoveLeft(1),
                        )?;
                        if poll(Duration::from_millis(1_000))? {
                            let event = read()?;
                            match event {
                                Event::Key(key) => match key {
                                    KeyEvent {
                                        code: KeyCode::Esc, ..
                                    }
                                    | KeyEvent {
                                        code: KeyCode::Char('c'),
                                        modifiers: KeyModifiers::CONTROL,
                                        ..
                                    }
                                    | KeyEvent {
                                        code: KeyCode::Char('d'),
                                        modifiers: KeyModifiers::CONTROL,
                                        ..
                                    } => {
                                        if !history.is_empty() {
                                            fs::write(
                                                "history.txt",
                                                history
                                                    .iter()
                                                    .map(|i| format!("{i}\n"))
                                                    .collect::<String>(),
                                            )?;
                                        }
                                        break;
                                    }
                                    KeyEvent {
                                        code: KeyCode::Char('l'),
                                        modifiers: KeyModifiers::CONTROL,
                                        ..
                                    } => {
                                        x = 0;
                                        y = 0;
                                        execute!(
                                            writer,
                                            Clear(ClearType::All),
                                            MoveTo(x, y),
                                            Print(prompt(&line))
                                        )?;
                                    }
                                    KeyEvent {
                                        code: KeyCode::Up, ..
                                    } => {
                                        history_number = history_number.saturating_sub(1);
                                        line = history[history_number].to_string();
                                    }
                                    KeyEvent {
                                        code: KeyCode::Backspace,
                                        ..
                                    } => {
                                        line.pop();
                                        execute!(
                                            writer,
                                            MoveTo(x, y),
                                            Print(prompt(&line)),
                                            MoveLeft(1),
                                        )?;
                                    }
                                    KeyEvent {
                                        code: KeyCode::Char(c),
                                        ..
                                    } => {
                                        line.push(c);
                                        execute!(
                                            writer,
                                            MoveTo(x, y),
                                            Print(prompt(&line)),
                                            MoveLeft(1),
                                        )?;
                                    }
                                    KeyEvent {
                                        code: KeyCode::Enter,
                                        ..
                                    } => {
                                        history.push(line.clone());
                                        history_number = history.len() - 1;
                                        execute!(
                                            writer,
                                            MoveTo(x, y),
                                            Print(prompt(&line)),
                                        )?;
                                        let result = eval(&line);
                                        y += 1;
                                        execute!(writer, MoveTo(x, y), Print(&result),)?;
                                        y += result.lines().count() as u16;
                                        line.clear();
                                    }
                                    _ => {}
                                },
                                _ => {}
                            }
                        }
                    }
                    Ok(())
                })
            })
        })
        .and_then(|_| disable_raw_mode())
        .ok()
        .unwrap_or(disable_raw_mode().unwrap_or(()));
}
