use crate::tabs::SelectedTab;
use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Layout, Rect},
    style::{Color, Stylize, palette::tailwind},
    widgets::{Block, Clear, Paragraph, Row, Table, TableState, Widget},
};
use strum::{Display, EnumIter, FromRepr};

#[derive(Default, Clone, Debug)]
pub struct ParamsList {
    pub items: Vec<Param>,
    pub state: TableState,
}

#[derive(Default, Clone, Debug)]
pub struct Param {
    pub key: String,
    pub value: String,
    pub enabled: bool,
}

#[derive(Default, Clone, Copy, Display, FromRepr, EnumIter, PartialEq)]
pub enum SelectedParamFeild {
    #[default]
    #[strum(to_string = "tabs")]
    Key,
    #[strum(to_string = "url")]
    Value,
}

impl SelectedTab {
    pub fn render_params(
        self,
        params: &mut ParamsList,
        area: Rect,
        buf: &mut Buffer,
        show_popup: bool,
        selected_param_feild: SelectedParamFeild,
        key_value: String,
        value_value: String,
    ) {
        let padding_block = Block::bordered().padding(ratatui::widgets::Padding::uniform(1));

        let padded_area = padding_block.inner(area);
        padding_block.render(area, buf);
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
                .title(" Add Parameter ")
                .border_type(ratatui::widgets::BorderType::Rounded)
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

            // Highlighting
            let key_feild_highlight = if SelectedParamFeild::Key == selected_param_feild {
                tailwind::GRAY.c400
            } else {
                tailwind::GRAY.c200
            };

            let value_feild_highlight = if SelectedParamFeild::Value == selected_param_feild {
                tailwind::GRAY.c400
            } else {
                tailwind::GRAY.c200
            };

            // Render form fields
            Paragraph::new(key_value)
                .block(Block::bordered().fg(key_feild_highlight).title(" Key "))
                .render(key_area, buf);

            Paragraph::new(value_value)
                .block(Block::bordered().fg(value_feild_highlight).title(" Value "))
                .render(value_area, buf);

            Paragraph::new("[Tab] Switch [Enter] Save  [ESC] Cancel ")
                .alignment(Alignment::Center)
                .render(buttons_area, buf);
        }

        let widths = [
            Constraint::Length(10),
            Constraint::Length(10),
            Constraint::Length(15),
        ];

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
            .row_highlight_style(Color::Green)
            .column_highlight_style(Color::Green)
            .cell_highlight_style(Color::Green)
            .highlight_spacing(ratatui::widgets::HighlightSpacing::Always)
            .highlight_symbol(">>");

        ratatui::widgets::StatefulWidget::render(table, padded_area, buf, &mut params.state);
    }
}
