use std::io::{self, BufRead};
use std::cmp::Ordering;

const N: usize = 2e5 as usize + 3;
const MOD: i64 = 1e9 as i64 + 7;

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let mut n: usize;
    let mut x: usize;
    let mut y: usize;
    let mut a = String::with_capacity(N);
    let mut z = vec![0; N];
    let mut p = vec![0; N];
    let mut t = vec![0; N];
    let mut ans = vec![0; N];

    let mut line = lines.next().unwrap().unwrap();
    let t: usize = line.trim().parse().unwrap();

    for _ in 0..t {
        line = lines.next().unwrap().unwrap();
        let mut iter = line.trim().split_whitespace();
        n = iter.next().unwrap().parse().unwrap();
        x = iter.next().unwrap().parse().unwrap();
        y = iter.next().unwrap().parse().unwrap();
        a.clear();
        a.push_str(iter.next().unwrap());

        for i in 1..=n + 1 {
            p[i] = i;
            t[i] = i;
            ans[i] = 0;
            z[i] = 0;
        }
        z[1] = n;

        for i in 2..=n {
            let mut j = 0;
            if j + z[j] > i {
                z[i] = std::cmp::min(z[i - j + 1], j + z[j] - i);
            }
            while a.chars().nth(i + z[i] - 1) == a.chars().nth(1 + z[i] - 1) {
                z[i] += 1;
            }
            if i + z[i] > j + z[j] {
                j = i;
            }
        }

        t.sort_unstable_by(|&i, &j| z[i].cmp(&z[j]));

        for i in 1..=n {
            let mut k = 1;
            while k <= n && z[t[k]] < i {
                p[t[k]] = t[k] + 1;
                k += 1;
            }
            let mut c = 0;
            for j in 1..=n {
                c += 1;
                j = find(j + i, &p);
            }
            ans[c] = i;
        }

        for i in (1..=n).rev() {
            ans[i] = std::cmp::max(ans[i], ans[i + 1]);
        }

        for i in x..=y {
            print!("{} ", ans[i]);
        }
        println!("");
    }
}

fn find mut x: usize, p: &mut [usize]) -> usize {
    if x > n { return x; }
    if p[x] == x { return x; }
    p[x] = find(p[x], p);
    x
}