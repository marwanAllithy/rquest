use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Rect},
    widgets::{Block, ListState, Row, Table, TableState},
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
        mut params: ParamsList,
        area: Rect,
        buf: &mut Buffer,
    ) {
        // TODO: add params.items after doing the adding logic
        // TODO: Consider using 3 separate lists to make navigation better?

        // TODO: switch to tables
        let widths = [
            Constraint::Length(5),
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
            //.footer(Row::new(vec!["Updated on Dec 28"]))
            .block(Block::new().title("Table"))
            .highlight_symbol(">>");

        ratatui::widgets::StatefulWidget::render(table, area, buf, &mut table_state);
    }
}
