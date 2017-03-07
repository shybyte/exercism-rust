pub fn number(input: &str) -> Option<String> {
    let digits: String = input.chars().filter(|&c| c.is_digit(10)).collect();
    match digits.len() {
        10 => Some(digits),
        11 if digits.starts_with("1") => Some(digits[1..].to_string()),
        _ => None,
    }
}

pub fn area_code(input: &str) -> Option<String> {
    number(input).map(|digits| parts(&digits).0.to_string())
}

fn parts(digits: &str) -> (&str, &str, &str) {
    let (area_code, remaining) = digits.split_at(3);
    let (part2, part3) = remaining.split_at(3);
    (area_code, part2, part3)
}

pub fn pretty_print(input: &str) -> String {
    match number(input) {
        Some(digits) => {
            let (area_code, part2, part3) = parts(&digits);
            format!("({}) {}-{}", area_code, part2, part3)
        }
        None => "invalid".to_string(),
    }
}
