// https://atcoder.jp/contests/past202004-open/tasks/past202004_c

use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let n: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
    let mut grid: Vec<Vec<char>> = Vec::new();

    for _ in 0..n {
        let line = lines.next().unwrap().unwrap();
        grid.push(line.chars().collect());
    }

    for i in (0..n - 1).rev() {
        for j in 1..(2 * n - 2) {
            if grid[i][j] == '#'
                && (grid[i + 1][j - 1] == 'X' || grid[i + 1][j] == 'X' || grid[i + 1][j + 1] == 'X')
            {
                grid[i][j] = 'X';
            }
        }
    }

    for row in grid {
        println!("{}", row.iter().collect::<String>());
    }
}
