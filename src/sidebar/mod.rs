use crate::{
    app::App,
    areas::SelectedArea,
    json::{
        add_collection, add_request, del_request, fetch_collection, fetch_collection_by_index,
        fetch_collections,
    },
    tabs::{Auth, Header, Param, SelectedTab},
};
use crossterm::event::KeyCode;
use ratatui_textarea::TextArea;
use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Layout, Rect},
    style::{Color, Stylize},
    text::ToSpan,
    widgets::{Block, BorderType, Borders, Clear, List, Padding, Paragraph, Widget},
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

const WHITE: Color = Color::White;
const BLACK: Color = Color::Black;
const GRAY: Color = Color::Gray;

//#[derive(Deserialize, Serialize, Debug, Default, Clone)]
//struct Collections {
//    collections: Vec<Collection>,
//}

#[derive(Deserialize, Serialize, Debug, Default, Clone)]
pub struct Collection {
    pub id: String,
    pub title: String,
    pub requests: Vec<RequestStructs>,
}

#[derive(Deserialize, Serialize, Debug, Default, Clone)]
pub struct RequestStructs {
    pub url: String,
    pub params: Vec<Param>,
    pub auth: Auth,
    pub headers: Vec<Header>,
    pub body: String,
}

// TODO: make formatting function

impl App {
    pub fn render_sidebar(
        &mut self,
        title_value: String,
        show_popup: bool,
        selected_area: SelectedArea,
        area: Rect,
        full_area: Rect,
        buf: &mut Buffer,
    ) {
        //.highlight_style(Style::default().fg(Color::Green));

        //frame.render_stateful_widget(notes_list, sidebar_area, &mut app_state.note_list_state);

        if show_popup {
            // Calculate centered popup area based on full_area
            let popup_layout = Layout::vertical([
                Constraint::Percentage(35),
                Constraint::Percentage(30),
                Constraint::Percentage(35),
            ])
            .split(full_area);

            let popup_area = Layout::horizontal([
                Constraint::Percentage(25),
                Constraint::Percentage(50),
                Constraint::Percentage(25),
            ])
            .split(popup_layout[1])[1];

            // Clear the background
            Clear.render(popup_area, buf);

            // Create the popup
            let popup_block = Block::bordered()
                .title(" New Collection ")
                .border_type(ratatui::widgets::BorderType::Plain)
                .border_style(WHITE);

            let inner = popup_block.inner(popup_area);
            popup_block.render(popup_area, buf);

            // Split inner area for form fields
            let [value_area, buttons_area] =
                Layout::vertical([Constraint::Length(3), Constraint::Min(1)]).areas(inner);
            Paragraph::new(title_value)
                .block(
                    Block::bordered()
                        .fg(WHITE)
                        .title(" Name ")
                        .border_type(ratatui::widgets::BorderType::Plain),
                )
                .render(value_area, buf);

            Paragraph::new(" [Enter] Save  [ESC] Cancel ")
                .alignment(Alignment::Center)
                .fg(WHITE)
                .render(buttons_area, buf);
        }
        if let Some(collection) = &self.curr_collection {
            // when collection is selected
            let highlight_color = if SelectedArea::Sidebar == selected_area {
                GRAY
            } else {
                BLACK
            };

            let curr_collection_requests_list =
                List::new(collection.requests.iter().map(|x| x.url.to_span()))
                    .block(
                        Block::default()
                            .border_type(BorderType::Plain)
                            .borders(Borders::ALL)
                            .fg(WHITE)
                            .padding(Padding::uniform(1)),
                    )
                    .highlight_symbol(">");

            ratatui::widgets::StatefulWidget::render(
                curr_collection_requests_list,
                area,
                buf,
                &mut self.curr_collection_request_list_state,
            );

            Block::bordered()
                .title(format!(" {} ", collection.title))
                .fg(highlight_color)
                .padding(Padding::uniform(1))
                .border_type(BorderType::Plain)
                .render(area, buf);
        } else {
            // collections list

            let highlight_color = if SelectedArea::Sidebar == selected_area {
                GRAY
            } else {
                BLACK
            };

            match fetch_collections() {
                Ok(collections) => {
                    let notes_list = List::new(collections.iter().map(|x| x.title.to_span()))
                        .block(
                            Block::default()
                                .border_type(BorderType::Plain)
                                .borders(Borders::ALL)
                                .fg(WHITE)
                                .padding(Padding::uniform(1)),
                        )
                        .highlight_symbol(">");

                    ratatui::widgets::StatefulWidget::render(
                        notes_list,
                        area,
                        buf,
                        &mut self.collections_list_state,
                    );
                }
                Err(e) => {
                    eprintln!("Failed to read collections: {}", e);
                }
            }

            Block::bordered()
                .title(" Collections ")
                .fg(highlight_color)
                .padding(Padding::uniform(1))
                .border_type(BorderType::Plain)
                .render(area, buf);
        }
    }

    // keybinds
    pub fn handle_sidebar_area(&mut self, key: KeyCode) {
        match key {
            KeyCode::Char('a') if !self.collection_popup => {
                if let Some(collection) = &self.curr_collection {
                    let new_request = RequestStructs {
                        url: "https://example.com/".to_string(),
                        params: Vec::new(),
                        auth: Auth::default(),
                        headers: Vec::new(),
                        body: String::new(),
                    };

                    match add_request(collection.id.clone(), new_request.clone()) {
                        Ok(_) => {
                            if let Ok(updated) = fetch_collection(collection.id.clone()) {
                                self.curr_collection = Some(updated.clone());
                                let idx = updated.requests.len() - 1;
                                self.curr_collection_request_list_state.select(Some(idx));
                                self.load_request(&new_request);
                            }
                        }
                        Err(e) => {
                            eprintln!("Failed to add request: {}", e);
                        }
                    }
                } else {
                    self.collection_popup = true;
                }
            }

            KeyCode::Char('d') if !self.collection_popup => {
                if let Some(collection) = &self.curr_collection
                    && let Some(index) = self.curr_collection_request_list_state.selected()
                {
                    // Deleted a request from collection

                    match del_request(collection.id.clone(), index) {
                        Ok(_) => {
                            // Refresh the current collection
                            if let Ok(updated) = fetch_collection(collection.id.clone()) {
                                self.curr_collection = Some(updated.clone());

                                // Fix the selection
                                if updated.requests.is_empty() {
                                    self.curr_collection_request_list_state.select(None);
                                } else if index >= updated.requests.len() {
                                    self.curr_collection_request_list_state
                                        .select(Some(updated.requests.len() - 1));
                                } else {
                                    self.curr_collection_request_list_state.select(Some(index));
                                }
                            }
                            eprintln!("Successfully deleted request");
                        }
                        Err(e) => {
                            eprintln!("Failed to delete request: {}", e);
                        }
                    }
                }
            }

            KeyCode::Down => {
                if self.moving {
                    self.next_area()
                } else {
                    if self.curr_collection.is_some() {
                        self.next_collection_request();
                    } else {
                        self.next_collection()
                    }
                }
            }
            KeyCode::Up => {
                if self.moving {
                    self.previous_area()
                } else {
                    if self.curr_collection.is_some() {
                        self.previous_collection_request();
                    } else {
                        self.previous_collection()
                    }
                }
            }

            KeyCode::Char('j') => {
                if !self.param_popup {
                    if self.curr_collection.is_some() {
                        self.next_collection_request();
                    } else {
                        self.next_collection()
                    }
                }
            }

            KeyCode::Char('k') => {
                if !self.param_popup {
                    if self.curr_collection.is_some() {
                        self.previous_collection_request();
                    } else {
                        self.previous_collection()
                    }
                }
            }

            KeyCode::Esc => {
                if self.collection_popup {
                    self.collection_popup = false
                } else if self.curr_collection.is_some() {
                    self.curr_collection = None;
                    self.curr_collection_request_list_state.select(None);
                } else {
                    if self.moving {
                        println!("movement made {:?}", self.moving);
                        self.moving = false
                    }
                }
            }
            KeyCode::Char('o') => {
                if let Some(collection) = &self.curr_collection {
                    if let Some(index) = self.curr_collection_request_list_state.selected() {
                        if let Some(request) = collection.requests.get(index).cloned() {
                            self.load_request(&request);
                        }
                    }
                } else {
                    if let Some(index) = self.collections_list_state.selected() {
                        match fetch_collection_by_index(index) {
                            Ok(collection) => {
                                self.curr_collection = Some(collection.clone());
                                if !collection.requests.is_empty() {
                                    self.curr_collection_request_list_state.select(Some(0));
                                }
                            }
                            Err(e) => {
                                eprintln!("Failed to load collection: {}", e);
                            }
                        }
                    } else {
                        eprintln!("No collection selected");
                    }
                }
            }
            KeyCode::Enter => {
                if self.collection_popup {
                    let id = Uuid::new_v4();
                    let title = self.new_collection_name_value.clone();
                    let new_collection = Collection {
                        id: id.to_string(),
                        title,
                        requests: Vec::new(),
                    };
                    match add_collection(new_collection) {
                        Ok(_) => {
                            self.new_collection_name_value.clear();
                            self.collection_popup = false;
                        }
                        Err(e) => {
                            eprintln!("Failed to save collection: {}", e);
                        }
                    }
                } else if let Some(collection) = &self.curr_collection {
                    if let Some(index) = self.curr_collection_request_list_state.selected() {
                        if let Some(request) = collection.requests.get(index).cloned() {
                            self.load_request(&request);
                        }
                    }
                } else {
                    if let Some(index) = self.collections_list_state.selected() {
                        match fetch_collection_by_index(index) {
                            Ok(collection) => {
                                self.curr_collection = Some(collection.clone());
                                if !collection.requests.is_empty() {
                                    self.curr_collection_request_list_state.select(Some(0));
                                }
                            }
                            Err(e) => {
                                eprintln!("Failed to load collection: {}", e);
                            }
                        }
                    } else {
                        eprintln!("No collection selected");
                    }
                }
            }
            KeyCode::Backspace if self.collection_popup => {
                self.new_collection_name_value.pop();
            }

            KeyCode::Char(c) if self.collection_popup => {
                self.new_collection_name_value.push(c);
            }
            _ => {}
        }
    }

    pub fn load_request(&mut self, request: &RequestStructs) {
        self.url_textarea = TextArea::from(request.url.lines().map(|s| s.to_string()).collect::<Vec<String>>());
        self.params.items = request.params.clone();
        self.headers.items = request.headers.clone();
        
        let body_lines: Vec<String> = request.body.lines().map(|s| s.to_string()).collect();
        self.body_textarea = TextArea::from(body_lines);

        self.auth = request.auth.clone();
        self.param_popup = false;
        self.header_popup = false;
        self.result.clear();
        self.result_scroll = 0;

        self.selected_area = SelectedArea::Url;
        self.selected_tab = SelectedTab::Params;
        self.moving = true;
    }

    pub fn unload_request(&mut self) {
        self.url_value = String::new();
        self.params.items = Vec::new();
        self.headers.items = Vec::new();
        self.body = String::new();

        self.auth = Auth::default();
        self.param_popup = false;
        self.header_popup = false;
        self.result.clear();
        self.result_scroll = 0;

        self.selected_area = SelectedArea::Sidebar;
        self.selected_tab = SelectedTab::Params;
        self.moving = false;
    }

    pub fn next_collection_request(&mut self) {
        self.curr_collection_request_list_state.select_next();
    }

    pub fn previous_collection_request(&mut self) {
        self.curr_collection_request_list_state.select_previous();
    }
    pub fn next_collection(&mut self) {
        self.collections_list_state.select_next();
    }

    pub fn previous_collection(&mut self) {
        self.collections_list_state.select_previous();
    }
}
