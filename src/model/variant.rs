use std::fmt::Display;

use crate::util;

/// Variant is possibly incorrect way of setting emphasis at word.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Variant {
    pub emphasis: usize,
    pub word: String,
    pub detail: Option<String>,
}

impl Display for Variant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let word = self.word.replace('ё', "е");
        let word = util::uppercase_letter(&word, self.emphasis);
        if let Some(detail) = &self.detail {
            write!(f, "{} {}", word, detail)
        } else {
            write!(f, "{}", word)
        }
    }
}
