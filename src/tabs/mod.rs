mod auth;
mod binds;
mod body;
mod headers;
mod params;
mod result;

use crate::areas::SelectedArea;
pub use auth::{Auth, SelectedAuthFeild};
pub use headers::{Header, HeadersList, SelectedHeaderFeild};
pub use params::{Param, ParamsList, SelectedParamFeild};
use ratatui::{
    style::{palette::tailwind, Stylize},
    text::Line,
    widgets::{Block, BorderType, Padding},
};
use strum::{Display, EnumIter, FromRepr};

#[derive(Default, Clone, Copy, Display, FromRepr, EnumIter)]
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
            .fg(tailwind::SLATE.c200)
            .bg(self.palette().c900)
            .into()
    }
    /// A block surrounding the tab's content
    pub fn block(self, selected_area: SelectedArea) -> Block<'static> {
        let highlight_color = if SelectedArea::TabArea == selected_area {
            tailwind::GREEN.c200
        } else {
            tailwind::GREEN.c700
        };

        Block::bordered()
            .border_type(BorderType::Rounded)
            .padding(Padding::horizontal(1))
            .fg(highlight_color)
            .border_style(self.palette().c700)
    }
    pub const fn palette(self) -> tailwind::Palette {
        match self {
            Self::Body => tailwind::BLUE,
            Self::Headers => tailwind::EMERALD,
            Self::Auth => tailwind::INDIGO,
            Self::Params => tailwind::RED,
            Self::Result => tailwind::PURPLE,
        }
    }
}
