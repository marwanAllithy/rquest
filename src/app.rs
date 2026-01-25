use crate::{
    areas::SelectedArea,
    tabs::{
        Auth, HeadersList, ParamsList, SelectedAuthFeild, SelectedHeaderFeild, SelectedParamFeild,
        SelectedTab,
    },
};
use arboard::Clipboard;
use color_eyre::Result;
use crossterm::event::KeyModifiers;
use ratatui::{
    DefaultTerminal,
    crossterm::event::{self, Event, KeyCode, KeyEventKind},
};

#[derive(Default)]
pub struct App {
    state: AppState,
    pub selected_tab: SelectedTab,
    pub selected_area: SelectedArea,
    pub url_value: String,
    //pub moving: bool,
    //pub history: Option<ListState>,

    // Params
    pub param_popup: bool,
    pub seleted_param_feild: SelectedParamFeild,
    pub params: ParamsList,
    pub param_key_value: String,
    pub param_value_value: String,

    // Headers
    pub header_popup: bool,
    pub headers: HeadersList,
    pub header_key_value: String,
    pub header_value_value: String,
    pub selected_header_feild: SelectedHeaderFeild,

    // Body
    pub body: String,
    pub body_content: String,
    pub body_file_path: String,

    // auth
    pub auth: Auth,
    pub selected_auth_feild: SelectedAuthFeild,
    pub auth_holder_value: String,
    pub auth_key_value: String,

    // result
    pub result: String,
    pub result_scroll: u16,

    // Collections
    pub collections: Vec<Collection>,
    pub curr_collection: Collection,
    pub collection_popup: bool,
    pub new_collection_name_value: String,
}

#[derive(Default)]
pub struct Collection {
    pub title: String,
    pub requests: Vec<RequestStructs>,
}

#[derive(Default)]
pub struct RequestStructs {
    pub url: String,
    pub params: ParamsList,
    pub auth: Auth,
    pub headers: HeadersList,
    pub body: String,
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
            terminal.draw(|frame| frame.render_widget(&mut self, frame.area()))?;
            self.handle_events()?;
        }
        Ok(())
    }

    pub fn handle_events(&mut self) -> std::io::Result<()> {
        if let Event::Key(key) = event::read()?
            && key.kind == KeyEventKind::Press
        {
            match self.selected_area {
                // Sidebar
                SelectedArea::Sidebar => self.handle_sidebar_area(key.code),

                // Tab selection area
                SelectedArea::Tabs => match key.code {
                    KeyCode::Char('1') => self.selected_tab = SelectedTab::Params,
                    KeyCode::Char('2') => self.selected_tab = SelectedTab::Headers,
                    KeyCode::Char('3') => self.selected_tab = SelectedTab::Auth,
                    KeyCode::Char('4') => self.selected_tab = SelectedTab::Body,
                    KeyCode::Char('5') => self.selected_tab = SelectedTab::Result,
                    KeyCode::Down => self.next_area(),
                    KeyCode::Up => self.previous_area(),
                    KeyCode::Right => self.next_tab(),
                    KeyCode::Left => self.previous_tab(),
                    KeyCode::Esc => self.quit(),
                    _ => {}
                },

                // URL input area
                SelectedArea::Url => match key.code {
                    KeyCode::Char('v') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                        if let Some(text) = self.get_clipboard_text() {
                            self.url_value.push_str(&text);
                        }
                    }

                    KeyCode::Char(c) => self.url_value.push(c),
                    KeyCode::Backspace => {
                        self.url_value.pop();
                    }
                    KeyCode::Down => self.next_area(),
                    KeyCode::Up => self.previous_area(),
                    KeyCode::Esc => self.quit(),
                    _ => {}
                },

                // Main tab content area - handles all tab-specific logic
                SelectedArea::TabArea => {
                    // Global tab area navigation
                    match key.code {
                        KeyCode::Up => {
                            self.previous_area();
                            return Ok(());
                        }
                        KeyCode::Esc => {
                            // Close any open popups first
                            if self.param_popup {
                                self.param_popup = false;
                                return Ok(());
                            }
                            if self.header_popup {
                                self.header_popup = false;
                                return Ok(());
                            }
                            self.quit();
                            return Ok(());
                        }
                        _ => {}
                    }

                    // Tab-specific handling
                    match self.selected_tab {
                        SelectedTab::Params => self.handle_params_tab(key.code),
                        SelectedTab::Headers => self.handle_headers_tab(key.code),
                        SelectedTab::Auth => self.handle_auth_tab(key.code),
                        SelectedTab::Body => self.handle_body_tab(key.code),
                        SelectedTab::Result => self.handle_result_tab(key.code),
                    }
                }
            }
        }
        Ok(())
    }
    fn get_clipboard_text(&self) -> Option<String> {
        if let Ok(mut clipboard) = Clipboard::new() {
            clipboard.get_text().ok()
        } else {
            None
        }
    }
    // Result tab specific handling
    // Helper methods
    pub fn next_header_row(&mut self) {
        self.headers.state.select_next();
    }

    pub fn previous_header_row(&mut self) {
        self.headers.state.select_previous();
    }

    pub fn next_param_row(&mut self) {
        self.params.state.select_next();
    }

    pub fn previous_param_row(&mut self) {
        self.params.state.select_previous();
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
