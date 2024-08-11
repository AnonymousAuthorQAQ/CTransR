use std::cmp::{min, Ordering};
use std::io::{self, BufRead};

const N: usize = 2e5 as usize + 3;

fn find(p: &mut Vec<usize>, k: usize) -> usize {
    if k > p.len() || p[k] == k {
        k
    } else {
        p[k] = find(p, p[k]);
        p[k]
    }
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let t: usize = lines.next().unwrap().unwrap().parse().unwrap();

    for _ in 0..t {
        let mut line = lines.next().unwrap().unwrap();
        let mut parts = line.split_whitespace();
        let n: usize = parts.next().unwrap().parse().unwrap();
        let x: usize = parts.next().unwrap().parse().unwrap();
        let y: usize = parts.next().unwrap().parse().unwrap();
        let a: Vec<char> = parts.next().unwrap().chars().collect();

        let mut p: Vec<usize> = (0..=n).collect();
        let mut t: Vec<usize> = (0..=n).collect();
        let mut ans: Vec<usize> = vec![0; n + 1];
        let mut z: Vec<usize> = vec![0; n + 1];

        z[0] = n;

        for i in 1..=n {
            let mut j = 0;
            while j + z[j] > i {
                z[i] = min(z[i - j + 1], j + z[j] - i);
            }
            while i + z[i] <= j + z[j] && a[i + z[i]] == a[1 + z[i]] {
                z[i] += 1;
            }
            if i + z[i] > j + z[j] {
                j = i;
            }
        }

        t.sort_by(|&i, &j| z[i].cmp(&z[j]));

        for i in 1..=n {
            let mut k = 1;
            while k <= n && z[t[k]] < i {
                p[t[k]] = t[k] + 1;
                k += 1;
            }
            let mut c = 0;
            for j in 1..=n {
                c += 1;
                j = find(&mut p, j + i);
            }
            ans[c] = i;
        }

        for i in (1..=n).rev() {
            ans[i] = max(ans[i], ans[i + 1]);
        }

        for i in x..=y {
            print!("{} ", ans[i]);
        }
        println!();
    }
}