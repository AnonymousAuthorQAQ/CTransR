use std::cmp::{min, Ordering};
use std::io::{self, Read};

const N: usize = 2e5 as usize + 3;

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut lines = input.lines();

    let t: usize = lines.next().unwrap().parse().unwrap();

    for _ in 0..t {
        let mut line = lines.next().unwrap().split_whitespace();
        let n: usize = line.next().unwrap().parse().unwrap();
        let x: usize = line.next().unwrap().parse().unwrap();
        let y: usize = line.next().unwrap().parse().unwrap();
        let a = line.next().unwrap();

        let mut p = vec![0; n + 2];
        let mut t = vec![0; n + 2];
        let mut ans = vec![0; n + 2];
        let mut z = vec![0; n + 2];

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

        t[1..=n].copy_from_slice(&*(1..=n).collect::<Vec<usize>>());
        t[1..=n].sort_unstable_by(|&i, &j| z[i].cmp(&z[j]));

        for i in 1..=n {
            let mut k = 1;
            while k <= n && z[t[k]] < i {
                p[t[k]] = t[k] + 1;
                k += 1;
            }
            let mut c = 0;
            for j in 1..=n {
                c += 1;
                let mut j = Find(j + i, &p);
            }
            ans[c] = i;
        }

        for i in (0..=n).rev() {
            ans[i] = max(ans[i], ans[i + 1]);
        }

        for i in x..=y {
            print!("{} ", ans[i]);
        }
        println!();
    }
}

fn Find(x: usize, p: &Vec<usize>) -> usize {
    if x > p.len() - 1 || p[x] == x {
        x
    } else {
        Find(p[x], p)
    }
}