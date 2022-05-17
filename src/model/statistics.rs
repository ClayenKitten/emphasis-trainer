use chrono::{DateTime, Utc};
use gloo::storage::{LocalStorage, Storage, errors::StorageError};
use indexmap::IndexMap;
use rand::seq::IteratorRandom;
use serde::{Serialize, Deserialize};
use thiserror::Error;

use super::{WordHash, CardResult};

/// Stats struct stores mapping between word and its progression.
pub struct Stats(IndexMap<WordHash, Record>);



impl Stats {
    const KEY: &'static str = "words-stats";

    pub fn new(words: Vec<WordHash>) -> Self {
        let new: IndexMap<WordHash, Record> = words.into_iter()
            .map(|h| { (h, Record::default()) })
            .collect();
        let mut new = Stats(new);
        new.sync();
        new
    }

    /// Load statistics from LocalStorage.
    fn load() -> Self {
        let stored = match LocalStorage::get(Self::KEY) {
            Ok(val) => val,
            Err(StorageError::KeyNotFound(_) | StorageError::SerdeError(_)) => {
                let map = IndexMap::new();
                match LocalStorage::set(Self::KEY, &map) {
                    Err(StorageError::JsError(e)) => gloo::console::error!(format!("LocalStorage is not functional {e}")),
                    Err(StorageError::SerdeError(_)) => unreachable!(),
                    Err(StorageError::KeyNotFound(_)) => unreachable!(),
                    _ => (),
                }
                map
            },
            Err(StorageError::JsError(e)) => panic!("JS error occured: {}", e),
        };
        Stats(stored)
    }

    /// Get a following word to train by.
    pub fn next(&mut self) -> WordHash {
        if let Some((word, record)) = self.0.iter_mut().choose(&mut rand::thread_rng()) {
            record.occured();
            *word
        } else {
            panic!("Word list is empty.");
        }
    }

    /// Update priority of word depending on card pass result.
    pub fn passed(&mut self, word: WordHash, result: CardResult) {
        if let Some(record) = self.0.get_mut(&word) {
            match result {
                CardResult::Solved => {
                    record.group.promote();
                }
                CardResult::Failed => {
                    record.group.demote();
                }
            }
            self.sync()
        }
    }

    fn sync(&mut self) {
        let current = &self.0;
        let mut stored = Self::load().0;
        if current == &stored {
            return;
        }
        stored.extend(current);
        self.0 = stored;

        match LocalStorage::set(Self::KEY, &self.0) {
            Ok(_) => { },
            Err(StorageError::KeyNotFound(_)) => unreachable!(),
            Err(StorageError::SerdeError(e)) => panic!("Serde error occured: {}", e),
            Err(StorageError::JsError(e)) => panic!("JS error occured: {}", e),
        };
        gloo::console::log!("Stats synced.");
    }
}

#[derive(Debug, Clone, Error)]
pub enum StatisticsError {
    #[error("No entries stored in statistics.")]
    NoEntries,
    #[error("Statistics data is in invalid state.")]
    Invalid,
}

/// Record contains statistical data about one word.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(default)]
struct Record {
    last_occured: Option<DateTime<Utc>>,
    group: Group,
}

impl Record {
    /// Create new record.
    pub fn new() -> Self {
        Self { last_occured: None, group: Group::new() }
    }
    /// Update inner timer of record.
    pub fn occured(&mut self) {
        self.last_occured = Some(Utc::now())
    }

    /// Returns true if time since last word repetition is longer than repetition period.
    pub fn should_repeat(&self) -> bool {
        let now = Utc::now();
        if let Some(last_occured) = self.last_occured {
            if last_occured > now {
                false
            } else {
                (now - last_occured).num_days() > self.group.repetition_days() as i64
            }
        } else {
            false
        }
    }
}

/// Words are defined in 8 groups
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Group(u8);

impl Group {
    pub fn new() -> Self {
        Group(0)
    }

    pub fn promote(&mut self) {
        if self.0 < 7 {
            self.0 += 1;
        }
    }

    pub fn demote(&mut self) {
        self.0 = self.0.saturating_sub(1);
    }

    pub fn repetition_days(&self) -> u32 {
        match self.0 {
            0 => 1,
            1 => 2,
            2 => 3,
            3 => 5,
            4 => 10,
            5 => 30,
            6 => 60,
            7 => 90,
            _ => unreachable!(),
        }
    }
}
