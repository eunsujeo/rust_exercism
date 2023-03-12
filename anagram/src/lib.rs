use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, candidates: &[&'a str]) -> HashSet<&'a str> {
    let mut result: HashSet<&'a str> = HashSet::new();
    let sorted_word = get_sorted_chars(word);
    for candidate in candidates {
        if candidate.to_lowercase() == word.to_lowercase() {
            continue;
        }
        if sorted_word == get_sorted_chars(candidate) {
            result.insert(candidate);
        }
    }
    result
}

fn get_sorted_chars(s: &str) -> Vec<char> {
    let mut chars: Vec<char> = s.to_lowercase().chars().collect();
    chars.sort_unstable();
    chars
}

