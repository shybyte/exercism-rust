pub fn factors(n_arg: u64) -> Vec<u64> {
    let mut result = vec![];
    let mut i = 2;
    let mut n = n_arg;

    while i <= n {
        if n % i == 0 {
            n = n / i;
            result.push(i);
        } else {
            i += 1;
        }
    }

    result
}
