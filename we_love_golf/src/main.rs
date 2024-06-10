use std::io;

fn main() {
    let mut input = String::new();

    // Kの入力
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let k: i32 = input.trim().parse().expect("Failed to parse K");

    // AとBの入力
    input.clear();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let range: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse().expect("Failed to parse range"))
        .collect();

    let a = range[0];
    let b = range[1];

    let mut result = "NG";
    for i in a..=b {
        if i % k == 0 {
            result = "OK";
            break;
        }
    }

    println!("{}", result);
}
