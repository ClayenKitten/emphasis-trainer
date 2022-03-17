use std::{str::FromStr, collections::HashMap};

use thiserror::Error;

use crate::util;

use super::Word;

pub fn parse(s: &str) -> (Vec<Word>, Vec<ParseError>) {
    let mut words = Vec::with_capacity(s.lines().count());
    let mut explanations = HashMap::new();
    let mut errors = Vec::new();
    for (line, text) in s.lines().enumerate() {
        if let Some(res) = parse_line(line, text, &explanations) {
            match res {
                ParseResult::Word(w) => words.push(w),
                ParseResult::Explanation(tag, text) => { explanations.insert(tag, text); },
                ParseResult::Error(err) => errors.push(err),
            }
        }
    }
    (words, errors)
}

fn parse_line(line: usize, text: &str, explanations: &HashMap<String, String>) -> Option<ParseResult> {
    let text = text.trim();
    if !should_parse(text) {
        return None;
    }
    let res = match text.strip_prefix('>') {
        Some(text) => {
            Explanation::from_str(text.trim_start())
                .map_err(|source| ParseError::new_explanation(line, source))
                .into()
        }
        None => {
            parse_word(text, explanations)
                .map_err(|source| ParseError::new_word(line, source))
                .into()
        }
    };
    Some(res)
}

fn should_parse(s: &str) -> bool {
    !s.is_empty() && !s.starts_with("//")
}

fn parse_word(line: &str, explanations: &HashMap<String, String>) -> Result<Word, WordParseError> {
    let (word, left) = line.split_once(|c: char| c.is_whitespace())
            .map(|(word, left)| (word.trim(), Some(left.trim())))
            .unwrap_or_else(|| (line, None));
    let emphasis = util::first_uppercase_position(word)
        .ok_or(WordParseError::EmphasisNotFound(word.to_string()))?;
    let mut word = Word::new(word, emphasis);

    if let Some(left) = left {
        // Detail
        if let Some(detail) = util::subslice_tags(left, &[], &[':', '!', '>', '<']) {
            word = word.with_detail(&detail);
        }
        // Group
        if left.chars().filter(|c| [':', '!'].contains(c)).count() > 1 {
            return Err(WordParseError::MoreThanOneGroup(word.inner().to_owned()));
        }
        if let Some(group) = util::subslice_tags(left, &[':', '!'], &['>', '<']) {
            let inverted = left.contains('!');
            word = word.with_group(group.trim(), inverted)
        }
        // Explanation
        match util::subslice_tags(left, &['>'], &[]) {
            Some(exp_tag) => {
                let exp_tag = exp_tag.trim().to_lowercase();
                if let Some(exp) = explanations.get(&exp_tag) {
                    word = word.with_explanation(exp.trim());
                } else {
                    return Err(WordParseError::ExplanationNotDefined { tag: exp_tag, word: word.inner().to_owned() });
                }
            }
            None => {
                let exp: String = left.chars()
                    .skip_while(|c| *c != '<')
                    .skip(1)
                    .collect();
                if !exp.is_empty() {
                    word = word.with_explanation(exp.trim());
                }
            }
        };
    }
    Ok(word)
}

#[derive(Debug)]
enum ParseResult {
    Word(Word),
    Explanation(String, String),
    Error(ParseError),
}

impl Into<ParseResult> for Result<Word, ParseError> {
    fn into(self) -> ParseResult {
        match self {
            Ok(w) => ParseResult::Word(w),
            Err(e) => ParseResult::Error(e),
        }
    }
}

impl Into<ParseResult> for Result<Explanation, ParseError> {
    fn into(self) -> ParseResult {
        match self {
            Ok(exp) => ParseResult::Explanation(exp.tag, exp.text),
            Err(e) => ParseResult::Error(e),
        }
    }
}

#[derive(Debug, Error, PartialEq, Eq)]
#[error("Line {}: {}", line, inner)]
pub enum ParseError {
    WordParseError { line: usize, inner: WordParseError },
    ExplanationParseError { line: usize, inner: ExplanationParseError },
}

impl ParseError {
    pub fn new_word(line: usize, inner: WordParseError) -> ParseError {
        ParseError::WordParseError { line, inner }
    }

    pub fn new_explanation(line: usize, inner: ExplanationParseError) -> ParseError {
        ParseError::ExplanationParseError { line, inner }
    }    
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum ExplanationParseError {
    #[error("`:` delimiter not found.")]
    DelimiterNotFound,
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum WordParseError {
    #[error("Word must contain emphasis specified by uppercase letter; Word `{0}` has no emphasis.")]
    EmphasisNotFound(String),
    #[error("Word `{0}` has multiple groups defined. Currently only one group allowed.")]
    MoreThanOneGroup(String),
    #[error("Explanation tag `{tag}` for `{word}` not defined.")]
    ExplanationNotDefined {tag: String, word: String },
    #[error("Explanation tag not found although it was expected.")]
    NoExplanationTag,
    #[error("Explanation can't be empty.")]
    ExplanationEmpty,
}

/// Binding between tag and text explaining accentuation.
#[derive(Debug, Clone, PartialEq, Eq)]
struct Explanation {
    tag: String,
    text: String,
}

impl FromStr for Explanation {
    type Err = ExplanationParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {    
        let (tag, text) = s.split_once(':')
            .ok_or(ExplanationParseError::DelimiterNotFound)?;
        Ok(Explanation {
            tag: tag.trim().to_lowercase(),
            text: text.trim().to_string(),
        })
    }
}

#[cfg(test)]
mod test {
    use crate::model::{parse::parse, Word};

    #[test]
    fn test_detail() {
        let data = "
        отзЫв (посла)
        Отзыв (о книге)
        ";
        let correct = (
            vec![
                Word::new("отзыв", 3).with_detail("(посла)"),
                Word::new("отзыв", 0).with_detail("(о книге)"),
            ],
            Vec::new(),
        );
        assert_eq!(parse(data), correct);
    }

    #[test]
    fn test_group() {
        let data = "
        водопровОд : ПРОВОД
        газопровОд : ПРОВОД
        нефтепровОд : ПРОВОД
        ";
        let correct = (
            vec![
                Word::new("водопровод", 8).with_group("ПРОВОД", false),
                Word::new("газопровод", 8).with_group("ПРОВОД", false),
                Word::new("нефтепровод", 9).with_group("ПРОВОД", false),
            ],
            Vec::new(),
        );
        assert_eq!(parse(data), correct);
    }

    #[test]
    fn test_explanation() {
        let data = "
        > ПРОВЕРКА: Просто проверка работоспособности.
        слОво > ПРОВЕРКА
        ";
        let correct = (
            vec![
                Word::new("слово", 2).with_explanation("Просто проверка работоспособности."),
            ],
            Vec::new(),
        );
        assert_eq!(parse(data), correct);
    }
}
