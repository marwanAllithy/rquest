use ratatui::{
    buffer::Buffer,
    layout::{
        Constraint::{Length, Min, Percentage},
        Layout, Rect,
    },
    style::{Color, Stylize, palette::tailwind},
    text::Line,
    widgets::{Block, BorderType, Padding, Paragraph, Tabs, Widget},
};
use strum::IntoEnumIterator;

use crate::{app::App, areas::SelectedArea, tabs::SelectedTab};

impl Widget for &mut App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        // Main app laytouts
        let [sidebar_area, view_area] = Layout::horizontal([Percentage(15), Percentage(85)])
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
        render_sidebar(sidebar_area, buf);
        render_title(title_area, buf);
        self.render_tabs(tabs_area, buf);
        self.render_selected_tab(selected_tab_area, buf);
        render_footer(footer_area, buf);
    }
}

impl App {
    pub fn render_tabs(&self, area: Rect, buf: &mut Buffer) {
        let titles = SelectedTab::iter().map(SelectedTab::title);
        let highlight_style = (Color::default(), self.selected_tab.palette().c500);
        let selected_tab_index = self.selected_tab as usize;
        Tabs::new(titles)
            .highlight_style(highlight_style)
            .select(selected_tab_index)
            .padding("", "")
            .divider(" ")
            .render(area, buf);
    }

    pub fn render_selected_tab(&mut self, area: Rect, buf: &mut Buffer) {
        match self.selected_tab {
            SelectedTab::Params => self.selected_tab.render_params(
                &mut self.params,
                area,
                buf,
                self.param_popup,
                self.seleted_param_feild,
                self.param_key_value.clone(),
                self.param_value_value.clone(),
            ),
            SelectedTab::Headers => self.selected_tab.render_headers(
                &mut self.headers,
                area,
                buf,
                self.header_popup,
                self.selected_header_feild,
                self.header_key_value.clone(),
                self.header_value_value.clone(),
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
                    .render_body(self.selected_area, self.body.clone(), area, buf)
            }
            SelectedTab::Result => {
                self.selected_tab
                    .render_result(self.selected_area, self.result.clone(), area, buf)
            }
        }
    }

    pub fn render_url(&self, area: Rect, buf: &mut Buffer) {
        let highlight_color = if SelectedArea::Url == self.selected_area {
            tailwind::GREEN.c200
        } else {
            tailwind::GREEN.c700
        };

        Paragraph::new(self.url_value.clone().as_str())
            .block(
                Block::bordered()
                    .title(" URL ")
                    .fg(highlight_color)
                    .border_type(BorderType::Rounded),
            )
            .render(area, buf);
    }
}
fn render_sidebar(area: Rect, buf: &mut Buffer) {
    Paragraph::new("this will be the sidebar")
        .block(
            Block::bordered()
                .fg(Color::Green)
                .padding(Padding::uniform(1))
                .border_type(BorderType::Rounded),
        )
        .render(area, buf);
}

fn render_title(area: Rect, buf: &mut Buffer) {
    "Rquest".bold().render(area, buf);
}

fn render_footer(area: Rect, buf: &mut Buffer) {
    Line::raw("◄ ► to change tab | Press q to quit")
        .centered()
        .render(area, buf);
}
