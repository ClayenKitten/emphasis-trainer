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

pub fn first_uppercase_position(s: &str) -> Option<usize> {
    s.chars().position(|c| c.is_uppercase())
}
