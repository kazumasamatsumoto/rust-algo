use std::io;

fn main() {
    // 標準入力から文字列Sを読み込む
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let mut s = input.trim().to_string();

    // 調査する文字列リスト
    let patterns = ["dream", "dreamer", "erase", "eraser"];

    // 文字列の末尾からチェックしていく
    while !s.is_empty() {
        let mut matched = false;
        for &pattern in &patterns {
            if s.ends_with(pattern) {
                // パターンにマッチしたら、その部分を削除する
                let len = s.len();
                s.truncate(len - pattern.len());
                matched = true;
                break;
            }
        }
        if !matched {
            // どのパターンにもマッチしなければ、NOを出力
            println!("NO");
            return;
        }
    }

    // 文字列が空になればYESを出力
    println!("YES");
}
