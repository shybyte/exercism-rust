use std::ops::{Add, Div, Mul, Sub};

pub struct WordProblem {
    command: String
}

static IGNORED_WORDS: [&'static str; 3] = ["What", "is", "by"];

impl WordProblem {
    pub fn new(command: &str) -> WordProblem {
        WordProblem { command: command.into() }
    }

    pub fn answer(&self) -> Result<i64, String> {
        let command = self.command.replace("?", "");
        let tokens = command.split_whitespace().filter(|word| !IGNORED_WORDS.contains(word));

        let mut result = 0;
        let mut operation: fn(i64, i64) -> i64 = i64::add;

        for token in tokens {
            match token.parse() {
                Ok(number) => {
                    result = operation(result, number);
                },
                Err(_) => {
                    operation = match token {
                        "plus" => i64::add,
                        "minus" => i64::sub,
                        "multiplied" => i64::mul,
                        "divided" => i64::div,
                        _ => return Err(format!("Expected operation but found {}", token))
                    }
                }
            }
        }

        Ok(result)
    }
}