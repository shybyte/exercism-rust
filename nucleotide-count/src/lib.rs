use std::collections::HashMap;

pub fn count(char_needle: char, s: &str) -> usize {
    s.chars().filter(|&c| c == char_needle).count()
}

pub fn nucleotide_counts(s: &str) -> HashMap<char, usize> {
    "ACGT"
        .chars()
        .map(|c| (c, count(c, s)))
        .collect::<HashMap<char, usize>>()
}
