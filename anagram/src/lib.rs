use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let mut anagrams = HashSet::new();
    let lowercase_word = word.to_lowercase();
    let word_characters = word_to_sorted_chars(&lowercase_word);

    for possible_anagram in possible_anagrams {
        let lowercase_possible_anagram = possible_anagram.to_lowercase();
        let possible_anagram_characters = word_to_sorted_chars(&lowercase_possible_anagram);

        if possible_anagram_characters == word_characters
            && lowercase_word != lowercase_possible_anagram
        {
            anagrams.insert(*possible_anagram);
        }
    }

    anagrams
}

fn word_to_sorted_chars(word: &str) -> Vec<char> {
    let mut word_characters: Vec<char> = word.chars().collect();
    word_characters.sort();
    word_characters
}
