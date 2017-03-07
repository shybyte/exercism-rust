use std::collections::HashMap;
use std::collections::HashSet;

type Mapping = HashMap<char, u8>;

// words are reversed
type Word = Vec<char>;

#[derive(Debug)]
struct Puzzle {
    // ordered in order of addition evaluation
    unique_chars: Vec<char>,
    leading_chars: HashSet<char>,
    summands: Vec<Word>,
    sum: Word,
    max_len: usize
}

impl Puzzle {
    fn from(puzzle: &str) -> Puzzle {
        let mut words: Vec<Word> = puzzle
            .split(|c: char| !c.is_alphabetic())
            .filter(|s| !s.is_empty())
            .map(|word_str| {
                let mut word: Word = word_str.chars().collect();
                word.reverse();
                word
            }).collect();

        let max_len = words.iter().map(|w| w.len()).max().unwrap();
        let mut unique_chars: Vec<char> = Vec::new();
        for col in 0..max_len {
            for word in &words {
                if let Some(c) = word.get(col) {
                    if !unique_chars.contains(c) {
                        unique_chars.push(c.clone());
                    }
                }
            }
        }

        let leading_chars: HashSet<char> = words.iter().map(|w| w.get(w.len() - 1).unwrap().clone()).collect();

        let sum = words.pop().unwrap();

        Puzzle {
            unique_chars: unique_chars,
            summands: words,
            leading_chars: leading_chars,
            sum: sum,
            max_len: max_len
        }
    }

    fn is_sum_valid(&self, mapping: &Mapping) -> bool {
        let sum: u32 = self.summands.iter().map(|s| Puzzle::to_number(s, mapping)).sum();
        let expected_sum = Puzzle::to_number(&self.sum, mapping);
        sum == expected_sum
    }

    fn is_partial_sum_invalid(&self, mapping: &Mapping) -> bool {
        let mut col_sum: u32 = 0;
        for col in 0..self.max_len {
            for word in &self.summands {
                if let Some(c) = word.get(col) {
                    if let Some(&digit) = mapping.get(c) {
                        col_sum += digit as u32;
                    } else {
                        return false;
                    }
                }
            }

            let rem = (col_sum % 10) as u8;

            if let Some(c) = self.sum.get(col) {
                if let Some(&digit) = mapping.get(c) {
                    if digit != rem {
                        return true;
                    }
                } else {
                    return false;
                }
            } else {
                return true
            }

            col_sum = col_sum / 10;
        }
        false
    }

    fn to_number(word: &Word, mapping: &Mapping) -> u32 {
        word.iter().enumerate().fold(0, |acc, (i, c)| acc + (10 as u32).pow(i as u32) * mapping[c] as u32)
    }

    fn solve(&self) -> Option<Mapping> {
        let mut mapping: Mapping = HashMap::new();
        let mut used_numbers: HashSet<u8> = HashSet::new();
        self.solve_internal(&mut used_numbers, &mut mapping)
    }

    fn solve_internal(&self, used_numbers: &mut HashSet<u8>, mapping: &mut Mapping) -> Option<Mapping> {
        if mapping.len() < self.unique_chars.len() {
            let c = self.unique_chars[mapping.len()];
            let begin = if self.leading_chars.contains(&c) { 1 } else { 0 };
            for x in begin..10 {
                if used_numbers.contains(&x) {
                    continue;
                }

                mapping.insert(c, x);
                used_numbers.insert(x);

                if !self.is_partial_sum_invalid(mapping) {
                    let solution = self.solve_internal(used_numbers, mapping);

                    if solution.is_some() {
                        return solution;
                    }
                }

                used_numbers.remove(&x);
                mapping.remove(&c);
            }
        } else if self.is_sum_valid(mapping) {
            return Some(mapping.clone());
        }
        None
    }
}

pub fn solve(puzzle_text: &str) -> Option<Mapping> {
    Puzzle::from(puzzle_text).solve()
}
