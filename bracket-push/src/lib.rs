use std::collections::HashMap;
use std::collections::HashSet;

pub struct Brackets {
    brackets: String,
}

impl Brackets {
    pub fn from(brackets: &str) -> Self {
        Brackets { brackets: brackets.to_string() }
    }

    pub fn are_balanced(&self) -> bool {
        let bracket_pairs = vec![('[', ']'), ('{', '}'), ('(', ')')];
        let open_to_close: HashMap<_, _> = bracket_pairs.into_iter().collect();
        let closing_brackets: HashSet<&char> = open_to_close.values().collect();
        let mut stack: Vec<char> = Vec::new(); // contains the expected closing brackets
        for c in self.brackets.chars() {
            if closing_brackets.contains(&c) && stack.pop() != Some(c) {
                return false;
            } else if let Some(closing) = open_to_close.get(&c) {
                stack.push(*closing);
            }
        }
        stack.is_empty()
    }
}
