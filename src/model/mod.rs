mod parse;
mod statistics;
mod variant;
mod word;

use indexmap::IndexMap;
use rand::Rng;

pub use self::parse::ParseError;
pub use self::{word::{Word, WordHash}, variant::Variant};

/// Struct that manages whole logic of trainer.
pub struct Model {
    latest: Option<WordHash>,
    words: IndexMap<WordHash, Word>,
}

impl Model {
    /// Create new model.
    pub fn new() -> (Self, Vec<ParseError>) {
        let data = include_str!("./data.txt");
        let (words, errors) = parse::parse(data);
        let words = words.into_iter()
            .map(|word| (WordHash::from(&word), word))
            .collect();
        
        let model = Model {
            latest: None,
            words,
        };
        (model, errors)
    }

    /// Get new word.
    pub fn next(&self) -> Word {
        let mut rng = rand::thread_rng();
        let i: usize = rng.gen_range(0..self.words.len());
        self.words[i].clone()
    }

    /// Get words with the same group and rule.
    pub fn seealso(&self, word: &Word) -> Vec<Word> {
        let group = match word.group {
            Some(group) => group,
            None => return Vec::new(),
        };
        self.words.values()
            .filter(|w| w.group.map(|g| g == group).unwrap_or(false))
            .map(|w| w.clone())
            .filter(|w| w != word)
            .collect()
    }

    /// Get word with the same group but opposite rule.
    pub fn opposite(&self, word: &Word) -> Vec<Word> {
        let group = match word.group {
            Some(group) => group,
            None => return Vec::new(),
        };
        self.words.values()
            .filter(|w| w.group.map(|g| g.0 == !group.0 && g.1 == group.1).unwrap_or(false))
            .map(|w| w.clone())
            .filter(|w| w != word)
            .collect()
    }
}
