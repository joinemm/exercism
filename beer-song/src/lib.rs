pub fn verse(n: u32) -> String {
    match n {
        0 => "No more bottles of beer on the wall, no more bottles of beer.
Go to the store and buy some more, 99 bottles of beer on the wall.\n"
            .to_string(),
        1 => "1 bottle of beer on the wall, 1 bottle of beer.
Take it down and pass it around, no more bottles of beer on the wall.\n"
            .to_string(),
        2 => "2 bottles of beer on the wall, 2 bottles of beer.
Take one down and pass it around, 1 bottle of beer on the wall.\n"
            .to_string(),
        _ => format!(
            "{0} bottles of beer on the wall, {0} bottles of beer.
Take one down and pass it around, {1} bottles of beer on the wall.\n",
            n,
            n - 1
        ),
    }
}

pub fn sing(start: u32, end: u32) -> String {
    let mut verses = Vec::new();
    for i in (end..=start).rev() {
        verses.push(verse(i));
    }
    verses.join("\n")
}
