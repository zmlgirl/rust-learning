use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();
    let str = Some(String::from("d1001"));
    map.insert(str, 1);
    let str2 = None;
    map.insert(str2, 2);
    dbg!(map);
}