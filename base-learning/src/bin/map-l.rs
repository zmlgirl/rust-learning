use std::collections::HashMap;

fn main() {
    let mut record_map: HashMap<String, String> = HashMap::with_capacity(10);
    println!("capacity: {}", record_map.capacity());
    record_map.try_reserve(100).unwrap();
    println!("capacity: {}", record_map.capacity());
    // let s = record_map.get("quality_col").unwrap().clone();
    // println!("{s}");
}
