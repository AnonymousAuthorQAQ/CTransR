use std::cmp::min;
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
        let mut parts = input.trim().split_whitespace();
        let n: usize = parts.next().unwrap().parse().unwrap();
        let x: usize = parts.next().unwrap().parse().unwrap();
        let y: usize = parts.next().unwrap().parse().unwrap();
        let a: String = parts.next().unwrap().to_string();

        let mut z = vec![0; n + 1];
        let mut p = (0..=n).collect::<Vec<usize>>();
        let mut t = (0..=n).collect::<Vec<usize>>();
        let mut ans = vec![0; n + 1];

        let mut j = 0;
        z[1] = n;
        for i in 2..=n {
            if j + z[j] > i {
                z[i] = min(z[i - j + 1], j + z[j] - i);
            }
            let mut k = i;
            while a.chars().nth(k-1).unwrap() == a.chars().nth(1 + z[i]).unwrap() {
                z[i] += 1;
            }
            if i + z[i] > j + z[j] {
                j = i;
            }
        }

        t.sort_by(|&i, &j| z[i].cmp(&z[j]));

        let mut k = 1;
        for i in 1..=n {
            let mut c = 0;
            while k <= n && z[t[k]] < i {
                p[t[k]] = t[k] + 1;
                k += 1;
            }
            let mut j = 1;
            loop {
                let root = find(j + i, &mut p);
                c += 1;
                if root > n {
                    break;
                }
                j = root;
            }
            ans[c] = i;
        }

        for i in (1..=n).rev() {
            ans[i] = ans[i].max(ans[i + 1]);
        }

        for i in x..=y {
            print!("{} ", ans[i]);
        }
        println!();
    }
}