pub fn encrypt(input: &str) -> String {
    let normalized_input: Vec<char> = input.to_lowercase().chars()
        .filter(|c| c.is_alphabetic())
        .collect();
    let columns = ((normalized_input.len() as f64).sqrt().ceil()) as usize;
    let mut result = String::with_capacity(normalized_input.len());

    for column in 0..columns {
        let mut i = column;
        while i < normalized_input.len() {
            result.push(normalized_input[i]);
            i += columns
        }

        if { column < columns - 1 } {
            result.push(' ');
        }
    }

    result
}
