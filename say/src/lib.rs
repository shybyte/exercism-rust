static BELOW_14: [&'static str; 14] = ["zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "ten", "eleven", "twelve", "thirteen"];
static BELOW_100: [&'static str; 8] = ["twenty", "thirty", "forty", "fifty", "sixty", "seventy", "eighty", "ninety"];
static CHUNK_WORDS: [&'static str; 7] = ["", "thousand", "million", "billion", "trillion", "quadrillion", "quintillion"];

/* 123456789 => [789, 456, 123] */
pub fn to_reversed_chunks(x_arg: u64) -> Vec<u64> {
    let mut result = vec![];
    let mut x = x_arg;
    while x > 0 {
        result.push(x % 1000);
        x = x / 1000;
    }
    result
}

pub fn encode(x_arg: u64) -> String {
    let x = x_arg as usize;
    if x < 14 {
        BELOW_14[x].to_string()
    } else if x < 20 {
        format!("{}teen", BELOW_14[x - 10])
    } else if x < 100 {
        format!("{}-{}", BELOW_100[x / 10 - 2], BELOW_14[x % 10])
    } else if x < 1000 {
        let rem100_text = if x % 100 > 0 { format!(" {}", encode(x_arg % 100)) } else { "".to_string() };
        format!("{} hundred{}", BELOW_14[x / 100], rem100_text)
    } else {
        let text_chunks: Vec<String> = to_reversed_chunks(x as u64).iter().enumerate().rev()
            .filter(|&(_, &chunk_value)| chunk_value > 0)
            .map(|(chunk_i, &chunk_value)| {
                if chunk_i > 0 {
                    format!("{} {}", encode(chunk_value), CHUNK_WORDS[chunk_i])
                } else {
                    encode(chunk_value)
                }
            }).collect();
        text_chunks.join(" ")
    }
}