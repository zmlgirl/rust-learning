use anyhow::Context;
use itertools::Itertools;

fn main() {
    let version = "3.0.5.0".to_string();
    // let split = version.split('.').dropping_back(1);
    let mut version_vec = version.splitn(4, '.').collect_vec();
    version_vec.truncate(3);
    dbg!(&version_vec);

    let (a, b, c) = version_vec
        .into_iter()
        // .split('.').dropping_back(1)
        .map(|x| x.parse::<i32>().unwrap())
        .collect_tuple()
        .unwrap();
    println!("a: {}, b: {}, c: {}", a, b, c);
}

fn simple_err() -> Result<String, String> {
    Err(format!("simple error"))
}

fn anyhow_err() -> anyhow::Result<()> {
    anyhow::bail!("anyhow error")
}

// 测试在使用 with_context 的情况下使用 {:#} 能够打印出完整的 with_context 和 root_cause
#[test]
fn test_error_print() {
    fn error_with_context() -> anyhow::Result<()> {
        anyhow::bail!("root cause bulalala");
    }

    let error = error_with_context().with_context(|| format!("err with context"));
    match error {
        Ok(_) => (),
        Err(err) => {
            println!("{err:#}")
        }
    }
}
