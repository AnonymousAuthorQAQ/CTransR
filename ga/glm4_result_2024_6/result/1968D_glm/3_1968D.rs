use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut line = String::new();
    let mut t = 0;

    // Read number of test cases
    stdin.read_line(&mut line).unwrap();
    t = line.trim().parse::<i32>().unwrap();

    for _ in 0..t {
        let mut line = String::new();
        let mut n = 0;
        let mut k = 0;
        let mut a = 0;
        let mut b = 0;

        // Read array sizes, limit, and starting indices
        stdin.read_line(&mut line).unwrap();
        let mut parts = line.trim().split_whitespace();
        n = parts.next().unwrap().parse::<i32>().unwrap();
        k = parts.next().unwrap().parse::<i32>().unwrap();
        a = parts.next().unwrap().parse::<i32>().unwrap();
        b = parts.next().unwrap().parse::<i32>().unwrap();

        let mut arr = Vec::with_capacity(n as usize);
        let mut aa = Vec::with_capacity(n as usize);

        // Read the 'arr' array
        for _ in 0..n {
            let mut line = String::new();
            stdin.read_line(&mut line).unwrap();
            arr.push(line.trim().parse::<i32>().unwrap());
        }

        // Read the 'aa' array
        for _ in 0..n {
            let mut line = String::new();
            stdin.read_line(&mut line).unwrap();
            aa.push(line.trim().parse::<i32>().unwrap());
        }

        let m = std::cmp::min(n, k);
        let mut p1 = 0;
        let mut p2 = 0;
        let mut p1_ = 0;
        let mut p2_ = 0;
        let mut a = a;
        let mut b = b;

        // Calculate scores for Bodya and Sasha
        for x in 1..=m {
            p1 = std::cmp::max(p1, p1_ + aa[a as usize] * (k - x + 1));
            p1_ += aa[a as usize];
            a = arr[a as usize];

            p2 = std::cmp::max(p2, p2_ + aa[b as usize] * (k - x + 1));
            p2_ += aa[b as usize];
            b = arr[b as usize];
        }

        // Print the winner or draw
        if p1 > p2 {
            println!("Bodya");
        } else if p1 < p2 {
            println!("Sasha");
        } else {
            println!("Draw");
        }
    }
}