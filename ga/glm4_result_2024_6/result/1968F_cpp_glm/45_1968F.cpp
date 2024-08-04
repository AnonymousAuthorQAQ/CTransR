use std::io::{self, Read};

fn solve() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut lines = input.lines();

    let (n, q) = lines.next().unwrap().split_whitespace().map(|s| s.parse().unwrap()).collect_tuple().unwrap();
    let arr = lines.next().unwrap().split_whitespace().map(|s| s.parse().unwrap()).collect::<Vec<_>>();
    let mut x = vec![0; n + 1];
    let mut x2 = vec![0; n + 1];

    x[0] = 0;
    for i in 1..=n {
        x[i] = x[i - 1] ^ arr[i - 1];
        x2[i] = x[i];
    }

    for _ in 0..q {
        let (l, r) = lines.next().unwrap().split_whitespace().map(|s| s.parse().unwrap()).collect_tuple().unwrap();
        let c = x2[r] ^ x2[l - 1];

        if c == 0 {
            println!("YES");
            continue;
        }

        let i1 = x.binary_search_by(|&x| x.cmp(&x2[l - 1] ^ c)).unwrap_or_else(|x| x + 1);

        if i1 == n + 1 || x[i1] != x2[l - 1] ^ c || x[i1] < l {
            println!("NO");
            continue;
        }

        let i2 = x.binary_search_by(|&x| x.cmp(&x2[l - 1])).unwrap_or_else(|x| x + 1);

        if i2 == n + 1 || x[i2] != x2[l - 1] || x[i2] < x[i1] + 1 {
            println!("NO");
        } else {
            println!("YES");
        }
    }
}

fn main() {
    let t = io::stdin().lines().next().unwrap().parse::<i32>().unwrap();
    for _ in 0..t { solve(); }
}