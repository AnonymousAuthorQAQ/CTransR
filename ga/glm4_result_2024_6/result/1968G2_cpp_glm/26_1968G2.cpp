use std::io::{self, Read};
use std::cmp::Ordering;
use std::collections::HashMap;

const N: usize = 2e5 as usize + 3;
const MOD: i64 = 1e9 as i64 + 7;

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut lines = input.lines();
    
    let t: usize = lines.next().unwrap().parse().unwrap();
    for _ in 0..t {
        let mut parts = lines.next().unwrap().split_whitespace().map(|s| s.parse().unwrap());
        let n = parts.next().unwrap();
        let x = parts.next().unwrap();
        let y = parts.next().unwrap();
        let s = parts.next().unwrap();
        let mut z = vec![0; n + 1];
        let mut p = vec![0; n + 1];
        let mut t = vec![0; n + 1];
        let mut ans = vec![0; n + 1];

        z[0] = n;
        for i in 1..=n {
            let mut j = 0;
            while j + z[j] > i {
                z[i] = std::cmp::min(z[i - j + 1], j + z[j] - i);
            }
            while i + z[i] > j + z[j] {
                j = i;
            }
        }

        t[0..n].sort_by(|&i, &j| z[i].cmp(&z[j]));
        for i in 1..=n {
            let mut k = 0;
            while k <= n && z[t[k]] < i {
                p[t[k]] = t[k] + 1;
                k += 1;
            }
            let mut c = 0;
            for j in 1..=n {
                let mut j = j;
                while j <= n {
                    c += 1;
                    j = p[j];
                }
            }
            ans[c] = i;
        }

        for i in (0..n).rev() {
            ans[i] = std::cmp::max(ans[i], ans[i + 1]);
        }

        for i in x..=y {
            print!("{} ", ans[i]);
        }
        println!("");
    }
}