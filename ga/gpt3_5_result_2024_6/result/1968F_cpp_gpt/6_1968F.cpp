use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut line = String::new();

    stdin.lock().read_line(&mut line).unwrap();
    let t: i32 = line.trim().parse().unwrap();

    for _ in 0..t {
        line.clear();
        stdin.lock().read_line(&mut line).unwrap();
        let mut iter = line.split_whitespace();
        let n: usize = iter.next().unwrap().parse().unwrap();
        let q: usize = iter.next().unwrap().parse().unwrap();

        line.clear();
        stdin.lock().read_line(&mut line).unwrap();
        let arr: Vec<i64> = line.split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();

        let mut x: Vec<(i64, usize)> = vec![(0, 0); n + 1];
        let mut x2: Vec<i64> = vec![0; n + 1];
        x[0] = (0, 0);

        for i in 1..=n {
            x[i] = (x[i - 1].0 ^ arr[i - 1], i);
        }

        for i in 0..=n {
            x2[i] = x[i].0;
        }

        x.sort();

        for _ in 0..q {
            line.clear();
            stdin.lock().read_line(&mut line).unwrap();
            let mut iter = line.split_whitespace();
            let l: usize = iter.next().unwrap().parse().unwrap();
            let r: usize = iter.next().unwrap().parse().unwrap();

            let c = x2[r] ^ x2[l - 1];
            if c == 0 {
                println!("YES");
                continue;
            }

            let i1 = x.binary_search_by_key(&(c ^ x2[l - 1]), |&(val, _)| val)
                .unwrap_or_else(|x| x) as usize;

            if i1 == n + 1 || x[i1].1 > r || x[i1].0 != (c ^ x2[l - 1]) {
                println!("NO");
                continue;
            }

            let i2 = x.binary_search_by_key(&x2[l - 1], |&(val, _)| val)
                .unwrap_or_else(|x| x) as usize;

            if i2 == n + 1 || x[i2].1 > r || x[i2].0 != x2[l - 1] {
                println!("NO");
            } else {
                println!("YES");
            }
        }
    }
}