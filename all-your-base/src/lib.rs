pub fn convert(input: &[u32], from_base: u32, to_base: u32) -> Result<Vec<u32>, ()> {
    if from_base < 2 || to_base < 2 || input.iter().any(|&x| x >= from_base) {
        return Err(());
    }
    let mut number = input.iter().fold(0, |acc, x| acc * from_base + x);
    let mut result: Vec<u32> = Vec::new();
    while number > 0 {
        result.push(number % to_base);
        number = number / to_base;
    }
    result.reverse();
    Ok(result)
}
