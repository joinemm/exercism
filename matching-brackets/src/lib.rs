const OPENING: [char; 3] = ['(', '{', '['];
const CLOSING: [char; 3] = [')', '}', ']'];

pub fn brackets_are_balanced(string: &str) -> bool {
    let mut waiting_for = vec![];

    for c in string.chars() {
        if OPENING.contains(&c) {
            waiting_for.push(get_closing_bracket(c));
        } else if CLOSING.contains(&c) {
            if !waiting_for.is_empty() && waiting_for.last().unwrap() == &c {
                waiting_for.pop();
            } else {
                return false;
            }
        }
    }

    waiting_for.is_empty()
}

fn get_closing_bracket(c: char) -> char {
    CLOSING[OPENING.iter().position(|x| x == &c).unwrap()]
}
