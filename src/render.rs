use ratatui::{
    buffer::Buffer,
    layout::{
        Constraint::{self, Length, Min, Percentage},
        Layout, Rect,
    },
    style::{Color, Stylize},
    text::Line,
    widgets::{Block, BorderType, Clear, Paragraph, Tabs, Widget, Wrap},
};

const WHITE: Color = Color::White;
const BLACK: Color = Color::Black;
const GRAY: Color = Color::Gray;
use strum::IntoEnumIterator;

use crate::{app::App, areas::SelectedArea, tabs::get_help_categories, tabs::SelectedTab};

impl Widget for &mut App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        // Main app laytouts
        let [sidebar_area, view_area] = Layout::horizontal([Percentage(20), Percentage(80)])
            .margin(1)
            .areas(area);

        // view_area
        let vertical = Layout::vertical([Length(1), Min(0), Length(1)]);
        let [header_area, inner_area, footer_area] = vertical.areas(view_area);

        // top bar
        let horizontal = Layout::horizontal([Min(0), Length(20)]);
        let [tabs_area, title_area] = horizontal.areas(header_area);

        // inner_area
        let [url_area, selected_tab_area] = Layout::vertical([Length(3), Min(0)]).areas(inner_area);

        self.render_url(url_area, buf);
        self.render_sidebar(
            self.new_collection_name_value.clone(),
            self.collection_popup,
            self.selected_area,
            sidebar_area,
            area,
            buf,
        );
        self.render_tabs(tabs_area, buf);
        self.render_selected_tab(selected_tab_area, inner_area, buf);

        if self.help_popup {
            let popup_layout = Layout::vertical([
                Constraint::Percentage(20),
                Constraint::Percentage(60),
                Constraint::Percentage(20),
            ])
            .split(area);

            let popup_area = Layout::horizontal([
                Constraint::Percentage(10),
                Constraint::Percentage(80),
                Constraint::Percentage(10),
            ])
            .split(popup_layout[1])[1];

            Clear.render(popup_area, buf);

            let categories = get_help_categories();
            let mid = (categories.len() + 1) / 2;
            let left_categories = &categories[..mid];
            let right_categories = &categories[mid..];

let _block = Block::bordered()
                .title(" Help ")
                .border_type(BorderType::Plain)
                .border_style(WHITE)
                .fg(WHITE);

            let inner = _block.inner(popup_area);
            _block.render(popup_area, buf);
            let [left_inner, right_inner] = Layout::horizontal([Constraint::Percentage(50), Constraint::Percentage(50)])
                .areas(inner);

            let mut left_content = String::new();
            for category in left_categories {
                left_content.push_str(&format!("{}:\n", category.name));
                for bind in &category.keybinds {
                    left_content.push_str(&format!("  {}\n", bind));
                }
            }
            Paragraph::new(left_content.trim())
                .style(WHITE)
                .wrap(Wrap { trim: false })
                .render(left_inner, buf);

            let mut right_content = String::new();
            for category in right_categories {
                right_content.push_str(&format!("{}:\n", category.name));
                for bind in &category.keybinds {
                    right_content.push_str(&format!("  {}\n", bind));
                }
            }
            Paragraph::new(right_content.trim())
                .style(WHITE)
                .wrap(Wrap { trim: false })
                .render(right_inner, buf);
        }

        render_title(title_area, buf);
        render_footer(footer_area, buf, self.moving, self.selected_area);
    }
}

impl App {
    pub fn render_tabs(&self, area: Rect, buf: &mut Buffer) {
        let titles = SelectedTab::iter().map(SelectedTab::title);
        let highlight_style = (BLACK, GRAY);
        let selected_tab_index = self.selected_tab as usize;
        Tabs::new(titles)
            .style(WHITE)
            .highlight_style(highlight_style)
            .select(selected_tab_index)
            .padding("", "")
            .divider(" ")
            .render(area, buf);
    }

    pub fn render_selected_tab(&mut self, area: Rect, full_area: Rect, buf: &mut Buffer) {
        match self.selected_tab {
            SelectedTab::Params => self.selected_tab.render_params(
                &mut self.params,
                area,
                buf,
                self.param_popup,
                self.seleted_param_feild,
                self.param_key_value.clone(),
                self.param_value_value.clone(),
                full_area,
                self.param_delete_popup,
            ),
            SelectedTab::Headers => self.selected_tab.render_headers(
                &mut self.headers,
                area,
                buf,
                self.header_popup,
                self.selected_header_feild,
                self.header_key_value.clone(),
                self.header_value_value.clone(),
                full_area,
                self.header_delete_popup,
            ),
            SelectedTab::Auth => self.selected_tab.render_auth(
                area,
                buf,
                self.selected_auth_feild,
                self.auth_holder_value.clone(),
                self.auth_key_value.clone(),
            ),

            SelectedTab::Body => {
                self.selected_tab
                    .render_body(self.selected_area, &self.body_textarea, area, buf)
            }
            SelectedTab::Result => self.selected_tab.render_result(
                self.selected_area,
                &self.result,
                self.result_scroll,
                area,
                buf,
            ),
        }
    }

    pub fn render_url(&self, area: Rect, buf: &mut Buffer) {
        let highlight_color = if SelectedArea::Url == self.selected_area {
            GRAY
        } else {
            BLACK
        };

        let block = Block::bordered()
            .title(" URL ")
            .fg(highlight_color)
            .border_type(BorderType::Plain);

        let inner = block.inner(area);
        block.render(area, buf);
        self.url_textarea.render(inner, buf);
    }
}

fn render_title(area: Rect, buf: &mut Buffer) {
    "Rquest".bold().fg(WHITE).render(area, buf);
}

fn render_footer(area: Rect, buf: &mut Buffer, _moving: bool, selected_area: SelectedArea) {
    let area_name = selected_area.to_string();
    let footer_text = format!(
        "Area: {} | j/k to navigate | C-c to exit edit mode",
        area_name
    );
    Line::raw(footer_text)
        .fg(WHITE)
        .centered()
        .render(area, buf);
}
