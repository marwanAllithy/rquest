use crate::{app::App, areas::SelectedArea, tabs::SelectedTab};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Color,
    widgets::{Paragraph, Widget},
};
use ratatui_textarea::TextArea;

const WHITE: Color = Color::White;

impl SelectedTab {
    pub fn render_body(
        self,
        selected_area: SelectedArea,
        body_textarea: &TextArea<'static>,
        area: Rect,
        buf: &mut Buffer,
    ) {
        let block = self.block(selected_area);
        let inner = block.inner(area);
        block.render(area, buf);

        let lines = body_textarea.lines();
        if lines.is_empty() || (lines.len() == 1 && lines[0].is_empty()) {
            Paragraph::new("Press Enter + vim keybinds to edit body...")
                .style(WHITE)
                .render(inner, buf);
        } else {
            body_textarea.render(inner, buf);
        }
    }
}

impl App {
    pub fn handle_body_tab(&mut self, key: crossterm::event::KeyCode) {
        match key {
            crossterm::event::KeyCode::Enter => {
                self.moving = false;
            }
            crossterm::event::KeyCode::Esc => {
                self.moving = true;
            }
            _ => {}
        }
    }
}
