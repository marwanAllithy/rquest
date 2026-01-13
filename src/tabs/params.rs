use crate::{app::App, tabs::SelectedTab};
use ratatui::{
    buffer::Buffer,
    crossterm::event::KeyCode,
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
            .split(padded_area);

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

impl App {
    pub fn handle_params_tab(&mut self, key: KeyCode) {
        match key {
            KeyCode::Char('a') if !self.param_popup => {
                self.param_popup = true;
            }
            KeyCode::Tab if self.param_popup => {
                self.seleted_param_feild = if self.seleted_param_feild == SelectedParamFeild::Value
                {
                    SelectedParamFeild::Key
                } else {
                    SelectedParamFeild::Value
                };
            }
            KeyCode::Char('j') if !self.param_popup => self.next_param_row(),
            KeyCode::Char('k') if !self.param_popup => self.previous_param_row(),
            KeyCode::Char(c) if self.param_popup => {
                if self.seleted_param_feild == SelectedParamFeild::Key {
                    self.param_key_value.push(c);
                } else {
                    self.param_value_value.push(c);
                }
            }
            KeyCode::Backspace if self.param_popup => {
                if self.seleted_param_feild == SelectedParamFeild::Key {
                    self.param_key_value.pop();
                } else {
                    self.param_value_value.pop();
                }
            }
            KeyCode::Enter => {
                if self.param_popup {
                    if !self.param_key_value.trim().is_empty()
                        && !self.param_value_value.trim().is_empty()
                    {
                        let new_param = Param {
                            key: self.param_key_value.trim().to_string(),
                            value: self.param_value_value.trim().to_string(),
                            enabled: true,
                        };
                        self.param_key_value.clear();
                        self.param_value_value.clear();
                        self.params.items.push(new_param);
                        self.param_popup = false;
                    }
                } else {
                    // Toggle param enabled/disabled
                    if let Some(index) = self.params.state.selected()
                        && let Some(item) = self.params.items.get_mut(index)
                    {
                        item.enabled = !item.enabled;
                    }
                }
            }
            _ => {}
        }
    }
}
