use std::{thread::sleep, time::{Instant, Duration}};

fn main() {
    let insert_start = Instant::now();
    sleep(Duration::from_secs(3 as _));
    println!("{}", insert_start.elapsed().as_millis());
}