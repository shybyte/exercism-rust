const DIGIT_PATTERN_SIZE: usize = 3;
const INPUT_ROW_HEIGHT: usize = DIGIT_PATTERN_SIZE + 1;

static DIGITS_PATTERNS: [[&'static str; DIGIT_PATTERN_SIZE]; 10] = [
    [
        " _ ",
        "| |",
        "|_|"
    ], [
        "   ",
        "  |",
        "  |"
    ], [
        " _ ",
        " _|",
        "|_ "
    ], [
        " _ ",
        " _|",
        " _|"
    ], [
        "   ",
        "|_|",
        "  |"
    ], [
        " _ ",
        "|_ ",
        " _|"
    ], [
        " _ ",
        "|_ ",
        "|_|"
    ], [
        " _ ",
        "  |",
        "  |"
    ], [
        " _ ",
        "|_|",
        "|_|"
    ], [
        " _ ",
        "|_|",
        " _|"
    ]
];


pub fn convert(input: &str) -> Result<String, ()> {
    let lines: Vec<&str> = input.lines().collect();

    if lines.len() % INPUT_ROW_HEIGHT != 0
        || lines.iter().any(|line| line.len() % DIGIT_PATTERN_SIZE != 0) {
        return Err(());
    }

    Ok(lines.chunks(INPUT_ROW_HEIGHT).map(convert_row).collect::<Vec<_>>().join(","))
}


pub fn convert_row(lines: &[&str]) -> String {
    let number_of_digits = lines[0].len() / DIGIT_PATTERN_SIZE;

    (0..number_of_digits)
        .map(|i|
            lines[0..DIGIT_PATTERN_SIZE].iter()
                .map(|line| &line[i * DIGIT_PATTERN_SIZE..(i + 1) * DIGIT_PATTERN_SIZE])
                .collect::<Vec<&str>>()
        )
        .map(|d| lookup_digit(&d[0..DIGIT_PATTERN_SIZE]))
        .collect()
}

pub fn lookup_digit(pattern: &[&str]) -> String {
    match DIGITS_PATTERNS.iter().position(|&x| x == pattern) {
        Some(pos) => pos.to_string(),
        None => "?".to_string()
    }
}
