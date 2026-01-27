use crate::{
    app::App,
    areas::SelectedArea,
    tabs::{Auth, Header, Param},
};
use crossterm::event::KeyCode;
use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Layout, Rect},
    style::{palette::tailwind, Stylize},
    widgets::{Block, BorderType, Clear, Padding, Paragraph, Widget},
};
use serde::{Deserialize, Serialize};
use serde_json::Result;
use uuid::Uuid;

#[derive(Deserialize, Serialize, Debug, Default, Clone)]
struct Collections {
    collections: Vec<Collection>,
}

#[derive(Deserialize, Serialize, Debug, Default, Clone)]
pub struct Collection {
    pub title: String,
    pub requests: Vec<RequestStructs>,

    pub id: String,
}

#[derive(Deserialize, Serialize, Debug, Default, Clone)]
pub struct RequestStructs {
    pub url: String,
    pub params: Vec<Param>,
    pub auth: Vec<Auth>,
    pub headers: Vec<Header>,
    pub body: String,
}

// TODO: make formatting function

impl App {
    pub fn render_sidebar(
        &self,
        title_value: String,
        show_popup: bool,
        selected_area: SelectedArea,
        area: Rect,
        buf: &mut Buffer,
    ) {
        if show_popup {
            // Calculate centered popup area inline
            let popup_layout = Layout::vertical([
                Constraint::Percentage(5),
                Constraint::Percentage(20),
                Constraint::Percentage(55),
            ])
            .split(area);

            let popup_area = Layout::horizontal([
                Constraint::Percentage(5),  // Less margin on left = wider popup
                Constraint::Percentage(90), // Wider popup (was probably 50 or less)
                Constraint::Percentage(5),  // Less margin on right
            ])
            .split(popup_layout[1])[1];

            // Clear the background
            Clear.render(popup_area, buf);

            // Create the popup
            let popup_block = Block::bordered()
                .title(" Add Parameter ")
                .border_type(ratatui::widgets::BorderType::Rounded)
                .border_style(tailwind::GRAY.c200);

            let inner = popup_block.inner(popup_area);
            popup_block.render(popup_area, buf);

            // Split inner area for form fields
            let [value_area, buttons_area] =
                Layout::vertical([Constraint::Length(3), Constraint::Min(1)]).areas(inner);
            Paragraph::new(title_value)
                .block(Block::bordered().fg(tailwind::GRAY.c200).title(" Value "))
                .render(value_area, buf);

            Paragraph::new(" [Enter] Save  [ESC] Cancel ")
                .alignment(Alignment::Center)
                .render(buttons_area, buf);
        }
        let highlight_color = if SelectedArea::Sidebar == selected_area {
            tailwind::GREEN.c200
        } else {
            tailwind::GREEN.c700
        };
        Paragraph::new("this will be the sidebar")
            .block(
                Block::bordered()
                    .fg(highlight_color)
                    .padding(Padding::uniform(1))
                    .border_type(BorderType::Rounded),
            )
            .render(area, buf);
    }
    pub fn handle_sidebar_area(&mut self, key: KeyCode) {
        match key {
            KeyCode::Char('a') if !self.collection_popup => {
                self.collection_popup = true;
            }

            KeyCode::Down => self.next_area(),
            KeyCode::Up => self.previous_area(),
            KeyCode::Esc => self.quit(),
            KeyCode::Enter => {
                if self.collection_popup {
                    // TODO: Make all of the data inside of a data.json file insted of the file
                    // based matching for the sake of simplicity
                    let id = Uuid::new_v4();
                    let title = self.new_collection_name_value.clone();
                }
            }

            KeyCode::Char(c) if self.collection_popup => {
                self.new_collection_name_value.push(c);
            }
            _ => {}
        }
    }
}
