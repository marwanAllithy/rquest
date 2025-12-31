use crate::{
    areas::SelectedArea,
    tabs::{Param, ParamsList, SelectedParamFeild, SelectedTab},
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
    pub param_popup: bool,
    pub seleted_param_feild: SelectedParamFeild,
    pub params: ParamsList,
    pub param_key_value: String,
    pub param_value_value: String,
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
                //Tab keybind
                SelectedArea::Tabs => match key.code {
                    KeyCode::Down => self.next_area(),
                    KeyCode::Up => self.previous_area(),
                    KeyCode::Right => self.next_tab(),
                    KeyCode::Left => self.previous_tab(),
                    KeyCode::Esc => self.quit(),
                    _ => {}
                },

                //Params keybind
                SelectedArea::Params => match key.code {
                    KeyCode::Char('k') | KeyCode::Up => self.previous_area(),
                    KeyCode::Char('a') => self.param_popup = true,
                    KeyCode::Tab => {
                        if self.seleted_param_feild == SelectedParamFeild::Value {
                            self.seleted_param_feild = SelectedParamFeild::Key
                        } else {
                            self.seleted_param_feild = SelectedParamFeild::Value
                        }
                    }

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
