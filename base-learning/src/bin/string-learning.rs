
pub const BRANCH :&str = r#"enh/add-commitid-in-version"#;
fn main() {
    let branch: &str = "123";
    let str1 = String::from("123");

    println!("eq: {}", str1 == branch); // 一个 String 可以和 &str 使用 == 进行比较
    // println!("eq: {}", st.eq(branch.to_string())); // 但是不能和 String 进行比较
    println!("eq: {}", str1 == branch.to_string()); // 一个 String 可以和另外一个 String 使用 == 进行比较
    println!("eq: {}", BRANCH == "main");
}