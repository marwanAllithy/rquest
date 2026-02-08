use crate::{app::App, tabs::SelectedTab};
use ratatui::{
    buffer::Buffer,
    crossterm::event::KeyCode,
    layout::{Alignment, Constraint, Layout, Rect},
    style::{Color, Stylize, palette::tailwind},
    widgets::{Block, Clear, Paragraph, Row, Table, TableState, Widget},
};
use serde::{Deserialize, Serialize};
use strum::{Display, EnumIter, FromRepr};

#[derive(Default, Clone, Debug)]
pub struct HeadersList {
    pub items: Vec<Header>,
    pub state: TableState,
}

#[derive(Default, Deserialize, Serialize, Clone, Debug)]
pub struct Header {
    pub key: String,
    pub value: String,
    pub enabled: bool,
}

#[derive(Default, Clone, Copy, Display, FromRepr, EnumIter, PartialEq)]
pub enum SelectedHeaderFeild {
    #[default]
    #[strum(to_string = "tabs")]
    Key,
    #[strum(to_string = "url")]
    Value,
}

impl SelectedTab {
    pub fn render_headers(
        self,
        headers: &mut HeadersList,
        area: Rect,
        buf: &mut Buffer,
        show_popup: bool,
        selected_header_feild: SelectedHeaderFeild,
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
                .title(" Add Header ")
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
            let key_feild_highlight = if SelectedHeaderFeild::Key == selected_header_feild {
                tailwind::GRAY.c400
            } else {
                tailwind::GRAY.c200
            };

            let value_feild_highlight = if SelectedHeaderFeild::Value == selected_header_feild {
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

        let rows: Vec<Row> = headers
            .items
            .iter()
            .map(|header| {
                let checkbox = if header.enabled { "✓" } else { " " };

                Row::new(vec![checkbox, &header.key, &header.value])
            })
            .collect();

        let table = Table::new(rows, widths)
            .column_spacing(1)
            .header(Row::new(vec!["Active", "Header", "Value"]).top_margin(1))
            .block(Block::new().title("Headers"))
            .row_highlight_style(Color::Green)
            .column_highlight_style(Color::Green)
            .cell_highlight_style(Color::Green)
            .highlight_spacing(ratatui::widgets::HighlightSpacing::Always)
            .highlight_symbol(">>");

        ratatui::widgets::StatefulWidget::render(table, padded_area, buf, &mut headers.state);
    }
}

impl App {
    pub fn handle_headers_tab(&mut self, key: KeyCode) {
        match key {
            KeyCode::Char('a') if !self.header_popup => {
                self.header_popup = true;
            }
            KeyCode::Tab if self.header_popup => {
                self.selected_header_feild =
                    if self.selected_header_feild == SelectedHeaderFeild::Value {
                        SelectedHeaderFeild::Key
                    } else {
                        SelectedHeaderFeild::Value
                    };
            }
            KeyCode::Char('j') if !self.header_popup => self.next_header_row(),
            KeyCode::Char('k') if !self.header_popup => self.previous_header_row(),
            KeyCode::Char(c) if self.header_popup => {
                if self.selected_header_feild == SelectedHeaderFeild::Key {
                    self.header_key_value.push(c);
                } else {
                    self.header_value_value.push(c);
                }
            }
            KeyCode::Backspace if self.header_popup => {
                if self.selected_header_feild == SelectedHeaderFeild::Key {
                    self.header_key_value.pop();
                } else {
                    self.header_value_value.pop();
                }
            }
            KeyCode::Enter => {
                if self.header_popup {
                    if !self.header_key_value.trim().is_empty()
                        && !self.header_value_value.trim().is_empty()
                    {
                        let new_header = Header {
                            key: self.header_key_value.trim().to_string(),
                            value: self.header_value_value.trim().to_string(),
                            enabled: true,
                        };
                        self.header_key_value.clear();
                        self.header_value_value.clear();
                        self.headers.items.push(new_header);
                        self.header_popup = false;
                    }
                } else {
                    // Toggle header enabled/disabled
                    if let Some(index) = self.headers.state.selected()
                        && let Some(item) = self.headers.items.get_mut(index)
                    {
                        item.enabled = !item.enabled;
                    }
                }
            }
            _ => {}
        }
    }
}
