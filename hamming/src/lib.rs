pub fn hamming_distance(s1: &str, s2: &str) -> Result<usize, String> {
    if s1.len() == s2.len() {
        Ok(s1.chars()
            .zip(s2.chars())
            .filter(|&(c1, c2)| c1 != c2)
            .count())
    } else {
        Err("s1 and s2 must have same length.".to_string())
    }
}
