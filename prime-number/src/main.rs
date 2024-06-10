// エラトステネスの篩を使用して素数を求める
// 今回は10000までの素数を求める
// 平方根は100となる

fn main() {
    let limit = 10000;
    let mut primes = vec![true; limit + 1];
    primes[0] = false;
    primes[1] = false;

    for num in 2..=((limit as f64).sqrt() as usize) {
        if primes[num] {
            let mut multiple = num * num;
            while multiple <= limit {
                primes[multiple] = false;
                multiple += num;
            }
        }
    }

    for (num, is_prime) in primes.iter().enumerate() {
        if *is_prime {
            println!("{}", num);
        }
    }
}
