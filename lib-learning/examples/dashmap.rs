use std::{sync::Arc, time::Duration};

use dashmap::DashMap;
use tokio::sync::Mutex;

#[tokio::main]
async fn main() {
    let map = DashMap::new();
    map.insert("Jack", "Goalie");
    let v = map.get("Jack").unwrap();
    // 如果不 drop 掉，那么 remove 则会死锁，或者并不能称之为死锁
    drop(v);
    map.remove("Jack");

    // 多线程插入可能死锁??
    let mut handles = Vec::new();
    let map = Arc::new(DashMap::new());
    let mutex: Arc<Mutex<()>> = Arc::new(Mutex::new(()));
    for i in 0..40 {
        let handle = tokio::spawn(insert_index(map.clone(), i, mutex.clone()));
        handles.push(handle);
    }

    for handle in handles {
        let _ = handle.await;
        println!("worker done");
    }
}

async fn insert_index(map: Arc<DashMap<i32, Mutex<i32>>>, key: i32, mutex: Arc<Mutex<()>>) {
    let mut j = 0;
    loop {
        if j > 10 {
            break;
        }
        if !map.contains_key(&key) {
            let guard = mutex.lock();
            // let mut rng = rand::thread_rng();
            // std::thread::sleep(Duration::from_secs(rng.gen_range(0..10)));
            println!("insert {key}");
            map.insert(key, Mutex::new(key));
            println!("insert {key} done");
            drop(guard);
        }

        // println!("get {key}");
        let v = map.get(&key).expect("should have value for {key}");
        // println!("get {key} lock done");
        let _ = v.value().lock();
        std::thread::sleep(Duration::from_secs(2));
        j += 1;
    }
}
