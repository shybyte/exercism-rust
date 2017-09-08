pub fn find() -> Option<i64> {
    for a in 1..1000 {
        for b in 1..1000 - a {
            let c = 1000 - a - b;
            if a * a + b * b == c * c {
                return Some(a * b * c);
            }
        }
    }
    None
}