pub fn is_armstrong_number(x: u32) -> bool {
    let number_of_digits = (x as f64).log10() as u32 + 1;
    let mut sum = 0;
    let mut temp = x;
    while temp > 0 {
        sum += (temp % 10).pow(number_of_digits);
        temp /= 10;
    }
    x == sum
}

