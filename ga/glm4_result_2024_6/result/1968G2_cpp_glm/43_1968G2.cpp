use std::io::{self, Read};
use std::cmp::min;

const N: usize = 2e5 as usize + 3;

fn find(p: &mut [usize], k: usize) -> usize {
    if k > p.len() || p[k] == k {
        k
    } else {
        p[k] = find(p, p[k]);
        p[k]
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut lines = input.lines();

    let t: usize = lines.next().unwrap().parse().unwrap();

    for _ in 0..t {
        let mut tokens = lines.next().unwrap().split_whitespace();
        let n: usize = tokens.next().unwrap().parse().unwrap();
        let x: usize = tokens.next().unwrap().parse().unwrap();
        let y: usize = tokens.next().unwrap().parse().unwrap();
        let a = tokens.next().unwrap().as_bytes();

        let mut p = vec![0; n + 1];
        let mut t = (0..n + 1).collect::<Vec<usize>>();
        let mut ans = vec![0; n + 1];
        let mut z = vec![0; n + 1];

        z[0] = n;

        for i in 1..n {
            let mut j = 0;
            if j + z[j] > i {
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

        for i in 1..n {
            let mut c = 0;
            let mut k = 1;
            while k <= n && z[t[k]] < i {
                p[t[k]] = t[k] + 1;
                k += 1;
            }
            for j in 1..=n {
                c += 1;
                while j + find(&mut p, j) <= n {
                    j = find(&mut p, j);
                }
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