/// Determines whether the supplied string is a valid ISBN number
pub fn is_valid_isbn(isbn: &str) -> bool {
    isbn.chars()
        .filter(|&c| c != '-')
        .enumerate()
        .map(|i_and_c| match i_and_c {
            (9, 'X') => Some(10),
            (_, c) => c.to_digit(10)
        })
        .collect::<Option<Vec<u32>>>()
        .map(|digits| {
            if digits.len() == 10 {
                digits.iter()
                    .enumerate()
                    .map(|(i, d)| d * (10 - i as u32))
                    .sum::<u32>() % 11 == 0
            } else { false }
        }).unwrap_or(false)
}
