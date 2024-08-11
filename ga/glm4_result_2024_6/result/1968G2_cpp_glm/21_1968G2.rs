use std::io;
use std::cmp::min;
use std::io::{stdout, Write};
use std::str::FromStr;

const N: usize = 2e5 as usize + 3;
const INF: usize = N + 1;

fn find(p: &mut Vec<usize>, k: usize) -> usize {
    if k > N || p[k] == k {
        k
    } else {
        p[k] = find(p, p[k]);
        p[k]
    }
}

fn main() {
    let mut t = String::new();
    io::stdin().read_line(&mut t).unwrap();
    let t: usize = t.trim().parse().unwrap();

    for _ in 0..t {
        let mut nxy = String::new();
        io::stdin().read_line(&mut nxy).unwrap();
        let mut nxy = nxy.trim().split_whitespace();
        let n: usize = nxy.next().unwrap().parse().unwrap();
        let x: usize = nxy.next().unwrap().parse().unwrap();
        let y: usize = nxy.next().unwrap().parse().unwrap();

        let mut a = String::new();
        io::stdin().read_line(&mut a).unwrap();
        let a: Vec<char> = a.chars().collect();

        let mut p = vec![0; N];
        for i in 1..=N {
            p[i] = i;
        }

        let mut z = vec![0; N];
        z[1] = n;
        for i in 2..=n {
            let mut j = 0;
            while j + z[j] > i {
                z[i] = min(z[i - j + 1], j + z[j] - i);
            }
            while a[i + z[i]] == a[1 + z[i]] {
                z[i] += 1;
            }
            if i + z[i] > j + z[j] {
                j = i;
            }
        }

        let mut t = (1..=n).collect::<Vec<usize>>();
        t.sort_by(|&i, &j| z[i].cmp(&z[j]));

        let mut ans = vec![0; N];
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

        for i in (0..n).rev() {
            ans[i] = max(ans[i], ans[i + 1]);
        }

        for i in x..=y {
            print!("{} ", ans[i]);
        }
        println!("");
    }
}