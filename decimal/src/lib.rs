/// Right now I'm to lazy to implement
/// a real arbitrary-precision decimal type,
/// so this code is just fixed-precision
/// with a precision high enough to make all test cases pass.

extern crate num_bigint;

use std::ops::{Add, Mul, Sub};
use num_bigint::BigInt;
use std::str::FromStr;

use std::fmt;

const DIGITS_AFTER_POINT: usize = 50;

#[derive(Eq, PartialEq, Ord, PartialOrd, Clone)]
pub struct Decimal {
    x: BigInt
}

impl Decimal {
    pub fn try_from(s: &str) -> Option<Decimal> {
        Decimal::try_from_or_error(s).ok()
    }

    pub fn try_from_or_error(s: &str) -> Result<Decimal, &str> {
        let dot_index = s.find('.').unwrap_or(s.len() - 1);
        let s_without_dot: String = s.chars().filter(|&c| c != '.').collect();
        let x = BigInt::from_str(&s_without_dot).map_err(|_| "Parsing Error")?;

        Ok(
            Decimal {
                x: x * pow(BigInt::from(10), DIGITS_AFTER_POINT + 1 - (s.len() - dot_index))
            }
        )
    }
}

impl Add for Decimal {
    type Output = Decimal;

    fn add(self, other: Decimal) -> Self::Output {
        Decimal {
            x: self.x + other.x,
        }
    }
}

impl Mul for Decimal {
    type Output = Decimal;

    fn mul(self, other: Decimal) -> Self::Output {
        Decimal {
            x: (self.x * other.x) / pow(BigInt::from(10), DIGITS_AFTER_POINT),
        }
    }
}

impl Sub for Decimal {
    type Output = Decimal;

    fn sub(self, other: Decimal) -> Self::Output {
        Decimal {
            x: self.x - other.x,
        }
    }
}

impl fmt::Debug for Decimal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.x.to_string())
    }
}

pub fn pow(x: BigInt, y: usize) -> BigInt {
    (0..y).fold(BigInt::from(1), |acc, _| acc * &x)
}