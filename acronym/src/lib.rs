pub fn abbreviate(phrase: &str) -> String {
    let mut acronym = String::new();
    let phrase = &phrase.replace('-', " ");
    for word in phrase.split_whitespace() {
        let mut n = 0;
        let mut last_was_upper = false;
        while let Some(c) = word.chars().nth(n) {
            if c.is_alphabetic() {
                if c.is_uppercase() || n == 0 {
                    if !last_was_upper {
                        acronym.push(c);
                    }
                    last_was_upper = true;
                } else {
                    last_was_upper = false;
                }
            }
            n += 1;
        }
    }
    acronym.to_uppercase()
}
