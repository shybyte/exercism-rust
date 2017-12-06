pub fn series(digits: &str, len: usize) -> Vec<String> {
    if len == 0 {
        return vec!["".to_string(); digits.len() + 1];
    }

    let chars: Vec<char> = digits.chars().collect();

    chars.windows(len)
        .map(|window| window.iter().collect())
        .collect()
}
