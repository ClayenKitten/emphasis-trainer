use gloo::storage::{LocalStorage, Storage, errors::StorageError};

use super::Word;

/// Stats struct stores mapping between word and its progression.
pub struct Stats;

impl Stats {
    pub fn get(word: &Word) -> Priority {
        match LocalStorage::get(word.inner()) {
            Ok(val) => Priority(val),
            Err(StorageError::KeyNotFound(_)) => Priority::default(),
            Err(StorageError::SerdeError(_)) => Priority::default(),
            Err(StorageError::JsError(e)) => panic!("JS error occured: {}", e),
        }
    }

    /// Update priority of word if it was solved correctly.
    pub fn solved(word: &Word) {
        todo!();
    }

    /// Update priority of word if it was solved incorrectly.
    pub fn failed(word: &Word) {
        todo!();
    }

    fn set(word: &Word, priority: Priority) {
        match LocalStorage::set(word.inner(), priority.0) {
            Ok(_) => todo!(),
            Err(StorageError::KeyNotFound(_)) => unreachable!(),
            Err(StorageError::SerdeError(e)) => panic!("Serde error occured: {}", e),
            Err(StorageError::JsError(e)) => panic!("JS error occured: {}", e),
        }
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Priority(f32);
