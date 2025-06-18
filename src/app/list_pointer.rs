use ratatui::widgets::{ListState, TableState};
use std::iter::{DoubleEndedIterator, ExactSizeIterator};

#[derive(Debug)]
pub enum ScrollError {
    EndOfList,
    EmptyList,
}

#[derive(Debug, Copy, Clone)]
pub struct ListPointer {
    offset: usize,
    selected: Option<usize>,
    paused: bool,
    length: usize,
}

impl ListPointer {
    pub fn new(length: usize) -> Self {
        let selected = match length {
            0 => None,
            _ => Some(0),
        };
        Self {
            offset: 0,
            selected,
            paused: false,
            length,
        }
    }

    pub fn length(&self) -> usize {
        self.length
    }

    pub fn selected(&self) -> Option<usize> {
        if self.paused {
            None
        } else {
            self.selected
        }
    }

    pub fn delete_current(&mut self) {
        todo!()
    }

    pub fn select(&mut self, i: Option<usize>) -> Result<(), ScrollError> {
        if let Some(i) = i {
            if i < self.length {
                self.selected = Some(i);
                Ok(())
            } else {
                Err(ScrollError::EndOfList)
            }
        } else {
            self.selected = i;
            Ok(())
        }
    }

    pub fn pause(&mut self) {
        self.paused = true;
    }

    pub fn unpause(&mut self) {
        self.paused = false;
    }

    pub fn try_next(&mut self) -> Result<(), ScrollError> {
        if let Some(i) = self.selected {
            if i + 1 >= self.length {
                Err(ScrollError::EndOfList)
            } else {
                self.selected = Some(i + 1);
                Ok(())
            }
        } else {
            Err(ScrollError::EmptyList)
        }
    }

    pub fn try_prev(&mut self) -> Result<(), ScrollError> {
        if let Some(i) = self.selected {
            if i == 0 {
                Err(ScrollError::EndOfList)
            } else {
                self.selected = Some(i - 1);
                Ok(())
            }
        } else {
            Err(ScrollError::EmptyList)
        }
    }

    /// Takes an iterator parallel to the item that somehow defines which items should
    /// be skipped (false ones).
    pub fn try_next_selectable(
        &mut self,
        selectable: impl Iterator<Item = bool>,
    ) -> Result<(), ScrollError> {
        match self.selected {
            Some(i) => {
                let j = selectable
                    .enumerate()
                    .skip(i + 1)
                    .find_map(|(i, s)| if s { Some(i) } else { None });
                match j {
                    Some(_) => {
                        self.selected = j;
                        Ok(())
                    }
                    None => Err(ScrollError::EndOfList),
                }
            }
            None => Err(ScrollError::EmptyList),
        }
    }

    /// Takes an iterator parallel to the item that somehow defines which items should
    /// be skipped (false ones).
    pub fn try_prev_selectable(
        &mut self,
        selectable: impl DoubleEndedIterator<Item = bool> + ExactSizeIterator,
    ) -> Result<(), ScrollError> {
        match self.selected {
            Some(i) => {
                let j =
                    selectable
                        .enumerate()
                        .take(i)
                        .rev()
                        .find_map(|(i, s)| if s { Some(i) } else { None });
                match j {
                    Some(_) => {
                        self.selected = j;
                        Ok(())
                    }
                    None => Err(ScrollError::EndOfList),
                }
            }
            None => Err(ScrollError::EmptyList),
        }
    }

    /// Announces to the pointer that an item has been added anywhere BEFORE the pointer. This command
    /// preserves which item the pointer points to.
    pub fn prepend_item(&mut self) {
        if let Some(i) = self.selected {
            self.selected = Some(i + 1);
        }
        self.length += 1;
    }

    /// Announces to the pointer that an item has been added anywhere AFTER the pointer. Does
    /// not impact which item the pointer points to.
    pub fn append_item(&mut self) {
        self.length += 1;
    }
}

impl From<ListPointer> for ListState {
    fn from(val: ListPointer) -> Self {
        ListState::default()
            .with_offset(val.offset)
            .with_selected(val.selected)
    }
}

impl From<ListPointer> for TableState {
    fn from(val: ListPointer) -> Self {
        TableState::default()
            .with_offset(val.offset)
            .with_selected(val.selected)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hello() {
        panic!("dont forget to test this module");
    }
}
