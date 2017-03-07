pub fn mark_multiple(sieve: &mut Vec<bool>, x: usize) {
    let mut i = x;
    while i < sieve.len() {
        sieve[i] = true;
        i += x
    }
}

pub fn primes_up_to(max: usize) -> Vec<usize> {
    let mut sieve = vec![false; max+1];
    let mut primes: Vec<usize> = Vec::new();
    for i in 2..(max + 1) {
        if !sieve[i] {
            primes.push(i);
            mark_multiple(&mut sieve, i);
        }
    }
    primes
}
