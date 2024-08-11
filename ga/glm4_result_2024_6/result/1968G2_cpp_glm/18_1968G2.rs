use std::io::{self, BufRead};
use std::collections::HashMap;
use std::cmp::Ordering;

const N: usize = 2_000_03;

fn find(p: &mut Vec<usize>, k: usize) -> usize {
    if k > p.len() - 1 || p[k] == k {
        k
    } else {
        let mut r = find(p, p[k]);
        p[k] = r;
        r
    }
}

fn main() {
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut t = 1;
    loop {
        let mut line = String::new();
        if t == 0 {
            break;
        }
        t -= 1;
        stdin.read_line(&mut line).unwrap();
        let mut tokens = line.trim().split_whitespace();
        let n: usize = tokens.next().unwrap().parse().unwrap();
        let x: usize = tokens.next().unwrap().parse().unwrap();
        let y: usize = tokens.next().unwrap().parse().unwrap();
        let mut a = Vec::new();
        a.extend(tokens.next().unwrap().chars());

        let mut p: Vec<usize> = (0..n+2).collect();
        let mut t: Vec<usize> = (0..n+2).collect();
        let mut ans: Vec<usize> = (0..n+2).collect();
        let mut z: Vec<usize> = (0..n+2).collect();
        z[1] = n;

        for i in 2..n+1 {
            let mut j = 0;
            if j + z[j] > i {
                z[i] = std::cmp::min(z[i-j+1], j + z[j] - i);
            }
            while a[i+z[i]] == a[1+z[i]] {
                z[i] += 1;
            }
            if i + z[i] > j + z[j] {
                j = i;
            }
        }

        t[1..n+1].sort_by(|i, j| z[*i].cmp(&z[*j]));

        for i in 1..n+1 {
            let mut k = 1;
            while k <= n && z[t[k]] < i {
                p[t[k]] = t[k] + 1;
                k += 1;
            }
            let mut c = 0;
            for j in 1..n+1 {
                c += 1;
                while let Some(i) = find(&mut p, j+i) {
                    if p[i] == i {
                        break;
                    }
                }
            }
            ans[c] = i;
        }

        for i in (0..n).rev() {
            ans[i] = std::cmp::max(ans[i], ans[i+1]);
        }

        for i in x..=y {
            stdout.write_all(ans[i].to_string().as_bytes()).unwrap();
            stdout.write_all(b" ").unwrap();
        }
        stdout.write_all(b"\n").unwrap();
    }
}