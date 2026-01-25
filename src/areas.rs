use strum::{Display, EnumIter, FromRepr};

#[derive(Default, Clone, Copy, Display, FromRepr, EnumIter, PartialEq)]
pub enum SelectedArea {
    #[default]
    #[strum(to_string = "sidebar")]
    Sidebar,
    #[strum(to_string = "tabs")]
    Tabs,
    #[strum(to_string = "url")]
    Url,
    #[strum(to_string = "tab_content")]
    TabArea,
}

impl SelectedArea {
    pub fn previous(self) -> Self {
        let current_index: usize = self as usize;
        let previous_index = current_index.saturating_sub(1);
        Self::from_repr(previous_index).unwrap_or(self)
    }

    pub fn next(self) -> Self {
        let current_index = self as usize;
        let next_index = current_index.saturating_add(1);
        Self::from_repr(next_index).unwrap_or(self)
    }
}
