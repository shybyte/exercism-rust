pub fn build_proverb(words: Vec<&str>) -> String {
    match words.first() {
        Some(first_word) => {
            let for_want_lines: Vec<String> = words.windows(2)
                .map(|pair| format!("For want of a {} the {} was lost.\n", pair[0], pair[1])).collect();

            let for_what_object = {
                if words.len() >= 3 {
                    format!("{}{} {}", words[2], words[1], first_word)
                } else {
                    first_word.to_string()
                }
            };
            let last_line = format!("And all for the want of a {}.", for_what_object);

            for_want_lines.join("") + last_line.as_str()
        }
        None => String::new()
    }
}
