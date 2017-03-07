use std::collections::HashMap;

pub fn word_count(text: &str) -> HashMap<String, u32> {
    let cleaned_text: String = text.chars()
        .filter(|&c| c.is_numeric() || c.is_alphabetic() || c.is_whitespace())
        .collect();
    let mut result: HashMap<String, u32> = HashMap::new();
    for word in cleaned_text.to_lowercase().split_whitespace() {
        let count = result.entry(word.to_string()).or_insert(0);
        *count += 1;
    }
    result
}
