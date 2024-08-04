use std::io;
use std::cmp::min;
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
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let t: usize = input.trim().parse().unwrap();

    for _ in 0..t {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let mut tokens = input.trim().split_whitespace();
        let n: usize = tokens.next().unwrap().parse().unwrap();
        let x: usize = tokens.next().unwrap().parse().unwrap();
        let y: usize = tokens.next().unwrap().parse().unwrap();
        let a = tokens.next().unwrap();

        let mut p = vec![0; N];
        let mut t = vec![0; N];
        let mut ans = vec![0; N];
        let mut z = vec![0; N];

        for i in 1..=n + 1 {
            p[i] = i;
            t[i] = i;
            ans[i] = 0;
            z[i] = 0;
        }
        z[1] = n;

        for i in 2..=n {
            let mut j = 0;
            while j + z[j] > i {
                z[i] = min(z[i - j + 1], j + z[j] - i);
            }
            while a.chars().nth(i + z[i] - 1) == a.chars().nth(1 + z[i] - 1) {
                z[i] += 1;
            }
            if i + z[i] > j + z[j] {
                j = i;
            }
        }

        t[1..=n].sort_by(|i, j| z[*i].cmp(&z[*j]));

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