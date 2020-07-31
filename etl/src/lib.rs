use std::collections::BTreeMap;

pub fn transform(h: &BTreeMap<i32, Vec<char>>) -> BTreeMap<char, i32> {
    let mut new_tree: BTreeMap<char, i32> = BTreeMap::new();
    for (score, chars) in h {
        for c in chars {
            new_tree.insert(
                *c.to_lowercase().collect::<Vec<char>>().last().unwrap(),
                *score,
            );
        }
    }
    new_tree
}
