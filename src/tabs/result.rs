use std::time::Instant;

use crate::{app::App, areas::SelectedArea, sidebar::RequestStructs, tabs::SelectedTab};
use crossterm::event::KeyCode;
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Color,
    text::Text,
    widgets::{Paragraph, Widget, Wrap},
};

const WHITE: Color = Color::White;

impl SelectedTab {
    pub fn render_result(
        self,
        selected_area: SelectedArea,
        result: &str,
        scroll: u16,
        area: Rect,
        buf: &mut Buffer,
    ) {
        let content = if result.is_empty() {
            "Press Enter in URL field to make request".to_string()
        } else {
            result.to_string()
        };

        Paragraph::new(Text::from(content))
            .style(WHITE)
            .block(self.block(selected_area))
            .wrap(Wrap { trim: false })
            .scroll((scroll, 0))
            .render(area, buf);
    }
}
impl App {
    pub fn make_request(&mut self) {
        let start = Instant::now();

        let mut url = self.url_value.clone();

        let enabled_params: Vec<_> = self.params.items.iter().filter(|p| p.enabled).collect();

        if !enabled_params.is_empty() {
            url.push('?');
            let query: Vec<String> = enabled_params
                .iter()
                .map(|p| format!("{}={}", p.key, p.value))
                .collect();
            url.push_str(&query.join("&"));
        }

        let client = reqwest::blocking::Client::new();
        let mut request = client.get(url);

        //// Add headers
        for header in &self.headers.items {
            if header.enabled {
                request = request.header(&header.key, &header.value);
            }
        }

        //Add body if present
        if !self.body.is_empty() {
            request = request.body(self.body.clone());
        }

        // Make the BLOCKING request - no .await needed
        match request.send() {
            Ok(response) => {
                let duration = start.elapsed();
                let status = response.status();
                let headers = response.headers().clone();

                match response.text() {
                    Ok(body) => {
                        let mut result = String::new();
                        result.push_str(&format!(
                            "Status: {} {}\n",
                            status.as_u16(),
                            status.canonical_reason().unwrap_or("Unknown")
                        ));
                        result.push_str(&format!("Time: {:?}\n\n", duration));

                        result.push_str("Headers:\n");
                        for (name, value) in headers.iter() {
                            result.push_str(&format!(
                                "  {}: {}\n",
                                name,
                                value.to_str().unwrap_or("Invalid UTF-8")
                            ));
                        }

                        result.push_str("\nBody:\n");
                        result.push_str(&body);

                        self.result = result;
                    }
                    Err(e) => {
                        self.result = format!("Error reading response body: {}", e);
                    }
                }
            }
            Err(e) => {
                self.result = format!("Request failed: {}", e);
            }
        }

        // Switch to result tab
        self.selected_tab = SelectedTab::Result;
    }
    pub fn handle_result_tab(&mut self, key: KeyCode) {
        match key {
            KeyCode::Char('j') | KeyCode::Down => {
                self.result_scroll = self.result_scroll.saturating_add(1);
            }
            KeyCode::Char('k') | KeyCode::Up => {
                self.result_scroll = self.result_scroll.saturating_sub(1);
            }
            KeyCode::Enter => {
                if let Some(collection) = &self.curr_collection
                    && let Some(index) = self.curr_collection_request_list_state.selected()
                {
                    let new_request = RequestStructs {
                        url: self.url_value.clone(),
                        params: self.params.items.clone(),
                        auth: self.auth.clone(),
                        headers: self.headers.items.clone(),
                        body: self.body.clone(),
                    };
                    match crate::json::save_request(collection.id.clone(), index, new_request) {
                        Ok(updated) => {
                            self.curr_collection = Some(updated);
                        }
                        Err(e) => {
                            eprintln!("Failed to save request: {}", e);
                        }
                    }
                }
                self.make_request();
            }
            _ => {}
        }
    }
}
