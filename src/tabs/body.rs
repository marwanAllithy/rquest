use ratatui::{
    buffer::Buffer,
    layout::Rect,
    widgets::{Paragraph, Widget},
};

use crate::{areas::SelectedArea, tabs::SelectedTab};

impl SelectedTab {
    pub fn render_body(
        self,
        selected_area: SelectedArea,
        body: String,
        area: Rect,
        buf: &mut Buffer,
    ) {
        // TODO: make a preview of the file.
        // TODO outsource to external editor
        Paragraph::new("Body Tab - Add request body")
            .block(self.block(selected_area))
            .render(area, buf);
    }
}
