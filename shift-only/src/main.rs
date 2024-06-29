use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let n: usize = input.trim().parse().expect("Invalid input");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let mut numbers: Vec<u64> = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse().expect("Not a number"))
        .collect();

    let mut operations = 0;

    loop {
        if numbers.iter().all(|&x| x % 2 == 0) {
            numbers = numbers
                .iter()
                .map(|&x| x / 2)
                .collect();
            operations += 1;
        } else {
            break;
        }
    }

    println!("{}", operations);
}
