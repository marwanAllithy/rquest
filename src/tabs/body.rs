use crate::{app::App, areas::SelectedArea, tabs::SelectedTab};
use ratatui::{
    buffer::Buffer,
    crossterm::event::KeyCode,
    layout::Rect,
    style::Color,
    widgets::{Paragraph, Widget},
};
use std::{env, fs, process::Command};

const WHITE: Color = Color::White;

impl SelectedTab {
    pub fn render_body(
        self,
        selected_area: SelectedArea,
        body: &String, // Changed to referencd
        area: Rect,
        buf: &mut Buffer,
    ) {
        // Display the body content
        let content = if body.is_empty() {
            "Press Enter to edit body in your editor...".to_string()
        } else {
            format!("Body content:\n{}", body)
        };

        Paragraph::new(content)
            .style(WHITE)
            .block(self.block(selected_area))
            .render(area, buf);
    }
}

// TODO: make the body address dynamic

impl App {
    pub fn handle_body_tab(&mut self, key: KeyCode) {
        // Changed to &mut self
        match key {
            KeyCode::Enter => {
                if let Err(e) = self.open_editor_for_body() {
                    eprintln!("Failed to open editor: {}", e);
                }
            }
            _ => {}
        }
    }

    fn open_editor_for_body(&mut self) -> std::io::Result<()> {
        fs::write(&self.body_file_path, &self.body)?;
        let editor = env::var("EDITOR")
            .or_else(|_| env::var("VISUAL"))
            .unwrap_or_else(|_| "nano".to_string());
        Command::new(&editor).arg(&self.body_file_path).status()?;
        self.body = fs::read_to_string(&self.body_file_path)?;

        Ok(())
    }
}
