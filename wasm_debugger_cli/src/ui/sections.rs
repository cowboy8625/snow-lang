use crate::type_section::{FunctionType, Type, TypeSection};
use crate::utils::into_hex;
use ratatui::{
    crossterm::event::{self, Event, KeyCode},
    prelude::*,
    widgets::{Block, Borders, List, Paragraph, Widget},
    Frame,
};
pub trait SectionView {
    fn draw(&self, frame: &mut Frame);
    fn handle_event(&mut self, event: Event);
}

pub struct TypeSectionView<'a> {
    pub types: &'a TypeSection,
}

impl<'a> Widget for TypeSectionView<'a> {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(1), Constraint::Length(1)])
            .split(area);
        let id_line = Line::from(vec![
            Span::styled("ID", Style::default().add_modifier(Modifier::BOLD)),
            " ".into(),
            Span::styled("Length", Style::default().add_modifier(Modifier::BOLD)),
            " ".into(),
            Span::styled("Count", Style::default().add_modifier(Modifier::BOLD)),
        ]);
        let paragraph = Paragraph::new(id_line);

        paragraph.render(layout[0], buf);

        let byte_line = Line::from(format!(
            "{:02X} '{}' {:02X}",
            FunctionType::ID,
            &into_hex(&self.types.length)[0..self.types.length.len() * 3 - 1],
            self.types.count()
        ));
        let byte_paragraph = Paragraph::new(byte_line);

        byte_paragraph.render(layout[1], buf);
    }
}
