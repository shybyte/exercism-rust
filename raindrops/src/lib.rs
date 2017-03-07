pub fn raindrops(x: i32) -> String {
    let factor_sound_pairs = vec![(3, "Pling"), (5, "Plang"), (7, "Plong")];

    let sounds = factor_sound_pairs.into_iter()
        .filter(|&(factor, _)| x % factor == 0)
        .map(|(_, factor_sound)| factor_sound.to_string())
        .collect::<Vec<String>>()
        .concat();

    if sounds.is_empty() {
        x.to_string()
    } else {
        sounds
    }
}
