use std::collections::HashMap;

pub fn can_construct_note(magazine: &[&str], note: &[&str]) -> bool {
    let mut word_counts = HashMap::new();

    // Count the words in the magazine
    for word in magazine {
        *word_counts.entry(word).or_insert(0) += 1;
    }

    // Check if we can construct the note
    for word in note {
        if let Some(count) = word_counts.get_mut(word) {
            if *count > 0 {
                *count -= 1;
            } else {
                return false;
            }
        } else {
            return false;
        }
    }

    true
}

