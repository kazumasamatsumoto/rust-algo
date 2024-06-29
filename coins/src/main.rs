use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let a: usize = input.trim().parse().expect("Invalid Input");

    input.clear();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let b: usize = input.trim().parse().expect("Invalid input");

    input.clear();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let c: usize = input.trim().parse().expect("Invalid input");

    input.clear();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let x: usize = input.trim().parse().expect("INvalid input");

    let mut count = 0;

    for i in 0..=a {
        for j in 0..=b {
            for k in 0..=c {
                if 500 * i + 100 * j + 50 * k == x {
                    count += 1;
                }
            }
        }
    }

    println!("{}", count);
}
