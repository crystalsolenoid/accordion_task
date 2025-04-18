use ratatui::widgets::ListState;

pub struct ListPointer {
    pub offset: usize,
    pub selected: usize,
    pub length: usize,
}

impl Into<ListState> for ListPointer {
    fn into(self) -> ListState {
        ListState::Default()
            .with_offset(self.offset)
            .with_selected(Some(self.selected))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hello() {
        panic!();
    }
}
