use std::io;

fn main() {
    // 標準入力からデータを読み込む
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let n: usize = input.trim().parse().expect("Invalid input");

    let mut t_prev = 0;
    let mut x_prev = 0;
    let mut y_prev = 0;

    for _ in 0..n {
        input.clear();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let parts: Vec<i32> = input
            .trim()
            .split_whitespace()
            .map(|x| x.parse().expect("Not a number"))
            .collect();
        
        let t = parts[0];
        let x = parts[1];
        let y = parts[2];

        let dt = t - t_prev;
        let dx = (x - x_prev).abs();
        let dy = (y - y_prev).abs();

        // 移動距離が時間以内に収まるか、および偶奇が一致するかを確認
        if dx + dy > dt || (dt - dx - dy) % 2 != 0 {
            println!("No");
            return;
        }

        t_prev = t;
        x_prev = x;
        y_prev = y;
    }

    println!("Yes");
}
