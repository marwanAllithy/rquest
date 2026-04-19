mod auth;
mod binds;
mod body;
mod headers;
mod help;
mod params;
mod result;

use crate::areas::SelectedArea;
pub use auth::{Auth, SelectedAuthFeild};
pub use headers::{Header, HeadersList, SelectedHeaderFeild};
pub use help::get_help_categories;
pub use params::{Param, ParamsList, SelectedParamFeild};
use ratatui::{
    style::{Color, Stylize},
    text::Line,
    widgets::{Block, BorderType, Padding},
};
use strum::{Display, EnumIter, FromRepr};

const WHITE: Color = Color::White;
const BLACK: Color = Color::Black;
const GRAY: Color = Color::Gray;

#[derive(Default, Clone, Copy, Display, FromRepr, EnumIter, PartialEq)]
pub enum SelectedTab {
    #[default]
    #[strum(to_string = "[1] Params")]
    Params,
    #[strum(to_string = "[2] Headers")]
    Headers,
    #[strum(to_string = "[3] Auth")]
    Auth,
    #[strum(to_string = "[4] Body")]
    Body,
    #[strum(to_string = "[5] Result")]
    Result,
}

impl SelectedTab {
    pub fn title(self) -> Line<'static> {
        format!("  {self}  ")
            .fg(WHITE)
            .into()
    }
    /// A block surrounding the tab's content
    pub fn block(self, selected_area: SelectedArea) -> Block<'static> {
        let highlight_color = if SelectedArea::TabArea == selected_area {
            GRAY
        } else {
            BLACK
        };

        Block::bordered()
            .border_type(BorderType::Plain)
            .padding(Padding::horizontal(1))
            .fg(highlight_color)
    }
}
