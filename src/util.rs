/// Make letter of `s` at specified `pos` uppercase.
pub fn uppercase_letter(s: &str, pos: usize) -> String {
    s.chars()
        .enumerate()
        .flat_map(|(p, c)| {
            if p == pos {
                c.to_uppercase().collect()
            } else {
                vec![c]
            }
        })
        .collect()
}

/// Get position of all vowels in provided str.
pub fn get_vowel_positions(s: &str) -> Vec<usize> {
    const VOWELS: [char; 10] = ['а', 'у', 'о', 'и', 'э', 'ы', 'я', 'ю', 'е', 'ё'];
    s.to_lowercase()
        .chars()
        .enumerate()
        .filter(|(_, c)| VOWELS.contains(c))
        .map(|(p, _)| p)
        .collect()
}

/// Get position of first uppercase letter.
pub fn first_uppercase_position(s: &str) -> Option<usize> {
    s.chars().position(|c| c.is_uppercase())
}

/// Get subslice of str between tags.
pub fn subslice_tags(s: &str, opening: &[char], closing: &[char]) -> Option<String> {
    let s: String = if opening.is_empty() {
        s.chars().take_while(|c| !closing.contains(c)).collect()
    } else {
        s.chars()
            .skip_while(|c| !opening.contains(c))
            .skip(1)
            .take_while(|c| !closing.contains(c))
            .collect()
    };
    if s.is_empty() {
        None
    } else {
        Some(s)
    }
}

#[cfg(test)]
mod test {
    use super::subslice_tags;

    #[test]
    fn test_subslice_tags() {
        let data = subslice_tags("Hello, <world>", &['<'], &['>']);
        assert_eq!(data, Some(String::from("world")));
    }
}
