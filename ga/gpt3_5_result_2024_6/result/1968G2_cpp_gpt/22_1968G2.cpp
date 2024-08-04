use std::cmp::{min, Ordering};
use std::collections::HashMap;
use std::io;

fn find(k: usize, p: &mut Vec<usize>) -> usize {
    if k > p.len() || p[k] == k {
        k
    } else {
        let root = find(p[k], p);
        p[k] = root;
        root
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let t: usize = input.trim().parse().unwrap();

    for _ in 0..t {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let tokens: Vec<usize> = input.split_whitespace()
                                    .map(|x| x.trim().parse().unwrap())
                                    .collect();
        let n = tokens[0];
        let x = tokens[1];
        let y = tokens[2];

        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let a: Vec<char> = input.trim().chars().collect();

        let mut p: Vec<usize> = (0..=n+1).collect();
        let mut t = (0..=n+1).collect::<Vec<usize>>();
        let mut ans = vec![0; n+2];
        let mut z = vec![0; n+2];
        z[1] = n;
        let mut i = 0;
        let mut j = 0;

        for i in 2..=n {
            if j + z[j] > i {
                z[i] = min(z[i-j+1], j+z[j]-i);
            }
            let mut k = i;
            while a[k] == a[1 + z[i]] {
                z[i] += 1;
                k += 1;
            }
            if i + z[i] > j + z[j] {
                j = i;
            }
        }

        let mut t_sorted = (1..=n).collect::<Vec<usize>>();
        t_sorted.sort_by(|&i, &j| z[i].partial_cmp(&z[j]).unwrap());

        let mut k = 1;
        for i in 1..=n {
            let mut c = 0;
            while k <= n && z[t_sorted[k]] < i {
                p[t_sorted[k]] = t_sorted[k] + 1;
                k += 1;
            }
            let mut j = 1;
            loop {
                j = find(j + i, &mut p);
                if j > n {
                    break;
                }
                c += 1;
            }
            ans[c] = i;
        }

        for i in (1..=n).rev() {
            ans[i] = ans[i].max(ans[i+1]);
        }

        for i in x..=y {
            print!("{} ", ans[i]);
        }
        println!();
    }
}