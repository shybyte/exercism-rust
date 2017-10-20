// return Ok(x) where x is the number of steps required to reach 1
pub fn collatz(n: u64) -> Result<u64, &'static str> {
    if n <= 0 {
        return Err("n must be bigger than 0");
    }

    let mut x = n;
    let mut count = 0;

    while x != 1 {
        x = match x % 2 {
            0 => x / 2,
            _ => x * 3 + 1
        };
        count += 1;
    }

    return Ok(count);
}
