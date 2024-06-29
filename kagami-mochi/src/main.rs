use std::collections::HashSet;
use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let n: usize = input.trim().parse().expect("Invalid input");

    let mut diameters = HashSet::new();

    for _ in 0..n {
        input.clear();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let d: usize = input.trim().parse().expect("Invalid input");
        diameters.insert(d);
    }

    let max_layers = diameters.len();
    println!("{}", max_layers)
}