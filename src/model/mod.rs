mod parse;
mod statistics;
mod variant;
mod word;

use indexmap::IndexMap;

pub use self::parse::ParseError;
use self::statistics::Stats;
pub use self::{
    variant::Variant,
    word::{Word, WordHash},
};

/// Struct that manages whole logic of trainer.
pub struct Model {
    pub stats: Stats,
    latest: Option<WordHash>,
    words: IndexMap<WordHash, Word>,
}

impl Model {
    /// Create new model.
    pub fn new() -> (Self, Vec<ParseError>) {
        let data = include_str!("./data.txt");
        let (words, errors) = parse::parse(data);

        let stats = Stats::new(words.iter().map(|w| w.hash()).collect());
        let words = words
            .into_iter()
            .map(|word| (WordHash::from(&word), word))
            .collect();

        let model = Model {
            stats,
            latest: None,
            words,
        };
        (model, errors)
    }

    /// Get new word.
    pub fn next(&mut self) -> Word {
        if self.words.len() < 2 {
            let key = self.stats.next();
            self.words.get(&key).unwrap().clone()
        } else {
            loop {
                let key = self.stats.next();
                if Some(key) != self.latest {
                    break self.words.get(&key).unwrap().clone();
                }
            }
        }
    }

    /// Get words with the same group and rule.
    pub fn seealso(&self, word: &Word) -> Vec<Word> {
        let group = match word.group {
            Some(group) => group,
            None => return Vec::new(),
        };
        self.words
            .values()
            .filter(|w| w.group.map(|g| g == group).unwrap_or(false))
            .cloned()
            .filter(|w| w != word)
            .collect()
    }

    /// Get word with the same group but opposite rule.
    pub fn opposite(&self, word: &Word) -> Vec<Word> {
        let group = match word.group {
            Some(group) => group,
            None => return Vec::new(),
        };
        self.words
            .values()
            .filter(|w| {
                w.group
                    .map(|g| g.0 == !group.0 && g.1 == group.1)
                    .unwrap_or(false)
            })
            .cloned()
            .filter(|w| w != word)
            .collect()
    }
}

#[derive(Debug, Clone, Copy)]
pub enum CardResult {
    Solved,
    Failed,
}

#[cfg(test)]
mod test {
    use super::Model;

    #[test]
    fn test_all_data_loaded_correctly() {
        let (_, errors) = Model::new();
        assert!(errors.is_empty());
    }

    /// Test that word isn't shown twice in a row.
    #[test]
    fn test_words_dont_repeat() {
        let (mut model, _) = Model::new();
        let mut last = None;
        for _ in 0..5000 {
            let word = model.next();
            assert_ne!(Some(word.clone()), last);
            last = Some(word);
        }
    }
}
