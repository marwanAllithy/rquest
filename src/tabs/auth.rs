use crate::{app::App, tabs::SelectedTab};
use ratatui::{
    buffer::Buffer,
    crossterm::event::KeyCode,
    layout::{Constraint::Length, Layout, Rect},
    style::{Color, Stylize},
    widgets::{Block, Paragraph, Widget},
};
use serde::{Deserialize, Serialize};
use strum::{Display, EnumIter, FromRepr};

const WHITE: Color = Color::White;
const BLACK: Color = Color::Black;
const GRAY: Color = Color::Gray;

#[derive(Deserialize, Serialize, Default, Debug, Clone)]
pub struct Auth {
    pub holder: String,
    pub value: String,
}

#[derive(Default, Clone, Copy, Display, FromRepr, EnumIter, PartialEq)]
pub enum SelectedAuthFeild {
    #[default]
    #[strum(to_string = "holder")]
    Holder,
    #[strum(to_string = "value")]
    Value,
}

impl SelectedTab {
    pub fn render_auth(
        self,
        area: Rect,
        buf: &mut Buffer,
        selected_auth_feild: SelectedAuthFeild,
        holder_value: String,
        value_value: String,
    ) {
        let padding_block = Block::bordered()
            .padding(ratatui::widgets::Padding::uniform(1))
            .fg(WHITE);

        let padded_area = padding_block.inner(area);
        padding_block.render(area, buf);

        let auth_form = Layout::vertical([Length(3), Length(3)]);
        let [holder_area, value_area] = auth_form.areas(padded_area);

        // Highlighting
        let holder_highlight = if SelectedAuthFeild::Holder == selected_auth_feild {
            GRAY
        } else {
            BLACK
        };

        let value_highlight = if SelectedAuthFeild::Value == selected_auth_feild {
            GRAY
        } else {
            BLACK
        };

        Paragraph::new(holder_value)
            .block(
                Block::bordered()
                    .title(" Holder ")
                    .fg(holder_highlight)
                    .border_type(ratatui::widgets::BorderType::Plain),
            )
            .render(holder_area, buf);

        Paragraph::new(value_value)
            .block(
                Block::bordered()
                    .title(" value ")
                    .fg(value_highlight)
                    .border_type(ratatui::widgets::BorderType::Plain),
            )
            .render(value_area, buf);
    }
}

impl App {
    pub fn handle_auth_tab(&mut self, key: KeyCode) {
        match key {
            KeyCode::Enter => {
                if self.moving {
                    self.moving = false
                }
            }

            KeyCode::Esc => {
                if !self.moving {
                    self.moving = true
                }
            }

            KeyCode::Tab => {
                self.selected_auth_feild = if self.selected_auth_feild == SelectedAuthFeild::Value {
                    SelectedAuthFeild::Holder
                } else {
                    SelectedAuthFeild::Value
                }
            }
            KeyCode::Char(c) => {
                if self.selected_auth_feild == SelectedAuthFeild::Value && !self.moving {
                    self.auth_key_value.push(c);
                } else {
                    self.auth_holder_value.push(c);
                }
            }
            _ => {}
        }
    }
}
