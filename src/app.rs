use crate::{
    areas::SelectedArea,
    tabs::{
        Header, HeadersList, Param, ParamsList, SelectedHeaderFeild, SelectedParamFeild,
        SelectedTab,
    },
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
    pub body: Option<ListState>,
    // auth
    pub auth: Option<ListState>,
    // result
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
                    KeyCode::Backspace => { self.url_value.pop(); },
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

    // Params tab specific handling
    fn handle_params_tab(&mut self, key: KeyCode) {
        match key {
            KeyCode::Char('a') if !self.param_popup => {
                self.param_popup = true;
            }
            KeyCode::Tab if self.param_popup => {
                self.seleted_param_feild = if self.seleted_param_feild == SelectedParamFeild::Value {
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
                    if let Some(index) = self.params.state.selected() && let Some(item) = self.params.items.get_mut(index) {
                            item.enabled = !item.enabled;
                    }
                }
            }
            _ => {}
        }
    }

    // Headers tab specific handling
    fn handle_headers_tab(&mut self, key: KeyCode) {
        match key {
            KeyCode::Char('a') if !self.header_popup => {
                self.header_popup = true;
            }
            KeyCode::Tab if self.header_popup => {
                self.selected_header_feild = if self.selected_header_feild == SelectedHeaderFeild::Value {
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
                    if let Some(index) = self.headers.state.selected() {
                        if let Some(item) = self.headers.items.get_mut(index) {
                            item.enabled = !item.enabled;
                        }
                    }
                }
            }
            _ => {}
        }
    }

    // Auth tab specific handling
    fn handle_auth_tab(&mut self, key: KeyCode) {
        match key {
            // Add auth-specific keybinds here
            _ => {}
        }
    }

    // Body tab specific handling
    fn handle_body_tab(&mut self, key: KeyCode) {
        match key {
            // Add body-specific keybinds here
            _ => {}
        }
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
