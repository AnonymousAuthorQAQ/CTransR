use std::cmp::Ordering;
use std::io::{self, BufRead};
use std::vec::Vec;

#[derive(Debug)]
struct Pair {
    xor_value: i64,
    index: usize,
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    let t: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();

    for _ in 0..t {
        let first_line = lines.next().unwrap().unwrap();
        let mut parts = first_line.split_whitespace();
        let n: usize = parts.next().unwrap().parse().unwrap();
        let q: usize = parts.next().unwrap().parse().unwrap();

        let mut arr: Vec<i64> = lines.next().unwrap().unwrap()
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();

        let mut x: Vec<Pair> = vec![Pair { xor_value: 0, index: 0 }];
        let mut x2: Vec<i64> = vec![0; n + 1];

        for i in 1..=n {
            x.push(Pair { xor_value: x[i - 1].xor_value ^ arr[i - 1], index: i });
            x2[i] = x[i].xor_value;
        }

        x.sort_by(|a, b| a.xor_value.cmp(&b.xor_value));

        for _ in 0..q {
            let query_line = lines.next().unwrap().unwrap();
            let mut query_parts = query_line.split_whitespace();
            let l: usize = query_parts.next().unwrap().parse().unwrap();
            let r: usize = query_parts.next().unwrap().parse().unwrap();

            let c = x2[r] ^ x2[l - 1];
            if c == 0 {
                println!("YES");
                continue;
            }

            let target_xor = c ^ x2[l - 1];
            let i1 = match x.binary_search_by(|pair| pair.xor_value.cmp(&target_xor)) {
                Ok(i) => i,
                Err(i) => i,
            };

            if i1 == x.len() || x[i1].index > r || x[i1].xor_value != target_xor {
                println!("NO");
                continue;
            }

            let i2 = match x.binary_search_by(|pair| pair.xor_value.cmp(&x2[l - 1])) {
                Ok(i) => i,
                Err(i) => i,
            };

            if i2 == x.len() || x[i2].index > r || x[i2].xor_value != x2[l - 1] {
                println!("NO");
            } else {
                println!("YES");
            }
        }
    }
}