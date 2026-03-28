use crate::app::App;
use color_eyre::Result;
mod app;
mod areas;
mod json;
mod render;
mod sidebar;
mod tabs;

fn main() -> Result<()> {
    // default states

    color_eyre::install()?;
    let terminal = ratatui::init();
    let app_result = App::default().run(terminal);
    ratatui::restore();
    app_result
}
