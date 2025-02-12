use std::{collections::VecDeque, fmt};
use crate::document::Document;

#[non_exhaustive]
#[derive(Debug)]
pub enum UndoError {

}

impl fmt::Display for UndoError {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}

impl std::error::Error for UndoError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}

#[non_exhaustive]
#[derive(Debug)]
pub enum RedoError {

}

impl fmt::Display for RedoError {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}

impl std::error::Error for RedoError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}

#[non_exhaustive]
#[derive(Debug)]
pub enum Action {

}

impl fmt::Display for Action {
    /// The name for the type of change (what would be displayed in the edit history panel)
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}

impl Action {
    pub fn redo(&mut self, _document: &mut Document) -> Result<(), RedoError> {
        todo!()
    }

    pub fn undo(&mut self, _document: &mut Document) -> Result<(), UndoError> {
        todo!()
    }
}

pub(super) struct EditHistory {
    changes: VecDeque<Action>,
    present: usize,
}

impl EditHistory {
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            changes: VecDeque::with_capacity(capacity),
            present: 0,
        }
    }

    /// Apply a change that can be undone/redone and add it to the history
    pub fn push_change(&mut self, change: Action) {
        for _ in 0..self.present {
            self.changes.pop_front();
        }
        self.present = 0;
        if self.changes.len() == self.changes.capacity() {
            self.changes.pop_back();
        }
        self.changes.push_front(change);
    }

    pub fn prev(&mut self) -> Option<&mut Action> {
        let change = self.changes.get_mut(self.present);
        if change.is_some() { self.present += 1; }
        change
    }

    pub fn next(&mut self) -> Option<&mut Action> {
        if let Some(present) = self.present.checked_sub(1) {
            self.present = present;
            return Some(&mut self.changes[present])
        }
        None
    }
}
