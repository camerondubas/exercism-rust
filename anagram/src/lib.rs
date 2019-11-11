use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let mut anagrams = HashSet::new();
    let lowercase_word = word.to_lowercase();
    let mut word_characters: Vec<char> = lowercase_word.chars().collect();

    word_characters.sort();

    for possible_anagram in possible_anagrams {
        let lowercase_possible_anagram = possible_anagram.to_lowercase();
        let mut possible_anagram_characters: Vec<char> =
            lowercase_possible_anagram.chars().collect();

        possible_anagram_characters.sort();

        if possible_anagram_characters == word_characters
            && lowercase_word != lowercase_possible_anagram
        {
            anagrams.insert(possible_anagram.clone());
        }
    }

    anagrams
}
