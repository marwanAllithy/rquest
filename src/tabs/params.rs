use color_eyre::owo_colors::Color;
use ratatui::{
    buffer::Buffer,
    crossterm::terminal::Clear,
    layout::{Alignment, Constraint, Layout, Rect},
    widgets::{Block, ListState, Paragraph, Row, Table, TableState, Widget},
};

use crate::{areas::SelectedArea, tabs::SelectedTab};

#[derive(Default, Clone, Debug)]
pub struct ParamsList {
    pub items: Vec<Param>,
    pub state: ListState,
}

#[derive(Default, Clone, Debug)]
pub struct Param {
    pub key: String,
    pub value: String,
    pub enabled: bool,
}

impl SelectedTab {
    pub fn render_params(
        self,
        selected_area: SelectedArea,
        params: ParamsList,
        area: Rect,
        buf: &mut Buffer,
    ) {
        // TODO: add params.items after doing the adding logic
        // TODO: Consider using 3 separate lists to make navigation better?
        // TODO: make the form a pop up
        // TODO: switch to tables
        let widths = [
            Constraint::Length(6),
            Constraint::Length(5),
            Constraint::Length(10),
        ];
        let mut table_state = TableState::default();

        let rows: Vec<Row> = params
            .items
            .iter()
            .map(|param| {
                let checkbox = if param.enabled { "✓" } else { " " };

                Row::new(vec![checkbox, &param.key, &param.value])
            })
            .collect();

        let table = Table::new(rows, widths)
            .column_spacing(1)
            .header(Row::new(vec!["Active", "Param", "Value"]).top_margin(1))
            .block(Block::new().title("Params"))
            .highlight_symbol(">>");

        ratatui::widgets::StatefulWidget::render(table, area, buf, &mut table_state);
        if show_popup {
            // Calculate centered popup area inline
            let popup_layout = Layout::vertical([
                Constraint::Percentage(35),
                Constraint::Percentage(30),
                Constraint::Percentage(35),
            ])
            .split(area);

            let popup_area = Layout::horizontal([
                Constraint::Percentage(25),
                Constraint::Percentage(50),
                Constraint::Percentage(25),
            ])
            .split(popup_layout[1])[1];

            // Clear the background
            Clear.render(popup_area, buf);

            // Create the popup
            let popup_block = Block::bordered()
                .title("Add Parameter")
                .border_type(BorderType::Rounded)
                .border_style(Color::Green);

            let inner = popup_block.inner(popup_area);
            popup_block.render(popup_area, buf);

            // Split inner area for form fields
            let [key_area, value_area, buttons_area] = Layout::vertical([
                Constraint::Length(3),
                Constraint::Length(3),
                Constraint::Min(1),
            ])
            .areas(inner);

            // Render form fields
            Paragraph::new("Key: ")
                .block(Block::bordered().title("Key"))
                .render(key_area, buf);

            Paragraph::new("Value: ")
                .block(Block::bordered().title("Value"))
                .render(value_area, buf);

            Paragraph::new("[Enter] Save  [ESC] Cancel")
                .alignment(Alignment::Center)
                .render(buttons_area, buf);
        }
    }
}
