use std::io;

fn main() {
    // 標準入力からデータを読み込む
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let parts: Vec<usize> = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse().expect("Not a number"))
        .collect();

    let n = parts[0];
    let y = parts[1];

    for x in 0..=n {
        for yy in 0..=(n - x) {
            let z = n - x - yy;
            if 10000 * x + 5000 * yy + 1000 * z == y {
                println!("{} {} {}", x, yy, z);
                return;
            }
        }
    }

    println!("-1 -1 -1");
}
