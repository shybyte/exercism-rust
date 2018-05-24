pub fn get_diamond(max_char: char) -> Vec<String> {
    let diamond_size = (((max_char as u8) - b'A') * 2 + 1) as usize;
    let mut diamond: Vec<String> = (b'A'..(max_char as u8 + 1))
        .enumerate()
        .map(|(row_index, current_char_u8)| {
            let mut row_chars = vec![' '; diamond_size];
            row_chars[diamond_size / 2 - row_index] = current_char_u8 as char;
            row_chars[diamond_size / 2 + row_index] = current_char_u8 as char;
            row_chars.iter().collect()
        }).collect();

    let reflection: Vec<String> = diamond[0..(diamond.len() - 1)].iter().cloned().rev().collect();
    diamond.extend(reflection);

    diamond
}
