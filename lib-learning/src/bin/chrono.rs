use chrono::Utc;

fn main() {

    let timestamp = Utc::now().timestamp_millis();
    println!("{timestamp}");
}
