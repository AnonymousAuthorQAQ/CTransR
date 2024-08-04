use std::io::{self, Read, Write};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Failed to read input");

    let mut lines = input.lines();
    let t: usize = lines.next().expect("Expected number of test cases").parse().expect("Invalid test case count");

    for _ in 0..t {
        let nkab = lines.next().expect("Expected n, k, a, b values").split_whitespace().map(|s| s.parse::<usize>().expect("Invalid number")).collect::<Vec<_>>();
        let n = nkab[0];
        let k = nkab[1];
        let a = nkab[2];
        let b = nkab[3];

        let mut arr = Vec::with_capacity(n);
        let mut aa = Vec::with_capacity(n);

        for _ in 0..n {
            arr.push(lines.next().expect("Expected array element").parse::<usize>().expect("Invalid array element"));
        }

        for _ in 0..n {
            aa.push(lines.next().expect("Expected array element").parse::<usize>().expect("Invalid array element"));
        }

        let m = std::cmp::min(n, k);
        let mut p1 = 0;
        let mut p2 = 0;
        let mut p1_ = 0;
        let mut p2_ = 0;
        let mut a = a;
        let mut b = b;

        for _ in 0..m {
            p1 = std::cmp::max(p1, p1_ + aa[a]);
            p1_ += aa[a];
            a = arr[a];

            p2 = std::cmp::max(p2, p2_ + aa[b]);
            p2_ += aa[b];
            b = arr[b];
        }

        let result = if p1 > p2 { "Bodya\n" } else if p1 < p2 { "Sasha\n" } else { "Draw\n" };
        print!("{}", result);
    }
}