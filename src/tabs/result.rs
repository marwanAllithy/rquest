use std::collections::HashMap;

use crossterm::event::KeyCode;
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    widgets::{Paragraph, Widget, Wrap},
};
use reqwest::{Response, Url};

use crate::{
    app::App,
    areas::SelectedArea,
    tabs::{Auth, HeadersList, ParamsList, SelectedTab},
};

impl SelectedTab {
    pub fn render_result(
        self,
        selected_area: SelectedArea,
        result: String,
        area: Rect,
        buf: &mut Buffer,
    ) {
        // TODO: Add request type: GET, etc
        //let content = if result.is_empty() {
        //    "Enter to make request".to_string()
        //} else {
        //    result.clone()
        //};

        println!("result: {result:?}");
        Paragraph::new(result)
            .block(self.block(selected_area))
            .wrap(Wrap { trim: false })
            .render(area, buf);
    }
}

//-> std::result::Result<Response, Box<dyn std::error::Error>> {
impl App {
    pub async fn form_result(
        &mut self,
        //url: String,
        //headers: HeadersList,
        //params: ParamsList,
        //auth: Auth,
        //body: String,
    ) -> Result<(), Box<dyn std::error::Error>> {
        //let url = Url::parse(&url)?;
        let client = reqwest::Client::new();
        let mut map = HashMap::new();
        map.insert("lang", "rust");
        map.insert("body", "json");

        // TODO: format the params into a hash map

        let res = reqwest::get("https://dummy.restapiexample.com/api/v1/employees/1")
            .await?
            .text()
            .await?;

        println!("result: {res:?}");
        //self.result = res.text().await?;

        //let text = res;
        //println!("text: {res:?}");
        //println!("result data: {res.eesult:?}");
        Ok(())
    }
    pub fn handle_result_tab(&mut self, key: KeyCode) {
        match key {
            KeyCode::Enter => {
                self.form_result();
            }
            _ => {}
        }
    }
}
