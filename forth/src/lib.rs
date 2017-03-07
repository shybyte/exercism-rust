use std::ops::{Add, Div, Mul, Sub};
use std::collections::HashMap;


pub type Value = i32;
pub type ForthResult = Result<(), Error>;

#[derive(Debug)]
pub struct Forth {
    stack: Vec<String>,
    words: HashMap<String, Vec<String>>
}

#[derive(Debug, PartialEq)]
pub enum Error {
    DivisionByZero,
    StackUnderflow,
    UnknownWord,
    InvalidWord,
}

impl Forth {
    pub fn new() -> Forth {
        Forth { stack: Vec::new(), words: HashMap::new() }
    }

    pub fn format_stack(&self) -> String {
        self.stack.join(" ")
    }

    pub fn eval(&mut self, input: &str) -> ForthResult {
        let mut command_stack: Vec<String> = input.to_lowercase().
            split(is_whitespace)
            .rev()
            .map(|s| s.to_string()).collect();

        while let Some(word) = command_stack.pop() {
            if let Some(word_def) = self.words.get(&word) {
                for w in word_def {
                    command_stack.push(w.clone());
                }
                continue;
            }
            match word.as_ref() {
                "+" => self.apply_math_operator(Value::add)?,
                "-" => self.apply_math_operator(Value::sub)?,
                "*" => self.apply_math_operator(Value::mul)?,
                "/" => {
                    if self.stack.len() >= 2 && self.stack.last().unwrap().parse() == Ok(0) {
                        return Err(Error::DivisionByZero)
                    }
                    self.apply_math_operator(Value::div)?
                },
                "dup" => {
                    if self.stack.len() == 0 {
                        return Err(Error::StackUnderflow)
                    }
                    let last = self.stack[self.stack.len() - 1].clone();
                    self.stack.push(last);
                },
                "drop" => {
                    if self.stack.len() == 0 {
                        return Err(Error::StackUnderflow)
                    }
                    self.stack.pop();
                }
                "swap" => {
                    if self.stack.len() < 2 {
                        return Err(Error::StackUnderflow)
                    }
                    let op1 = self.stack.pop().unwrap();
                    let op2 = self.stack.pop().unwrap();
                    self.stack.push(op1);
                    self.stack.push(op2);
                }
                "over" => {
                    if self.stack.len() < 2 {
                        return Err(Error::StackUnderflow)
                    }
                    let clone = self.stack[self.stack.len() - 2].clone();
                    self.stack.push(clone);
                }
                ":" => {
                    self.add_new_word(&mut command_stack)?;
                }
                _ => {
                    if word.parse::<Value>().is_ok() {
                        self.stack.push(word.to_string());
                    } else {
                        return Err(Error::UnknownWord);
                    }
                }
            }
        }

        Ok(())
    }

    fn apply_math_operator(&mut self, operator: fn(Value, Value) -> Value) -> ForthResult {
        if self.stack.len() < 2 {
            return Err(Error::StackUnderflow)
        }
        let op1: Result<Value, _> = self.stack.pop().unwrap().parse();
        let op2: Result<Value, _> = self.stack.pop().unwrap().parse();
        if op1.is_err() || op2.is_err() {
            return Err(Error::StackUnderflow)
        }
        self.stack.push((operator(op2.unwrap(), op1.unwrap())).to_string());
        Ok(())
    }

    fn add_new_word(&mut self, command_stack: &mut Vec<String>) -> ForthResult {
        if let Some(new_word) = command_stack.pop() {
            match new_word.as_ref() {
                ";" | ":" => {
                    return Err(Error::InvalidWord)
                },
                _ => {
                    if new_word.parse::<Value>().is_ok() {
                        return Err(Error::InvalidWord);
                    }
                    let mut new_word_def: Vec<String> = Vec::new();
                    while let Some(word) = command_stack.pop() {
                        match word.as_ref() {
                            ";" => {
                                let mut reversed = new_word_def.clone();
                                reversed.reverse();
                                self.words.insert(new_word.to_string(), reversed);
                                return Ok(());
                            },
                            ":" => return Err(Error::InvalidWord),
                            other => new_word_def.push(other.to_string())
                        }
                    }
                    return Err(Error::InvalidWord);
                }
            }
        } else {
            return Err(Error::InvalidWord);
        }
    }
}

pub fn is_whitespace(c: char) -> bool {
    c.is_whitespace() || c == '\u{0000}' || c == '\u{0001}'
}