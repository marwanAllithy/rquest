use crate::{app::App, tabs::SelectedTab};
use ratatui::{
    buffer::Buffer,
    crossterm::event::KeyCode,
    layout::{Alignment, Constraint, Layout, Rect},
    style::{Color, Stylize},
    widgets::{Block, Clear, Paragraph, Row, Table, TableState, Widget},
};
use serde::{Deserialize, Serialize};
use strum::{Display, EnumIter, FromRepr};

const WHITE: Color = Color::White;
const BLACK: Color = Color::Black;
const GRAY: Color = Color::Gray;

#[derive(Default, Clone, Debug)]
pub struct ParamsList {
    pub items: Vec<Param>,
    pub state: TableState,
}

#[derive(Default, Deserialize, Serialize, Clone, Debug)]
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
        full_area: Rect,
        show_delete_popup: bool,
    ) {
        let padding_block = Block::bordered()
            .padding(ratatui::widgets::Padding::uniform(1))
            .fg(WHITE);

        let padded_area = padding_block.inner(area);
        padding_block.render(area, buf);

        if show_delete_popup {
            let popup_layout = Layout::vertical([
                Constraint::Percentage(35),
                Constraint::Percentage(30),
                Constraint::Percentage(35),
            ])
            .split(full_area);

            let popup_area = Layout::horizontal([
                Constraint::Percentage(25),
                Constraint::Percentage(50),
                Constraint::Percentage(25),
            ])
            .split(popup_layout[1])[1];

            Clear.render(popup_area, buf);

            let popup_block = Block::bordered()
                .title(" Delete Parameter ")
                .border_type(ratatui::widgets::BorderType::Plain)
                .border_style(WHITE);

            let inner = popup_block.inner(popup_area);
            popup_block.render(popup_area, buf);

            let [message_area, buttons_area] =
                Layout::vertical([Constraint::Length(3), Constraint::Min(1)]).areas(inner);

            Paragraph::new("Delete this parameter?")
                .alignment(Alignment::Center)
                .fg(WHITE)
                .render(message_area, buf);

            Paragraph::new(" [Enter] Yes  [Esc] No ")
                .alignment(Alignment::Center)
                .fg(WHITE)
                .render(buttons_area, buf);
        }

        if show_popup {
            // Calculate centered popup area based on full_area
            let popup_layout = Layout::vertical([
                Constraint::Percentage(35),
                Constraint::Percentage(30),
                Constraint::Percentage(35),
            ])
            .split(full_area);

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
                .border_type(ratatui::widgets::BorderType::Plain)
                .border_style(WHITE);

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
                GRAY
            } else {
                BLACK
            };

            let value_feild_highlight = if SelectedParamFeild::Value == selected_param_feild {
                GRAY
            } else {
                BLACK
            };

            // Render form fields
            Paragraph::new(key_value)
                .block(
                    Block::bordered()
                        .fg(key_feild_highlight)
                        .title(" Key ")
                        .border_type(ratatui::widgets::BorderType::Plain),
                )
                .render(key_area, buf);

            Paragraph::new(value_value)
                .block(
                    Block::bordered()
                        .fg(value_feild_highlight)
                        .title(" Value ")
                        .border_type(ratatui::widgets::BorderType::Plain),
                )
                .render(value_area, buf);

            Paragraph::new("[Tab] Switch [Enter] Save  [ESC] Cancel ")
                .alignment(Alignment::Center)
                .fg(WHITE)
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
            .row_highlight_style(GRAY)
            .column_highlight_style(GRAY)
            .cell_highlight_style(GRAY)
            .highlight_spacing(ratatui::widgets::HighlightSpacing::Always)
            .highlight_symbol(">>");

        ratatui::widgets::StatefulWidget::render(table, padded_area, buf, &mut params.state);
    }
}

impl App {
    pub fn handle_params_tab(&mut self, key: KeyCode) {
        match key {
            KeyCode::Char('a') if !self.param_popup => {
                if !self.moving {
                    self.param_popup = true;
                }
            }

            KeyCode::Tab if self.param_popup => {
                if !self.moving {
                    self.seleted_param_feild =
                        if self.seleted_param_feild == SelectedParamFeild::Value {
                            SelectedParamFeild::Key
                        } else {
                            SelectedParamFeild::Value
                        };
                }
            }

            KeyCode::Char('j') if !self.param_popup && !self.param_delete_popup && !self.moving => {
                self.next_param_row()
            }

            KeyCode::Char('k') if !self.param_popup && !self.param_delete_popup && !self.moving => {
                self.previous_param_row()
            }

            KeyCode::Char(' ') if !self.param_popup && !self.param_delete_popup && !self.moving => {
                if let Some(index) = self.params.state.selected()
                    && let Some(item) = self.params.items.get_mut(index)
                {
                    item.enabled = !item.enabled;
                }
            }

            KeyCode::Char('d') if !self.param_popup && !self.param_delete_popup && !self.moving => {
                if self.params.state.selected().is_some() {
                    self.param_delete_popup = true;
                }
            }
            KeyCode::Esc => {
                if self.param_delete_popup {
                    self.param_delete_popup = false;
                } else if self.param_popup {
                    self.param_popup = false;
                }
                self.moving = !self.moving;
            }
            KeyCode::Enter if self.param_delete_popup => {
                if let Some(index) = self.params.state.selected() {
                    self.params.items.remove(index);
                    if self.params.items.is_empty() {
                        self.params.state.select(None);
                    } else if index >= self.params.items.len() {
                        self.params.state.select(Some(self.params.items.len() - 1));
                    }
                }
                self.param_delete_popup = false;
            }
            KeyCode::Char(c) if self.param_popup => {
                if !self.moving {
                    if self.seleted_param_feild == SelectedParamFeild::Key {
                        self.param_key_value.push(c);
                    } else {
                        self.param_value_value.push(c);
                    }
                }
            }
            KeyCode::Backspace if self.param_popup => {
                if !self.moving {
                    if self.seleted_param_feild == SelectedParamFeild::Key {
                        self.param_key_value.pop();
                    } else {
                        self.param_value_value.pop();
                    }
                }
            }
            KeyCode::Enter => {
                if !self.moving {
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
                } else {
                    self.moving = false
                }
            }
            _ => {}
        }
    }
}
