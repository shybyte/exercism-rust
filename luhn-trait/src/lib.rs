pub trait Luhn {
    fn valid_luhn(&self) -> bool;
}

impl<T> Luhn for T where
    T: ToString {
    fn valid_luhn(&self) -> bool {
        is_valid(&self.to_string())
    }
}

fn is_valid(text: &str) -> bool {
    let text_without_spaces = text.replace(" ", "");

    let digits: Vec<u32> = text_without_spaces.chars()
        .filter_map(|c| c.to_digit(10))
        .collect();

    if digits.len() <= 1 || digits.len() != text_without_spaces.len() {
        return false;
    }

    let sum: u32 = digits.iter().enumerate()
        .map(|(i, &x)|
            match (digits.len() - i) % 2 {
                // even position from the right
                0 if x * 2 > 9 => x * 2 - 9,
                0 => x * 2,
                // odd position from the right
                _ => x,
            }
        )
        .sum();

    sum % 10 == 0
}