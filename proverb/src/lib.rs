pub fn build_proverb(list: &[&str]) -> String {
    if list.len() == 0 {
        String::new()
    } else {
        let mut rows = Vec::<String>::new();
        for i in 0..list.len() - 1 {
            rows.push(verse(list[i], list[i + 1]));
        }
        rows.push(format!("And all for the want of a {}.", list[0]));
        rows.join("\n")
    }
}

fn verse(a: &str, b: &str) -> String {
    format!("For want of a {} the {} was lost.", a, b)
}
