extern crate rand;
use rand::{thread_rng, sample};

pub struct Robot {
    name: String
}

impl Robot {
    pub fn new() -> Robot {
        Robot {name: generate_name()}
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn reset_name(&mut self) -> () {
        self.name = generate_name();
    }

}

// May generate sometimes the same name like before,
// but how can we prevent this without a global mut variable?
fn generate_name() -> String {
    let mut rng = thread_rng();
    let letters = sample(&mut rng, b'A'..b'Z', 2);
    let numbers = sample(&mut rng, b'1'..b'9', 3);
    String::from_utf8(letters).unwrap() + &String::from_utf8(numbers).unwrap()
}
