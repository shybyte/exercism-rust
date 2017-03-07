pub fn abbreviate(text: &str) -> String {
    let chars1 = " ".chars().chain(text.chars());
    let chars2 = text.chars();
    chars1.zip(chars2)
        .filter_map(|char_and_next_char| {
            match char_and_next_char {
                (c1, c2) if (!c1.is_uppercase() && c2.is_uppercase()) ||
                            ((!c1.is_alphabetic() && c2.is_alphabetic())) => Some(c2),
                _ => None,
            }
        })
        .collect::<String>()
        .to_uppercase()
}
