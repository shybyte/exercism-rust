pub fn is_vowel(c: char) -> bool {
    "aoiue".contains(c)
}

pub fn move_head_consonants_to_end(word: &str) -> String {
    if word.starts_with(is_vowel) || word.starts_with("yt") || word.starts_with("xr") {
        word.to_string()
    } else {
        if let Some(consonants_at_begin) = word.split(is_vowel).next() {
            let tail = &word[consonants_at_begin.len()..];
            if consonants_at_begin.ends_with('q') && tail.starts_with('u') {
                tail[1..].to_string() + consonants_at_begin + "u"
            } else {
                tail.to_string() + consonants_at_begin
            }
        } else {
            "".to_string()
        }
    }
}

pub fn translate_word(word: &str) -> String {
    move_head_consonants_to_end(word) + "ay"
}

pub fn translate(input: &str) -> String {
    input.split_whitespace().map(translate_word).collect::<Vec<_>>().join(" ")
}