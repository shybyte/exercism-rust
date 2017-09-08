pub fn nth(n: usize) -> Result<u32, &'static str> {
    if n == 0 {
        return Err("N should be larger than 0");
    }

    let mut primes = Vec::with_capacity(n);
    let mut current_candidate = 2;
    loop {
        if primes.iter().all(|prime| current_candidate % prime != 0) {
            primes.push(current_candidate);
            if primes.len() == n {
                return Ok(current_candidate);
            }
        }
        current_candidate += 1;
    }
}