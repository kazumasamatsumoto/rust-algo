use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let n: usize = input.trim().parse().expect("Invalid input");

    input.clear();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let mut cards: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse().expect("Not a number"))
        .collect();

    cards.sort_by(|a, b| b.cmp(a));

    let mut alice_score = 0;
    let mut bob_score = 0;

    for (i, &card) in cards.iter().enumerate() {
        if i % 2 == 0 {
            alice_score += card;
        } else {
            bob_score += card;
        }
    }

    let score_difference = alice_score - bob_score;
    println!("{}", score_difference);
}
