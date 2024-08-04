use std::io::{self, BufRead};
use std::cmp::min;

const N: usize = 2e5 as usize + 3;

fn find(p: &mut Vec<usize>, k: usize) -> usize {
    if k > p.len() - 1 || p[k] == k {
        k
    } else {
        p[k] = find(p, p[k]);
        p[k]
    }
}

fn main() {
    let stdin = io::stdin();
    let mut line = String::new();

    let mut t = 0;
    stdin.read_line(&mut line).unwrap();
    t = line.trim().parse::<i32>().unwrap();

    while t > 0 {
        let mut n = 0;
        let mut x = 0;
        let mut y = 0;
        line.clear();
        stdin.read_line(&mut line).unwrap();
        let mut tokens = line.trim().split_whitespace();
        n = tokens.next().unwrap().parse::<usize>().unwrap();
        x = tokens.next().unwrap().parse::<usize>().unwrap();
        y = tokens.next().unwrap().parse::<usize>().unwrap();

        let a = tokens.next().unwrap();

        let mut p = vec![0; n + 1];
        for i in 0..=n {
            p[i] = i;
        }

        let mut z = vec![0; n + 1];
        z[0] = n;

        for i in 1..=n {
            let mut j = 0;
            while j + z[j] > i {
                z[i] = min(z[i - j + 1], j + z[j] - i);
            }
            while i + z[i] > j + z[j] && i + z[i] <= n {
                j = i;
            }
        }

        let mut t = (0..=n).collect::<Vec<usize>>();
        t.sort_by(|&i, &j| z[i].cmp(&z[j]));

        let mut ans = vec![0; n + 1];
        for i in 1..=n {
            let mut k = 1;
            while k <= n && z[t[k]] < i {
                p[t[k]] = t[k] + 1;
                k += 1;
            }
            let mut c = 0;
            for j in 1..=n {
                if find(&mut p, j + i) == j {
                    c += 1;
                }
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

        t -= 1;
    }
}