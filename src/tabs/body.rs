use crate::{app::App, areas::SelectedArea, tabs::SelectedTab};
use crossterm::event::KeyCode;
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Color,
    widgets::{Paragraph, Widget},
};
use ratatui_textarea::{CursorMove, TextArea};
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
            Paragraph::new("Press Enter to edit the body...")
                .style(WHITE)
                .render(inner, buf);
        } else {
            body_textarea.render(inner, buf);
        }
    }
}

impl App {
    pub fn handle_body_tab(&mut self, key: crossterm::event::KeyEvent) {
        match key.code {
            KeyCode::Enter => {
                self.moving = false;
            }
            KeyCode::Esc => {
                if self.moving {
                    self.moving = false;
                } else {
                    self.moving = true;
                }
            }
            KeyCode::Char('j') | KeyCode::Down => {
                if !self.moving {
                    self.body_textarea.move_cursor(CursorMove::Down);
                }
            }
            KeyCode::Char('k') | KeyCode::Up => {
                if self.moving {
                    self.previous_area();
                } else {
                    self.body_textarea.move_cursor(CursorMove::Up);
                }
            }
            KeyCode::Left => {
                if self.moving {
                    self.previous_area();
                } else {
                    self.body_textarea.move_cursor(CursorMove::Back);
                }
            }
            KeyCode::Right => {
                if self.moving {
                    self.next_area();
                } else {
                    self.body_textarea.move_cursor(CursorMove::Forward);
                }
            }
            KeyCode::Char('y')
                if key
                    .modifiers
                    .contains(crossterm::event::KeyModifiers::CONTROL) =>
            {
                if !self.moving && self.body_textarea.is_selecting() {
                    if let Some(((start_row, start_col), (end_row, end_col))) =
                        self.body_textarea.selection_range()
                    {
                        let lines = self.body_textarea.lines();
                        let mut selected_text = String::new();
                        for (i, line) in lines.iter().enumerate() {
                            if i >= start_row && i <= end_row {
                                let start = if i == start_row { start_col } else { 0 };
                                let end = if i == end_row { end_col } else { line.len() };
                                selected_text.push_str(&line[start..end]);
                                if i < end_row {
                                    selected_text.push('\n');
                                }
                            }
                        }
                        let _ = self.set_clipboard_text(selected_text);
                    }
                }
            }
            KeyCode::Char('d')
                if key
                    .modifiers
                    .contains(crossterm::event::KeyModifiers::CONTROL) =>
            {
                if !self.moving && self.body_textarea.is_selecting() {
                    if let Some(((start_row, start_col), (end_row, end_col))) =
                        self.body_textarea.selection_range()
                    {
                        let lines = self.body_textarea.lines();
                        let mut selected_text = String::new();
                        for (i, line) in lines.iter().enumerate() {
                            if i >= start_row && i <= end_row {
                                let start = if i == start_row { start_col } else { 0 };
                                let end = if i == end_row { end_col } else { line.len() };
                                selected_text.push_str(&line[start..end]);
                                if i < end_row {
                                    selected_text.push('\n');
                                }
                            }
                        }
                        let _ = self.set_clipboard_text(selected_text);
                        self.body_textarea.cut();
                    }
                }
            }
            KeyCode::Char('v') | KeyCode::Char('p')
                if key
                    .modifiers
                    .contains(crossterm::event::KeyModifiers::CONTROL) =>
            {
                if !self.moving {
                    self.body_textarea.paste();
                }
            }
            _ => {}
        }
    }
}
