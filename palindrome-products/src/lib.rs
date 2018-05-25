pub type Palindrome = u64;

pub fn get_palindrome_products(min: u64, max: u64) -> Vec<Palindrome> {
    let mut digits = vec![];
    let products = (min..=max).flat_map(|f1| (f1..=max).map(move |f2| f1 * f2));
    products.filter(|&product| {
        to_digits(product, &mut digits);
        is_palindrome(&digits)
    }).collect()
}

pub fn min(palindromes: &[Palindrome]) -> Option<Palindrome> {
    palindromes.iter().min().cloned()
}

pub fn max(palindromes: &[Palindrome]) -> Option<Palindrome> {
    palindromes.iter().max().cloned()
}

fn is_palindrome(digits: &[u64]) -> bool {
    digits.iter().zip(digits.iter().rev()).all(|(digit, mirror)| digit == mirror)
}

fn to_digits(x: u64, digits: &mut Vec<u64>) {
    digits.clear();
    let mut temp = x;
    while temp > 0 {
        digits.push(temp % 10);
        temp /= 10;
    }
}
