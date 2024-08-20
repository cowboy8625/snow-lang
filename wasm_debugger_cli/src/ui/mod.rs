mod sections;

use crate::module::Module;

use ratatui::{
    backend::CrosstermBackend,
    crossterm::{
        event::{self, Event, KeyCode},
        execute,
        terminal::{
            disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
        },
    },
    widgets::{Block, Borders, Paragraph},
    Frame, Terminal,
};

use sections::TypeSectionView;

pub struct App {
    running: bool,
    module: Module,
}

impl App {
    pub fn new(module: Module) -> Self {
        enable_raw_mode().unwrap();
        execute!(std::io::stdout(), EnterAlternateScreen).unwrap();
        Self {
            running: true,
            module,
        }
    }

    fn draw(&self, frame: &mut Frame) {
        let paragraph = Paragraph::new("Hello World!")
            .block(Block::default().borders(Borders::ALL).title("Hello World!"));

        // frame.render_widget(paragraph, frame.area());
        frame.render_widget(
            TypeSectionView {
                types: &self.module.types,
            },
            frame.area(),
        );
    }

    fn handle_event(&mut self, event: Event) {
        match event {
            Event::Key(key) => match key.code {
                KeyCode::Char('q') => self.running = false,
                _ => {}
            },
            _ => {}
        }
    }

    pub fn run(mut self) -> std::io::Result<()> {
        let stdout = std::io::stdout();
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;

        while self.running {
            terminal.draw(|frame| self.draw(frame))?;
            let event = event::read()?;
            self.handle_event(event);
        }
        Ok(())
    }
}

impl Drop for App {
    fn drop(&mut self) {
        disable_raw_mode().unwrap();
        execute!(std::io::stdout(), LeaveAlternateScreen).unwrap();
    }
}
