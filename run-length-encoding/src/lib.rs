use std::cmp;

pub fn encode(input: &str) -> String {
    let mut result = String::new();

    let mut chars = input.chars();

    if let Some(first_char) = chars.next() {
        let mut c1 = first_char;
        let mut count = 1;

        for c2 in chars {
            if c1 == c2 {
                count += 1;
            } else {
                append_encoded_char_chain(count, c1, &mut result);
                count = 1;
                c1 = c2;
            }
        }

        append_encoded_char_chain(count, c1, &mut result);
    }

    result
}


pub fn append_encoded_char_chain(count: usize, c: char, output: &mut String) {
    if count > 1 {
        output.push_str(&count.to_string());
    }
    output.push(c);
}


pub fn decode(input: &str) -> String {
    let mut result = String::new();
    let mut count = 0;

    for c in input.chars() {
        if let Some(digit) = c.to_digit(10) {
            count = count * 10 + digit
        } else {
            for _ in 0..cmp::max(count, 1) {
                result.push(c);
            }
            count = 0;
        }
    }

    result
}