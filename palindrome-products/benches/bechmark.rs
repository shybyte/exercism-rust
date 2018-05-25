#![feature(test)]

// rustup run nightly cargo bench

extern crate test;
extern crate palindrome_products;

use test::Bencher;
use palindrome_products::*;

#[bench]
fn bench_triple_digits(b: &mut Bencher) {
    b.iter(|| {
        let palindromes = get_palindrome_products(100, 999);
        assert_eq!(min(&palindromes), Some(10201));
        assert_eq!(max(&palindromes), Some(906609));
    });
}

// with format string: 78,903,815 ns/iter
// with digit allocations: 16,330,280 ns/iter
// without allocations: 7,034,146 ns/iter
// with flat_map: 7,623,171 ns/iter