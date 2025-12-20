use ratatui::{
    buffer::Buffer,
    layout::Rect,
    widgets::{Paragraph, Widget},
};

use crate::{areas::SelectedArea, tabs::SelectedTab};

impl SelectedTab {
    pub fn render_headers(
        self,
        selected_area: SelectedArea,
        body: &Option<ratatui::widgets::ListState>,
        area: Rect,
        buf: &mut Buffer,
    ) {
        Paragraph::new("Body Tab - Add request body")
            .block(self.block(selected_area))
            .render(area, buf);
    }
}
