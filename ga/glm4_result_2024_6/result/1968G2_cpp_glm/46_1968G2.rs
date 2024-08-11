use std::io;
use std::cmp::min;
use std::collections::HashMap;

const N: usize = 2e5 as usize + 3;

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
        let mut p = vec![0; n + 1];
        let mut t = (0..=n).collect::<Vec<usize>>();
        let mut ans = vec![0; n + 1];

        z[1] = n;

        for i in 2..=n {
            let mut j = 0;
            while i + z[j] > i {
                z[i] = min(z[i - j + 1], j + z[j] - i);
                if a.chars().nth(i + z[i]) == a.chars().nth(1 + z[i]) {
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
                let mut parent = j;
                while parent <= n {
                    c += 1;
                    parent = p[parent];
                }
            }
            ans[c] = i;
        }

        for i in (0..n).rev() {
            ans[i] = max(ans[i], ans[i + 1]);
        }

        for i in x..=y {
            println!("{} ", ans[i]);
        }
        println!("");
    }
}