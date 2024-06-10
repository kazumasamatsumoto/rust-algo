// ・二次元配列
// ・3×3のビンゴカードが与えられる
// ・ビンゴカードとは別にN個の数が与えられるため、ビンゴカードに与えられた数が存在すれば、ビンゴカードのその数に印をつける
// ・印がついた数が縦横斜めのいずれかに３つ並んでいたら'Yes'を出力する。そうでなければ'No'を出力する
// ・入力はすべて整数

use std::io;

fn main() {
    let mut bingo_card = vec![vec![0; 3]; 3];
    let mut marked = vec![vec![false; 3]; 3];

    // ビンゴカードの入力
    for i in 0..3 {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let nums: Vec<i32> = input
            .trim()
            .split_whitespace()
            .map(|num| num.parse().expect("Please enter a number"))
            .collect();
        for j in 0..3 {
            bingo_card[i][j] = nums[j];
        }
    }

    // Nの数の入力
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let n: usize = input.trim().parse().expect("Please enter a number");

    for _ in 0..n {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let num: i32 = input.trim().parse().expect("Please enter a number");

        // ビンゴカードにある数に印をつける
        for i in 0..3 {
            for j in 0..3 {
                if bingo_card[i][j] == num {
                    marked[i][j] = true;
                }
            }
        }
    }

    // ビンゴのチェック
    let mut bingo = false;

    // 横のチェック
    for i in 0..3 {
        if marked[i][0] && marked[i][1] && marked[i][2] {
            bingo = true;
        }
    }

    // 縦のチェック
    for j in 0..3 {
        if marked[0][j] && marked[1][j] && marked[2][j] {
            bingo = true;
        }
    }

    // 斜めのチェック
    if (marked[0][0] && marked[1][1] && marked[2][2])
        || (marked[0][2] && marked[1][1] && marked[2][0])
    {
        bingo = true;
    }

    // 結果の出力
    if bingo {
        println!("Yes");
    } else {
        println!("No");
    }
}
