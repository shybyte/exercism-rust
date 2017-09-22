use std::cmp::Ordering::*;

#[derive(Debug, PartialEq)]
pub enum Classification {
    Perfect,
    Abundant,
    Deficient
}

pub fn classify(x: u64) -> Result<Classification, &'static str> {
    if x == 0 {
        return Err("Number must be positive");
    }
    Ok(
        match aliquot_sum(x).cmp(&x) {
            Equal => Classification::Perfect,
            Greater => Classification::Abundant,
            Less => Classification::Deficient
        }
    )
}

fn aliquot_sum(x: u64) -> u64 {
    (1..(x / 2 + 1)).filter(|d| x % d == 0).sum()
}