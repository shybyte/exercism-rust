use std::ops::{Add, Div, Mul, Sub};
use std::collections::HashMap;


pub type Value = i32;
pub type ForthResult = Result<(), Error>;

#[derive(Debug)]
pub struct Forth {
    stack: Vec<String>,
    state: State,
    words: HashMap<String, Vec<String>>
}

#[derive(Debug, Eq, PartialEq)]
enum State {
    Normal,
    NewWord,
    NewWordDefinition(String, Vec<String>)
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
        Forth { stack: Vec::new(), state: State::Normal, words: HashMap::new() }
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
            let mut next_state: Option<State> = None;
            match self.state {
                State::Normal => {
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
                            if self.state != State::Normal {
                                return Err(Error::InvalidWord)
                            }
                            next_state = Some(State::NewWord)
                        }
                        _ => {
                            if word.parse::<Value>().is_ok() {
                                self.stack.push(word.to_string());
                            } else {
                                return Err(Error::UnknownWord);
                            }
                        }
                    }
                },
                State::NewWord => {
                    match word.as_ref() {
                        ";" | ":" => {
                            return Err(Error::InvalidWord)
                        },
                        name => {
                            if name.parse::<Value>().is_ok() {
                                return Err(Error::InvalidWord);
                            }
                            next_state = Some(State::NewWordDefinition(name.to_string(), Vec::new()));
                        }
                    }
                },
                State::NewWordDefinition(ref new_word, ref mut def) => {
                    match word.as_ref() {
                        ";" => {
                            let mut reversed = def.clone();
                            reversed.reverse();
                            self.words.insert(new_word.to_string(), reversed);
                            next_state = Some(State::Normal);
                        },
                        ":" => return Err(Error::InvalidWord),
                        other => def.push(other.to_string())
                    }
                }
            }

            if let Some(state) = next_state {
                self.state = state;
            }
        }

        match self.state {
            State::Normal => Ok(()),
            _ => Err(Error::InvalidWord)
        }
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
}


pub fn is_whitespace(c: char) -> bool {
    c.is_whitespace() || c == '\u{0000}' || c == '\u{0001}'
}
