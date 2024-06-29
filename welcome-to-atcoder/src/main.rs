use std::io::{self, Read};

fn main() {
    // 標準入力から値を読み取ります
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let a: i32 = input.trim().parse().expect("Invalid input");

    input.clear();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let mut iter = input.split_whitespace();
    let b: i32 = iter.next().unwrap().parse().expect("Invalid input");
    let c: i32 = iter.next().unwrap().parse().expect("Invalid input");

    input.clear();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let s = input.trim();

    println!("{} {}", a + b + c, s);
}
