use std::io;
use std::io::{stdout, Write};
use std::cmp::min;
use std::collections::HashMap;

const N: usize = 2e5 as usize + 3;
const MAX_N: usize = N + 1;

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
    let t = input.trim().parse::<usize>().unwrap();

    for _ in 0..t {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let mut numbers = input.trim().split_whitespace().map(|s| s.parse::<usize>().unwrap());
        let n = numbers.next().unwrap();
        let x = numbers.next().unwrap();
        let y = numbers.next().unwrap();
        let a = numbers.next().unwrap();

        let mut p: Vec<usize> = (0..MAX_N).collect();
        let mut z: Vec<usize> = vec![0; MAX_N];
        let mut ans: Vec<usize> = vec![0; MAX_N];
        let mut t: Vec<usize> = (1..=n).collect();

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

        t.sort_by(|&i, &j| z[i].cmp(&z[j]));

        for i in 1..=n {
            let mut k = 1;
            while k <= n && z[t[k]] < i {
                p[t[k]] = t[k] + 1;
                k += 1;
            }
            let mut count = 0;
            for j in 1..=n {
                let mut idx = find(&mut p, j);
                while idx <= n {
                    count += 1;
                    idx = find(&mut p, idx + i);
                }
            }
            ans[count] = i;
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