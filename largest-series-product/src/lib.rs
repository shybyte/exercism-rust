pub fn lsp(input: &str, span_len: usize) -> Result<u32, (&'static str)> {
    if span_len == 0 {
        return Ok(1);
    }

    let digits: Vec<u32> = input.chars()
        .filter_map(|c| c.to_digit(10))
        .collect();

    if digits.len() != input.len() {
        return Err("Invalid input: Input should contain only digits.");
    }

    digits.windows(span_len)
        .map(|window| window.iter().product())
        .max()
        .ok_or("String is shorter than span_len.")
}
