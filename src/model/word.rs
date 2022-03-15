use std::{fmt::Display, hash::Hash};

use crate::util;

use super::variant::Variant;

/// Correct way to set emphasis at `word`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Word {
    /// Word in lowercase.
    pub word: String,
    /// Detail that defines correct emphasis.
    pub detail: Option<String>,
    /// Position of correct emphasis.
    pub emphasis: usize,
    /// Words with same seealso value are shown after failure.
    pub group: Option<(bool, u64)>,
    /// Explanation with presented tag shown after failute.
    pub explanation: Option<String>,
}

impl Word {
    pub fn new(word: &str, emphasis: usize) -> Self {
        Word {
            word: word.to_lowercase(),
            detail: None,
            emphasis,
            group: None,
            explanation: None,
        }
    }
    
    pub fn with_detail(mut self, detail: &str) -> Self {
        self.detail = Some(detail.trim().to_string());
        self
    }
    
    pub fn with_group(mut self, group: &str, inverted: bool) -> Self {
        self.group = Some((inverted, fxhash::hash64(&group.to_lowercase())));
        self
    }

    pub fn with_explanation(mut self, explanation: impl ToString) -> Self {
        self.explanation = Some(explanation.to_string());
        self
    }

    /// Get inner word in lowercase without any details etc.
    pub fn inner(&self) -> &str {
        &self.word
    }

    pub fn variants(&self) -> Vec<Variant> {
        util::get_vowel_positions(&self.word)
            .into_iter()
            .map(|emphasis| Variant {
                emphasis,
                word: self.word.clone(),
                detail: self.detail.clone(),
            })
            .collect()
    }
}

impl Display for Word {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let word = util::uppercase_letter(&self.word, self.emphasis);
        if let Some(detail) = &self.detail {
            write!(f, "{} {}", word, detail)
        } else {
            write!(f, "{}", word)
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct WordHash(u64);

impl From<&Word> for WordHash {
    fn from(val: &Word) -> Self {
        Self(fxhash::hash64(&val.word))
    }
}
