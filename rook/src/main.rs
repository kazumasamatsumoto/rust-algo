use std::collections::VecDeque;
use std::io::{self, BufRead};

/**
* 全体の流れ
* 標準入力で取得する
* 高さと幅を取得する
* 全体のマップを作成
* BFSの実行
* 上下左右に「進めるか否か？」というところでチェックをしながら進めたい
*/

fn main() {
    // 標準入力
    let stdin = io::stdin();
    // 標準入力をロックして、行ごとに読み取る準備をする
    // 1行ずつ読み取り、エラーがあればプログラムを終了させる
    let mut lines = stdin.lock().lines().map(|line| line.unwrap());

    // 最初の行から高さ H と幅 W を読み取る
    let first_line = lines.next().unwrap();
    let mut hw = first_line.split_whitespace();
    let h: usize = hw.next().unwrap().parse().unwrap();
    let w: usize = hw.next().unwrap().parse().unwrap();

    // グリッドの読み取りとスタート・ゴール位置の特定
    let mut grid = Vec::with_capacity(h); // 高さ分の容量を確保したベクタを作成
    let mut start = (0, 0); // スタート地点の初期化
    let mut _goal = (0, 0); // ゴール地点の初期化

    for i in 0..h {
        let line = lines.next().unwrap(); // 各行を読み取る
        let row: Vec<char> = line.chars().collect(); // 各行を文字ベクタに変換
        for j in 0..w {
            if row[j] == 's' {
                start = (i, j); // スタート地点を見つけたらその位置を保存
            } else if row[j] == 'g' {
                _goal = (i, j); // ゴール地点を見つけたらその位置を保存
            }
        }
        grid.push(row); // 読み取った行をグリッドに追加
    }

    // 飛車の動きを表す方向（東、西、南、北）
    let directions = [(0, 1), (0, -1), (1, 0), (-1, 0)];

    // BFSの初期化
    let mut queue = VecDeque::new();
    // 訪問していない場所を-1で初期化
    // 例えば、3x3のグリッドの場合
    // [  [-1, -1, -1],
    //    [-1, -1, -1],
    //    [-1, -1, -1]
    // ]
    let mut costs = vec![vec![-1; w]; h];
    // スタート地点をキューに追加し、コストを0に設定する
    // 例えば、スタート地点が (0, 0) の場合
    // [  [0, -1, -1],
    //    [-1, -1, -1],
    //    [-1, -1, -1]
    // ]
    queue.push_back(start);
    costs[start.0][start.1] = 0;

    // 幅優先探索（BFS）の開始
    while let Some((x, y)) = queue.pop_front() {
        // 現在の位置 (x, y) のコストを取得
        let current_cost = costs[x][y];

        // 4つの方向（東、西、南、北）に対してループを実行
        for &(dx, dy) in &directions {
            let mut nx = x as isize;
            let mut ny = y as isize;

            // 次の位置が道 ('.') またはゴール ('g') である限り進み続ける
            while let Some('.') | Some('g') = get(&grid, nx + dx, ny + dy) {
                nx += dx;
                ny += dy;

                // 新しい位置 (ux, uy) を計算する
                let ux = nx as usize;
                let uy = ny as usize;

                // 新しい位置が未訪問であれば、コストを更新してキューに追加する
                // 例えば、新しい位置が (0, 1) で未訪問 (-1) の場合
                // [  [0, 1, -1],
                //    [-1, -1, -1],
                //    [-1, -1, -1]
                // ]
                if costs[ux][uy] == -1 {
                    costs[ux][uy] = current_cost + 1;
                    queue.push_back((ux, uy));
                }

                // ゴールに到達した場合、コストを出力して終了する
                // 例えば、ゴールが (0, 2) にある場合
                // [  [0, 1, 2],
                //    [-1, -1, -1],
                //    [-1, -1, -1]
                // ]
                if grid[ux][uy] == 'g' {
                    println!("{}", costs[ux][uy]);
                    return;
                }
            }
        }
    }

    // ゴールに到達できなかった場合、-1を出力する
    println!("-1");
}

// グリッドの境界チェックと位置の文字を返す関数
fn get(grid: &Vec<Vec<char>>, x: isize, y: isize) -> Option<char> {
    // x と y がグリッドの範囲内であるかをチェック
    // 範囲外の場合は None を返す
    // 範囲内の場合はその位置の文字を返す
    if x >= 0 && x < grid.len() as isize && y >= 0 && y < grid[0].len() as isize {
        Some(grid[x as usize][y as usize])
    } else {
        None
    }
}
