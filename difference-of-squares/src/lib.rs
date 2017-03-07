pub fn square_of_sum(n: i64) -> i64 {
    let sum = n * (n + 1) / 2;
    sum * sum
}

pub fn sum_of_squares(n: i64) -> i64 {
    (n + 1) * (2 * n + 1) * n / 6
}

pub fn difference(n: i64) -> i64 {
    square_of_sum(n) - sum_of_squares(n)
}
