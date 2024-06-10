// 素数チェッカー

use std::env;

fn is_prime(n: usize) -> bool {
    if n <= 1 {
        return false;
    }
    for i in 2..=((n as f64).sqrt() as usize) {
        if n % i == 0 {
            return false;
        }
    }
    true
}

fn main() {
    // コマンドライン引数を取得
    let args: Vec<String> = env::args().collect();

    // 引数が正しいかチェック
    if args.len() != 2 {
        eprintln!("Usage: cargo run <number>");
        return;
    }

    // 引数を数値に変換
    let num: usize = match args[1].parse() {
        Ok(n) => n,
        Err(_) => {
            eprintln!("Please provide a valid number.");
            return;
        }
    };

    // 素数チェック
    if is_prime(num) {
        println!("{}は素数です。", num);
    } else {
        println!("{}は合成数です。", num);
    }
}
