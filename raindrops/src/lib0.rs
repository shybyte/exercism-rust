pub fn raindrops(x: i32) -> String {
    let is_divisible = |d: i32| x % d == 0;
    let mut sounds = String::new();
    if is_divisible(3) {
        sounds += "Pling"
    }
    if is_divisible(5) {
        sounds += "Plang"
    }
    if is_divisible(7) {
        sounds += "Plong"
    }
    if sounds.is_empty() {
        return x.to_string();
    }
    sounds
}
