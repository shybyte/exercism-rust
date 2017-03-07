const ALPHABET_LENGTH: u8 = 26;

fn rotate_u8(x: u8, min: u8, key: u8) -> char {
    ((x - min + key) % ALPHABET_LENGTH + min) as char
}

fn rotate_char(c: char, key: u8) -> char {
    match c {
        'a' ... 'z' => rotate_u8(c as u8, b'a', key),
        'A' ... 'Z' => rotate_u8(c as u8, b'A', key),
        _ => c
    }
}

pub fn rotate(text: &str, key: u8) -> String {
    text.chars().map(|c| rotate_char(c, key)).collect()
}