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
                //Tab keybind
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

                //Params keybind
                SelectedArea::Params => match key.code {
                    KeyCode::Up => self.previous_area(),
                    KeyCode::Char('a') => self.param_popup = true,
                    KeyCode::Tab => {
                        if self.seleted_param_feild == SelectedParamFeild::Value {
                            self.seleted_param_feild = SelectedParamFeild::Key
                        } else {
                            self.seleted_param_feild = SelectedParamFeild::Value
                        }
                    }

                    KeyCode::Char('j') => self.next_param_row(),

                    KeyCode::Char('k') => self.previous_param_row(),

                    event::KeyCode::Char(c) => {
                        if self.param_popup {
                            if self.seleted_param_feild == SelectedParamFeild::Key {
                                self.param_key_value.push(c);
                            } else {
                                self.param_value_value.push(c);
                            }
                        }
                    }
                    event::KeyCode::Backspace => {
                        if self.param_popup {
                            if self.seleted_param_feild == SelectedParamFeild::Key {
                                self.param_key_value.pop();
                            } else {
                                self.param_value_value.pop();
                            }
                        }
                    }

                    KeyCode::Esc => self.param_popup = false,
                    KeyCode::Enter => {
                        if self.param_popup
                            && !self.param_key_value.trim().is_empty()
                            && !self.param_value_value.trim().is_empty()
                        {
                            // adding new params
                            let new_param = Param {
                                key: self.param_key_value.trim().to_string(),
                                value: self.param_value_value.trim().to_string(),
                                enabled: true,
                            };

                            self.param_key_value.clear();
                            self.param_value_value.clear();
                            self.params.items.push(new_param);
                            self.param_popup = false;
                        } else {
                            // marking a param checked or unchecked
                            if let Some(index) = self.params.state.selected()
                                && let Some(item) = self.params.items.get_mut(index)
                            {
                                item.enabled = !item.enabled;
                            }
                        }
                    }
                    _ => {}
                },

                // Headers
                SelectedArea::Headers => match key.code {
                    KeyCode::Up => self.previous_area(),
                    KeyCode::Char('a') => self.header_popup = true,
                    KeyCode::Tab => {
                        if self.selected_header_feild == SelectedHeaderFeild::Value {
                            self.selected_header_feild = SelectedHeaderFeild::Key
                        } else {
                            self.selected_header_feild = SelectedHeaderFeild::Value
                        }
                    }

                    KeyCode::Char('j') => self.next_header_row(),
                    KeyCode::Char('k') => self.previous_header_row(),

                    event::KeyCode::Char(c) => {
                        if self.header_popup {
                            if self.selected_header_feild == SelectedHeaderFeild::Key {
                                self.header_key_value.push(c);
                            } else {
                                self.header_value_value.push(c);
                            }
                        }
                    }
                    event::KeyCode::Backspace => {
                        if self.header_popup {
                            if self.seleted_param_feild == SelectedParamFeild::Key {
                                self.param_key_value.pop();
                            } else {
                                self.param_value_value.pop();
                            }
                        }
                    }

                    KeyCode::Esc => self.header_popup = false,
                    KeyCode::Enter => {
                        if self.header_popup
                            && !self.header_key_value.trim().is_empty()
                            && !self.header_value_value.trim().is_empty()
                        {
                            // adding new params
                            let new_header = Header {
                                key: self.header_key_value.trim().to_string(),
                                value: self.header_value_value.trim().to_string(),
                                enabled: true,
                            };

                            self.header_key_value.clear();
                            self.header_value_value.clear();
                            self.headers.items.push(new_header);
                            self.header_popup = false;
                        } else {
                            // marking a param checked or unchecked
                            if let Some(index) = self.headers.state.selected()
                                && let Some(item) = self.headers.items.get_mut(index)
                            {
                                item.enabled = !item.enabled;
                            }
                        }
                    }
                    _ => {}
                },

                // Url keybind
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

                // tab area keybinds
                SelectedArea::TabArea => match key.code {
                    //KeyCode::Char('k') | KeyCode::Up => self.previous_area(),
                    KeyCode::Down => self.next_area(),
                    KeyCode::Up => self.previous_area(),
                    KeyCode::Esc => self.quit(),
                    _ => {}
                },
            }
        }
        Ok(())
    }

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

    pub fn next_feild(&mut self) {
        self.seleted_param_feild = self.seleted_param_feild.next();
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
