pub fn reply(message: &str) -> &str {
    if message == "" {
        "Fine. Be that way!"
    } else if message.to_uppercase() == message {
        "Whoa, chill out!"
    } else if message.ends_with("?") {
        "Sure."
    } else {
        "Whatever."
    }
}
