use std::collections::HashMap;

fn main() {
    let text = "hello wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        // entry api = .entry(k).or_insert(default)
        let count = map.entry(word).or_insert(0);
        // dereference operator to modify hash contents
        *count += 1;
    }
    println!("{:?}", map);
}
