use std::collections::HashSet;

/// Determine whether a sentence is a pangram.
pub fn is_pangram(sentence: &str) -> bool {
    let mut used_chars = HashSet::new();

    sentence
        .to_lowercase()
        .chars()
        .filter(|c| c.is_ascii_alphabetic())
        .for_each(|c| {
            used_chars.insert(c);
        });

    used_chars.len() == 26
}
