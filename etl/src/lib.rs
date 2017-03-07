use std::collections::BTreeMap;

pub fn transform(input: &BTreeMap<i32, Vec<String>>) -> BTreeMap<String, i32> {
    let mut result = BTreeMap::new();
    for (&score, letters) in input {
        for letter in letters {
            result.insert(letter.to_lowercase(), score);
        }
    }
    result
}
