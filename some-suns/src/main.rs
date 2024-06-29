use std::io;

fn sum_of_digits(n: usize) -> usize {
    let mut sum = 0;
    let mut number = n;
    while number > 0 {
        sum += number % 10;
        number /= 10;
    }
    sum
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let parts: Vec<usize> = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse().expect("Not a number"))
        .collect();

    let n = parts[0];
    let a = parts[1];
    let b = parts[2];

    let mut total_sum = 0;

    for i in 1..=n {
        let digit_sum = sum_of_digits(i);
        if digit_sum >= a && digit_sum <= b {
            total_sum += i;
        }
    }

    println!("{}", total_sum);
}
