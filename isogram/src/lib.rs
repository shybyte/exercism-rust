use std::collections::HashSet;

pub fn check(s: &str) -> bool {
    let letters = s.chars()
        .filter(|c| c.is_alphabetic())
        .flat_map(|c| c.to_lowercase());

    let mut letter_set = HashSet::new();

    for letter in letters {
        if letter_set.contains(&letter) {
            return false;
        } else {
            letter_set.insert(letter);
        }
    }

    true
}