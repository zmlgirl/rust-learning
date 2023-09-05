use std::time::Duration;

use chrono::{DateTime, Utc};
fn main() {
    let query_opts = QueryOpts::default();
    println!("{:?}", query_opts);
}

#[derive(Debug, Default, Clone, Copy)]
pub struct QueryOpts {
    _time_range: TimeRange,
    _unit: Duration, // default 0ns
    _limit: Limit,
    _select_from_stable: bool,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd)]
pub struct TimeRange {
    start: Option<DateTime<Utc>>,
    end: Option<DateTime<Utc>>,
}

#[derive(Debug, Default, Clone, Copy)]
struct Limit {
    _limit: u32, // default 0
    _offset: Option<u32>,
}
