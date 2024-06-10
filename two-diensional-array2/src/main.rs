// https://atcoder.jp/contests/abc088/tasks/abc088_c

use std::io::{self, BufRead};

fn main() {
    // 標準入力から読み込む
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // 3x3 グリッドを読み込む
    let mut c = [[0; 3]; 3];
    for i in 0..3 {
        if let Some(Ok(line)) = lines.next() {
            let numbers: Vec<i32> = line
                .split_whitespace()
                .map(|s| s.parse().expect("Parse error"))
                .collect();
            for j in 0..3 {
                c[i][j] = numbers[j];
            }
        }
    }

    // a[0] を 0 として b[j] を計算
    let mut b = [0; 3];
    for j in 0..3 {
        b[j] = c[0][j];
    }

    // a[i] を計算
    let mut a = [0; 3];
    for i in 1..3 {
        a[i] = c[i][0] - b[0];
    }

    // 条件を満たしているかをチェック
    let mut is_correct = true;
    for i in 0..3 {
        for j in 0..3 {
            if c[i][j] != a[i] + b[j] {
                is_correct = false;
                break;
            }
        }
        if !is_correct {
            break;
        }
    }

    // 結果を出力
    if is_correct {
        println!("Yes");
    } else {
        println!("No");
    }
}
