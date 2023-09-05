use regex::Regex;

pub const BRANCH: &str = r#"enh/add-commitid-in-version"#;

fn main() {
    let branch: &str = "123";
    let str1 = String::from("123");

    // string, &str, eq
    println!("eq: {}", str1 == branch); // 一个 String 可以和 &str 使用 == 进行比较
                                        // println!("eq: {}", st.eq(branch.to_string())); // 但是不能和 String 进行比较
    println!("eq: {}", str1 == branch.to_string()); // 一个 String 可以和另外一个 String 使用 == 进行比较
    println!("eq: {}", BRANCH == "main");
    println!("eq: {}", str1 == "123");

    let from_dsn = "mqtt://128.0.0.1?version=3.0&clean_session=true&ca=----BEGIN CERTIFICATE---
    MIIB2zCCAUQCCQCsC7PtwOpUNDANBgkqhkiG9w0BAQUFADAyMTAwLgYDVQQKDCdU
    TFMgUHJvamVjdCBEb2RneSBDZXJ0aWZpY2F0ZSBBdXRob3JpdHkwHhcNMTkwNzI2
    MDYyMzI0WhcNMzMwNDAzMDYyMzI0WjAyMTAwLgYDVQQKDCdUTFMgUHJvamVjdCBE
    b2RneSBDZXJ0aWZpY2F0ZSBBdXRob3JpdHkwgZ8wDQYJKoZIhvcNAQEBBQADgY0A
    MIGJAoGBAMt3CLNHbse7tNslbqD+r7V5jVzJPe2KHppRcSHs3KIOihq2o4pKV+eV
    JNQDc4YF5OnmsSeArJ6pkmVZbf5S78YcXB+nAo7q0ys/gG4AzDnVc+1KxT9O1h0j
    ZZFBaW6XIgoVbslAMKSiYq7JgDsee4hZFtPHwqPajP/2EwxIIcaFAgMBAAEwDQYJ
    KoZIhvcNAQEFBQADgYEAjbXg4JxYHNgS3mnOJt+4sxoTiQsucyuidmMACX84Ij2r
    SsPQ8YvrTj00pdRQnu7wtZfOyOE5Mu6icwLpDu1epvmskIhbRDD79hPEu/92NmYd
    mgrqr8rbWov9SRnlrKkGdfG3/RIkIP1joPjLL2XnVEryoXOs4vdTc1Chcz8c6Bw=
    ---END CERTIFICATE---
    &cert=---BEGIN CERTIFICATE---
    MIIB2zCCAUQCCQCsC7PtwOpUNDANBgkqhkiG9w0BAQUFADAyMTAwLgYDVQQKDCdU
    TFMgUHJvamVjdCBEb2RneSBDZXJ0aWZpY2F0ZSBBdXRob3JpdHkwHhcNMTkwNzI2
    MDYyMzI0WhcNMzMwNDAzMDYyMzI0WjAyMTAwLgYDVQQKDCdUTFMgUHJvamVjdCBE
    b2RneSBDZXJ0aWZpY2F0ZSBBdXR...";

    let ref re = Regex::new(
        r"(?x)
    (?P<driver>[\w.-]+)(\+(?P<protocol>[^@/?\#]+))?: # abc
    (
        # url-like dsn
        //((?P<username>[\w\-_%.]+)?(:(?P<password>[^@/?\#]+))?@)? # for authorization
            (((?P<protocol2>[\w.-]+)\()?
                (?P<addr>[\w\-_%.:]*(:\d{0,5})?(,[\w\-:_.]*(:\d{0,5})?)*)?  # for addresses
            \)?)?
            (/(?P<subject>[\w %$@./-]+)?)?                             # for subject
        | # or
        # path-like dsn
        (?P<path>([\\/.~]$|/\w+[\w %$@:.\\\-/]*|[\.~\w]?[\w %$@:.\\\-/]+))
    ) # abc
    (\?(?P<params>(?s:.)*))?",
    )
    .unwrap();
    // (\?(?P<params>[\d\D]*))?").unwrap();
    // (\?(?P<params>[\s\S]*))?").unwrap();
    // String append
    let cap = re.captures(from_dsn).unwrap();
    if let Some(p) = cap.name("params") {
        println!("p:{}", p.as_str());
        for p in p.as_str().split_terminator('&') {
            if p.contains('=') {
                if let Some((k, v)) = p.split_once('=') {
                    println!("{k}, {v}");
                }
            } else {
                println!("p:{p}");
            }
        }
    }

    let mut s = "123".to_string();
    s.pop();
    print!("{s}");

    let res = "{\\\\\\\"stable_prefix\\\":\\\"meters\\\",\\\"column_configs\\\":[{\\\"column_name\\\":\\\"received_time\\\",\\\"column_type\\\":9,\\\"column_latest_alias\\\":null,\\\"column_alias\\\":\\\"ts\\\",\\\"is_primary_key\\\":true},{\\\"column_name\\\":\\\"original_time\\\",\\\"column_type\\\":9,\\\"column_latest_alias\\\":null,\\\"column_alias\\\":null,\\\"is_primary_key\\\":false},{\\\"column_name\\\":\\\"value\\\",\\\"column_type\\\":null,\\\"column_latest_alias\\\":null,\\\"column_alias\\\":\\\"valueaa\\\",\\\"is_primary_key\\\":false}]}";
    println!("{res}");
    let res = res.replace("\\", "");
    println!("{res}");
}

/// Rust 中 String 的长度
#[test]
fn test_str_len() {
    // 1M = 1024 * 1024B
    let sql = "insert";
    println!("len1: {}", sql.len());
    let sql = "insert";
    println!("len2: {}", sql.to_string().len());
    let sql = "中文".to_string();
    println!("len3: {}", sql.len()); // 看起来一个中文字符在 Rust 里面需要三个字节存储
    let c = 'c';
    println!("len4: {}", c.len_utf8());
}

#[test]
fn test_string_regex() {
    let table1 = "tb_JHW72-11_tb_123";
    let re = Regex::new(r"^tb_").unwrap();
    println!("is match: {}", re.is_match(table1));
    // let capture = re.captures(table1).unwrap();
    // let suffix = capture.name("suffix").unwrap().as_str();
    // println!("suffix: {suffix}");
    // let expression = "fx_suffix";
    // let res = expression.replace("suffix", suffix);
    // println!("res: {res}");

    println!("res:{} ", re.replace_all("tb_JHW72-11_tb_123", "fr_").to_string());
    println!("res:{} ", re.replace_all("xx_JHW72-11_tb_123", "fr_"));

    let re = Regex::new("(?<old>.*)").unwrap();
    println!("res:{} ", re.replace_all("tb_JHW72-11_tb_123", "fr_$old").to_string());
    let re = Regex::new("tb(?<old>)").unwrap();
    println!("res:{} ", re.replace_all("tbd100", "fr_$old").to_string());
    // 如何过滤掉一些前面匹配的
    let re = Regex::new("d\\d+(?!\\d)").unwrap();
    println!("res:{} ", re.replace_all("d10001", "fr_$old").to_string());
    println!("res:{} ", re.replace_all("d42", "fr_$old").to_string());
    println!("res:{} ", re.replace_all("d45", "fr_$old").to_string());
}

