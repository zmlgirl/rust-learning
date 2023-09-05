use parse_duration::parse;

fn main() {
    let dura = parse("1d").unwrap();
    println!("{:?}", dura);
    let dura = parse("1M").unwrap();
    println!("{:?}", dura);
    let dura = parse("10ms").unwrap();
    println!("{:?}", dura);
    let dura = parse("10").unwrap();
    println!("{:?}", dura);
    let dura = parse("120").unwrap();
    println!("{:?}", dura);
}

#[test]
fn test_parse() {
    use std::time::Duration;
    // One hour less than a day
    assert_eq!(parse("1 day -1 hour"), Ok(Duration::new(82_800, 0)));
    // Using exponents
    assert_eq!(
        parse("1.26e-1 days"),
        Ok(Duration::new(10_886, 400_000_000))
    );
    // Extra things will be ignored
    assert_eq!(
        parse("Duration: 1 hour, 15 minutes and 29 seconds"),
        Ok(Duration::new(4529, 0))
    );
}
