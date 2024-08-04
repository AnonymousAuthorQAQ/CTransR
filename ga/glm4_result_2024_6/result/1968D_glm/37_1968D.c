use std::io;
use std::cmp::min;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let t: usize = input.trim().parse().expect("Please type a number");

    for _ in 0..t {
        input.clear();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let mut nums = input.trim().split_whitespace().map(|s| s.parse::<i32>().unwrap());
        let (n, k, a, b) = (nums.next().unwrap(), nums.next().unwrap(), nums.next().unwrap(), nums.next().unwrap());
        let m = min(n, k);

        let mut arr = vec![0; n + 1];
        let mut aa = vec![0; n + 1];

        for i in 1..=n {
            input.clear();
            io::stdin().read_line(&mut input).expect("Failed to read line");
            arr[i] = input.trim().parse().expect("Please type a number");
        }

        for i in 1..=n {
            input.clear();
            io::stdin().read_line(&mut input).expect("Failed to read line");
            aa[i] = input.trim().parse().expect("Please type a number");
        }

        let mut p1 = 0;
        let mut p2 = 0;
        let mut p1_ = 0;
        let mut p2_ = 0;
        let mut a = a;
        let mut b = b;

        for _ in 0..m {
            p1 = p1.max(1 * aa[a] * (k - _ + 1) + p1_);
            p1_ += aa[a];
            a = arr[a];

            p2 = p2.max(1 * aa[b] * (k - _ + 1) + p2_);
            p2_ += aa[b];
            b = arr[b];
        }

        if p1 > p2 {
            println!("Bodya");
        } else if p1 < p2 {
            println!("Sasha");
        } else {
            println!("Draw");
        }
    }
}