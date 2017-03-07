use std::collections::HashSet;

pub fn is_pangram(text: &str) -> bool {
    text.to_lowercase()
        .chars()
        .filter(|&c| 'a' <= c && c <= 'z')
        .collect::<HashSet<char>>()
        .len() == 26
}
