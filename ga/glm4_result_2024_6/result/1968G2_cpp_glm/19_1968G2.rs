use std::io::{self, BufRead};
use std::cmp::Ordering;

const N: usize = 2e5 as usize + 3;

fn find(p: &mut Vec<usize>, k: usize) -> usize {
    if k > N || p[k] == k {
        k
    } else {
        p[k] = find(p, p[k]);
        p[k]
    }
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let mut t = lines.next().unwrap().unwrap().parse::<usize>().unwrap();
    while t > 0 {
        let mut n = lines.next().unwrap().unwrap().parse::<usize>().unwrap();
        let mut x = lines.next().unwrap().unwrap().parse::<usize>().unwrap();
        let mut y = lines.next().unwrap().unwrap().parse::<usize>().unwrap();
        let mut a = lines.next().unwrap().unwrap();
        let mut z = vec![0; N];
        let mut p = vec![0; N];
        let mut t = vec![0; N];
        let mut ans = vec![0; N];

        z[1] = n;
        for i in 2..=n {
            let mut j = 0;
            while j + z[j] > i {
                z[i] = std::cmp::min(z[i - j + 1], j + z[j] - i);
            }
            while a.chars().nth(i + z[i] - 1) == a.chars().nth(1 + z[i] - 1) {
                z[i] += 1;
            }
            if i + z[i] > j + z[j] {
                j = i;
            }
        }

        t[..n].sort_by(|&i, &j| z[i].cmp(&z[j]));
        for i in 1..=n {
            let mut k = 1;
            while k <= n && z[t[k]] < i {
                p[t[k]] = t[k] + 1;
                k += 1;
            }
            let mut c = 0;
            for j in 1..=n {
                c += 1;
                while find(&mut p, j + i) != j {
                    j = find(&mut p, j + i);
                }
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

        t -= 1;
    }
}