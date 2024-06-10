use std::env;

fn main() {
    // コマンドライン引数の取得
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: cargo run <N>");
        return;
    }

    // Nの取得
    let n: usize = args[1]
        .trim()
        .parse()
        .expect("Input was not a valid number");

    // ゾロ目数のリストを生成
    let mut zorome_numbers = vec![];

    for digit in 1..=9 {
        let mut number = digit;
        for _ in 0..6 {
            zorome_numbers.push(number);
            number = number * 10 + digit;
        }
    }

    // ソート
    zorome_numbers.sort();

    // N番目のゾロ目数を出力
    if n > 0 && n <= zorome_numbers.len() {
        println!("{}", zorome_numbers[n - 1]);
    } else {
        eprintln!("N is out of range");
    }
}
