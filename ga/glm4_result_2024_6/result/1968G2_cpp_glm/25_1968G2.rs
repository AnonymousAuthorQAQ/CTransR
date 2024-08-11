use std::cmp::Ordering;
use std::io::{self, BufRead};

const N: usize = 2e5 as usize + 3;
const MAX_N: usize = N + 1;

fn find(p: &mut [usize], k: usize) -> usize {
    if k > N { return k; }
    if p[k] == k { return k; }
    p[k] = find(p, p[k]);
    p[k]
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
        let a = parts.next().unwrap().chars().map(|c| c as usize).collect::<Vec<usize>>();

        let mut p = vec![0; MAX_N];
        let mut t = (1..=n).collect::<Vec<usize>>();
        let mut ans = vec![0; MAX_N];
        let mut z = vec![0; MAX_N];

        z[1] = n;

        for i in 2..=n {
            let mut j = 0;
            while j + z[j] > i {
                z[i] = std::cmp::min(z[i - j + 1], j + z[j] - i);
            }
            while i + z[i] > j + z[j] {
                if a[i + z[i]] == a[1 + z[i]] {
                    z[i] += 1;
                } else {
                    break;
                }
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
                let j = find(&mut p, j + i);
                c += 1;
            }
            ans[c] = i;
        }

        for i in (1..=n).rev() {
            ans[i] = std::cmp::max(ans[i], ans[i + 1]);
        }

        for i in x..=y {
            print!("{} ", ans[i]);
        }
        println!();
    }
}