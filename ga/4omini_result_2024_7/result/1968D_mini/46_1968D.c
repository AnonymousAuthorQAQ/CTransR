use std::io::{self, Read};

fn main() {
    let stdin = io::stdin();
    let mut input = String::new();
    stdin.read_to_string(&mut input).expect("Failed to read input");

    let mut lines = input.lines();
    let t: usize = lines.next().unwrap().parse().unwrap();

    for _ in 0..t {
        let first_line: Vec<usize> = lines.next().unwrap()
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();

        let n = first_line[0];
        let k = first_line[1];
        let mut a = first_line[2];
        let mut b = first_line[3];

        let m = n.min(k);
        let mut p1 = 0;
        let mut p2 = 0;
        let mut p1_ = 0;
        let mut p2_ = 0;

        let arr: Vec<usize> = lines.next().unwrap()
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();

        let aa: Vec<usize> = lines.next().unwrap()
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();

        for x in 0..m {
            let idx1 = a - 1; // Convert to zero-based index
            p1 = p1.max(aa[idx1] as i64 * (k - x as i64) + p1_);
            p1_ += aa[idx1] as i64;
            a = arr[idx1];

            let idx2 = b - 1; // Convert to zero-based index
            p2 = p2.max(aa[idx2] as i64 * (k - x as i64) + p2_);
            p2_ += aa[idx2] as i64;
            b = arr[idx2];
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