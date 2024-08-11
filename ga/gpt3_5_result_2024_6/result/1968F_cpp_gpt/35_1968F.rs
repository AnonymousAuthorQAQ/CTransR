use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines().map(|line| line.unwrap());

    let t: i32 = lines.next().unwrap().parse().unwrap();
    for _ in 0..t {
        let line: String = lines.next().unwrap();
        let mut parts = line.split_whitespace();
        let n: usize = parts.next().unwrap().parse().unwrap();
        let q: usize = parts.next().unwrap().parse().unwrap();

        let line: String = lines.next().unwrap();
        let arr: Vec<i32> = line.split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();

        let mut x: Vec<(i32, usize)> = vec![(0, 0)];
        for i in 1..=n {
            x.push((x[i-1].0 ^ arr[i-1], i));
        }

        x.sort();

        for _ in 0..q {
            let line: String = lines.next().unwrap();
            let mut parts = line.split_whitespace();
            let l: usize = parts.next().unwrap().parse().unwrap();
            let r: usize = parts.next().unwrap().parse().unwrap();

            let c = x[r].0 ^ x[l - 1].0;
            if c == 0 {
                println!("YES");
                continue;
            }

            let i1 = x.binary_search_by_key(&(c ^ x[l - 1].0), |&(val, _)| val).unwrap_or_else(|x| x);
            if i1 == n + 1 || x[i1].1 > r || x[i1].0 != (c ^ x[l - 1].0) {
                println!("NO");
                continue;
            }

            let i2 = x.binary_search_by_key(&x[l - 1].0, |&(val, _)| val).unwrap_or_else(|x| x);
            if i2 == n + 1 || x[i2].1 > r || x[i2].0 != x[l - 1].0 {
                println!("NO");
            } else {
                println!("YES");
            }
        }
    }
}