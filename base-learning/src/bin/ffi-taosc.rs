use std::os::raw::c_int;
use std::os::raw::c_double;

fn main() {

}

// lib c 的函数
extern "C" {
  fn abs(num: c_int) -> c_int;
  fn sqrt(num: c_double) -> c_double;
  fn pow(num: c_double, power: c_double) -> c_double;  
}

#[test]
fn test_call_libc() {
    let x: i32 = -123;
    println!("\n{x}的绝对值是: {}.", unsafe { abs(x) });
    let n: f64 = 9.0;
    let p: f64 = 3.0;
    println!("\n{n}的{p}次方是: {}.", unsafe { pow(n, p) });
    let mut y: f64 = 64.0;
    println!("\n{y}的平方根是: {}.", unsafe { sqrt(y) });
    y = -3.14;
    println!("\n{y}的平方根是: {}.", unsafe { sqrt(y) }); //** NaN = NotaNumber（不是数字） 
}

// taosc 的函数调用
extern "C" {

}

#[test]
fn test_ffi_taosc() {

}