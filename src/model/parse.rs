use Either::{Left, Right};
use either::Either;
use thiserror::Error;

use crate::util;

use super::{Word, Explanation};

pub fn parse(s: &str) -> Result<(Vec<Word>, Vec<Explanation>), Vec<ParseError>> {
    let mut words = Vec::new();
    let mut explanations = Vec::new();
    let mut errors = Vec::new();
    for res in s.lines()
        .enumerate()
        .flat_map(|(line, text)| parse_line(line, text)) {
        match res {
            ParseResult::Word(w) => words.push(w),
            ParseResult::Explanation(e) => explanations.push(e),
            ParseResult::Error(err) => errors.push(err),
        }
    }
    if errors.is_empty() {
        Ok((words, explanations))
    } else {
        Err(errors)
    }
}

fn parse_line(line: usize, text: &str) -> Option<ParseResult> {
    let text = text.trim();
    if text.is_empty() || text.starts_with("//") {
        return None;
    }
    let res = match text.strip_prefix('>') {
        Some(text) => {
            parse_explanation(text.trim_start())
                .map_err(|source| ParseError { line, source: Right(source) })
                .into()
        }
        None => {
            parse_word(text)
                .map_err(|source| ParseError { line, source: Left(source) })
                .into()
        }
    };
    Some(res)
}

fn parse_explanation(line: &str) -> Result<Explanation, ExplanationParseError> {
    let (tag, text) = line.split_once(|c: char| c.is_ascii_whitespace())
        .ok_or(ExplanationParseError::ExplanationNotFound)?;
    Ok(Explanation::new(tag, text))
}

fn parse_word(line: &str) -> Result<Word, WordParseError> {
    let (word, left) = line.split_once(|c: char| c.is_whitespace())
            .map(|(word, left)| (word.trim(), Some(left.trim())))
            .unwrap_or_else(|| (line, None));
    let emphasis = util::first_uppercase_position(word)
        .ok_or(WordParseError::EmphasisNotFound(word.to_string()))?;
    let mut word = Word::new(word, emphasis);

    if let Some(left) = left {
        let detail: String =  left.chars()
            .take_while(|c| *c != ':' && *c != '!' && *c != '>')
            .collect();
        if !detail.is_empty() {
            word = word.with_detail(&detail);
        }

        let group: String = left.chars()
            .skip_while(|c| *c != ':' && *c != '!')
            .skip(1)
            .take_while(|c| *c != '>')
            .collect();
        if !group.is_empty() {
            let inverted = left.contains('!');
            word = word.with_group(group.trim(), inverted)
        }

        let explanation_tag: String = left.chars()
            .skip_while(|c| *c != '>')
            .skip(1)
            .collect();
        if !explanation_tag.is_empty() {
            word = word.with_explanation(explanation_tag.trim())
        }
    }
    Ok(word)
}

#[derive(Debug)]
enum ParseResult {
    Word(Word),
    Explanation(Explanation),
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
            Ok(exp) => ParseResult::Explanation(exp),
            Err(e) => ParseResult::Error(e),
        }
    }
}

#[derive(Debug, Error, PartialEq, Eq)]
#[error("{}: {}", line, source)]
pub struct ParseError {
    pub line: usize,
    pub source: Either<WordParseError, ExplanationParseError>,
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum ExplanationParseError {
    #[error("Explanation can't be empty.")]
    ExplanationNotFound,
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum WordParseError {
    #[error("Word must contain emphasis specified by uppercase letter.")]
    EmphasisNotFound(String),
    #[error("Currently only one group allowed.")]
    MoreThanOneGroup,
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
        let correct = Ok(
            (
                vec![
                    Word::new("отзыв", 3).with_detail("(посла)"),
                    Word::new("отзыв", 0).with_detail("(о книге)"),
                ],
                Vec::new()
            )
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
        let correct = Ok(
            (
                vec![
                    Word::new("водопровод", 8).with_group("ПРОВОД", false),
                    Word::new("газопровод", 8).with_group("ПРОВОД", false),
                    Word::new("нефтепровод", 9).with_group("ПРОВОД", false),
                ],
                Vec::new()
            )
        );
        assert_eq!(parse(data), correct);
    }
}
