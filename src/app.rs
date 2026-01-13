use std::{env, fs};

use crate::{
    areas::SelectedArea,
    tabs::{
        Auth, Header, HeadersList, Param, ParamsList, SelectedAuthFeild, SelectedHeaderFeild,
        SelectedParamFeild, SelectedTab,
    },
};
use color_eyre::Result;
use ratatui::{
    DefaultTerminal,
    crossterm::{
        Command,
        event::{self, Event, KeyCode, KeyEventKind},
    },
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

    // auth
    pub auth: Auth,
    pub selected_auth_feild: SelectedAuthFeild,
    pub auth_holder_value: String,
    pub auth_key_value: String,
    // result
    pub result: String,
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

        // Auth tab specific handling
        // TODO: open eidtor of choice
    // Body tab specific handling
    fn handle_body_tab(&mut self, key: KeyCode) {
        match key {
            KeyCode::Enter => {
                if let Err(e) = self.open_editor_for_body() {
                    eprintln!("Failed to open editor: {}", e);
                }
            }

            _ => {}
        }
    }

    fn open_editor_for_body(&mut self) -> std::io::Result<()> {
        // Write current body content to temp file if it exists
        //if let Some(current_body) = &self.body {
        //    fs::write(&self.body_file_path, current_body)?;
        //} else {
        //    fs::write(&self.body_file_path, "")?;
        //}

        // Get the user's preferred editor
        let editor = env::var("EDITOR")
            .or_else(|_| env::var("VISUAL"))
            .unwrap_or_else(|_| "nano".to_string());
        // Open the editor
        //Command::new(&editor)
        //    .arg(&self.body_file_path)
        //    .status()?;

        // Read the edited content back
        //let edited_content = fs::read_to_string(&self.body_file_path)?;

        // Store it back (you'll need to adjust based on your body structure)
        // Since body is Option<ListState>, you might want to change it to String
        // For now, I'll show both approaches:

        // Option 1: If you change body to String
        // self.body = edited_content;

        // Option 2: If you want to keep the structure, store it separately
        // Add a new field: pub body_content: String,
        // self.body_content = edited_content;

        Ok(())
    }
    // Result tab specific handling
    fn handle_result_tab(&mut self, key: KeyCode) {
        match key {
            // Add result-specific keybinds here
            _ => {}
        }
    }

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
