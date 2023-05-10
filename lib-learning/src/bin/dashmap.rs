use dashmap::DashMap;

fn main() {
    let map = DashMap::new();
    map.insert("Jack", "Goalie");
    let v = map.get("Jack").unwrap();
    // 如果不 drop 掉，那么 remove 则会死锁
    drop(v);
    map.remove("Jack");
}