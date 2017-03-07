pub fn is_leap_year(year: i32) -> bool {
    let is_divisible = |d: i32| year % d == 0;
    (is_divisible(4) && !is_divisible(100)) || is_divisible(400)
}
