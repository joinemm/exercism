pub fn reply(mut message: &str) -> &str {
    message = message.trim();
    if message.is_empty() {
        "Fine. Be that way!"
    } else if message.chars().any(char::is_alphabetic) && message.to_uppercase() == message {
        if message.ends_with('?') {
            "Calm down, I know what I'm doing!"
        } else {
            "Whoa, chill out!"
        }
    } else if message.ends_with('?') {
        "Sure."
    } else {
        "Whatever."
    }
}
