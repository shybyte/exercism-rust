fn sorted_chars(word: &str) -> Vec<char> {
    let mut vec: Vec<_> = word.chars().collect();
    vec.sort();
    vec
}

pub fn anagrams_for<'a>(word: &str, candidates: &[&'a str]) -> Vec<&'a str> {
    let word_lower_case = word.to_lowercase();
    let word_sorted_chars = sorted_chars(&word_lower_case);
    candidates.iter()
        .filter(|&candidate| {
            let candidate_lower_case = candidate.to_lowercase();
            word_lower_case != *candidate_lower_case &&
                word_sorted_chars == sorted_chars(&candidate_lower_case)
        })
        .map(|&candidate| candidate)
        .collect()
}