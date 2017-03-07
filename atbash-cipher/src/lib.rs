pub fn decode(text: &str) -> String {
    text.chars()
        .filter_map(|c| match c {
            'a'...'z' => Some((b'z' - c as u8 + b'a') as char),
            '0'...'9' => Some(c),
            _ => None,
        })
        .collect::<String>()
}

pub fn encode(text: &str) -> String {
    chunks(decode(&text.to_lowercase()), 5)
}

fn chunks(s: String, chunk_len: usize) -> String {
    s.chars()
        .collect::<Vec<_>>()
        .chunks(chunk_len)
        .map(|chunk| chunk.iter().cloned().collect())
        .collect::<Vec<String>>()
        .join(" ")
}
