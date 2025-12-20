use crate::{
    areas::SelectedArea,
    tabs::{ParamsList, SelectedTab},
};
use color_eyre::Result;
use ratatui::{
    DefaultTerminal,
    crossterm::event::{self, Event, KeyCode, KeyEventKind},
    widgets::ListState,
};

#[derive(Default)]
pub struct App {
    state: AppState,
    pub selected_tab: SelectedTab,
    pub selected_area: SelectedArea,
    pub url_value: String,
    pub moving: bool,
    pub history: Option<ListState>,
    pub params: ParamsList,
    pub headers: Option<ListState>,
    pub body: Option<ListState>,
    pub auth: Option<ListState>,
    pub result: Option<String>,
}

#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub enum AppState {
    #[default]
    Running,
    Quitting,
}
impl App {
    pub fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        while self.state == AppState::Running {
            terminal.draw(|frame| frame.render_widget(&self, frame.area()))?;
            self.handle_events()?;
        }
        Ok(())
    }

    pub fn handle_events(&mut self) -> std::io::Result<()> {
        if let Event::Key(key) = event::read()?
            && key.kind == KeyEventKind::Press
        {
            match self.selected_area {
                SelectedArea::Tabs => match key.code {
                    KeyCode::Char('j') | KeyCode::Down => self.next_area(),
                    KeyCode::Char('k') | KeyCode::Up => self.previous_area(),
                    KeyCode::Char('l') | KeyCode::Right => self.next_tab(),
                    KeyCode::Char('h') | KeyCode::Left => self.previous_tab(),
                    KeyCode::Char('q') | KeyCode::Esc => self.quit(),
                    _ => {}
                },

                SelectedArea::Url => match key.code {
                    event::KeyCode::Char(c) => {
                        self.url_value.push(c);
                    }
                    event::KeyCode::Backspace => {
                        self.url_value.pop();
                    }
                    KeyCode::Down => self.next_area(),
                    KeyCode::Up => self.previous_area(),
                    KeyCode::Esc => self.quit(),
                    _ => {}
                },

                SelectedArea::TabArea => match key.code {
                    KeyCode::Char('j') | KeyCode::Down => self.next_area(),
                    KeyCode::Char('k') | KeyCode::Up => self.previous_area(),
                    KeyCode::Char('q') | KeyCode::Esc => self.quit(),
                    _ => {}
                },
            }
        }
        Ok(())
    }

    pub fn get_selected_area(&self) -> SelectedArea {
        self.selected_area
    }

    pub fn next_area(&mut self) {
        self.selected_area = self.selected_area.next();
    }

    pub fn previous_area(&mut self) {
        self.selected_area = self.selected_area.previous();
    }

    pub fn next_tab(&mut self) {
        self.selected_tab = self.selected_tab.next();
    }

    pub fn previous_tab(&mut self) {
        self.selected_tab = self.selected_tab.previous();
    }

    pub fn quit(&mut self) {
        self.state = AppState::Quitting;
    }
}
