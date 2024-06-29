use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    let mut iter = input.split_whitespace();
    let a: i32 = iter.next().unwrap().parse().expect("Invalid input");
    let b: i32 = iter.next().unwrap().parse().expect("Invalid input");

    if (a * b) % 2 == 0 {
        println!("Even")
    } else {
        println!("Odd")
    }
}
