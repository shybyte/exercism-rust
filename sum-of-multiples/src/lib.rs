pub fn sum_of_multiples(max: i32, numbers: &Vec<i32>) -> i32 {
    let s = "123";
    (1..max)
        .filter(|x| numbers.iter().any(|d| x % d == 0))
        .sum()
}
