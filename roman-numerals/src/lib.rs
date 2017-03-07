pub struct Roman {
    roman: String,
}

impl Roman {
    pub fn from(x: u32) -> Roman {
        let mut roman = String::new();
        let mut remaining_x = x;
        loop {
            let rule = RULES.iter().find(|&&(_, value)| remaining_x >= value);
            match rule {
                Some(&(letters, value)) => {
                    roman.push_str(letters);
                    remaining_x -= value;
                }
                None => break,
            }
        }
        Roman { roman: roman }
    }

    pub fn to_string(&self) -> &str {
        &self.roman
    }
}

static RULES: [(&'static str, u32); 13] = [("M", 1000),
                                           ("CM", 900),
                                           ("D", 500),
                                           ("CD", 400),
                                           ("C", 100),
                                           ("XC", 90),
                                           ("L", 50),
                                           ("XL", 40),
                                           ("X", 10),
                                           ("IX", 9),
                                           ("V", 5),
                                           ("IV", 4),
                                           ("I", 1)];
