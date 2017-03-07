
fn n_bottles(n: i32) -> String {
    match n {
        0 => "no more bottles".to_string(),
        1 => "1 bottle".to_string(),
        _ => format!("{} bottles", n),
    }
}

pub fn verse(n: i32) -> String {
    if n > 0 {
        format!("{n_bottles} of beer on the wall, {n_bottles} of beer.\nTake {one_or_it} down \
                 and pass it around, {next_n_bottles} of beer on the wall.\n",
                n_bottles = n_bottles(n),
                one_or_it = if n == 1 { "it" } else { "one" },
                next_n_bottles = n_bottles(n - 1))
    } else {
        "No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and \
         buy some more, 99 bottles of beer on the wall.\n"
            .to_string()
    }
}

pub fn sing(max: i32, min: i32) -> String {
    (min..(max + 1))
        .rev()
        .map(verse)
        .collect::<Vec<String>>()
        .join("\n")
}
