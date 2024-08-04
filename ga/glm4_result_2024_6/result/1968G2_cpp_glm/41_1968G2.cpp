use std::cmp::Ordering;
use std::io::{self, BufRead};

const N: usize = 2e5 as usize + 3;

fn find(p: &mut [usize], k: usize) -> usize {
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

    let t: usize = lines.next().unwrap().unwrap().parse().unwrap();

    for _ in 0..t {
        let mut line = lines.next().unwrap().unwrap();
        let n: usize = line.parse::<usize>().unwrap();
        let x: usize = line.parse::<usize>().unwrap();
        let y: usize = line.parse::<usize>().unwrap();
        let a: Vec<char> = line.chars().collect();

        let mut p: Vec<usize> = (0..=n + 1).collect();
        let mut t: Vec<usize> = (0..=n).collect();
        let mut ans: Vec<usize> = (0..=n).collect();
        let mut z: Vec<usize> = (0..=n + 1).collect();

        z[1] = n;

        for i in 2..=n {
            let mut j = 0;
            while j + z[j] > i {
                z[i] = std::cmp::min(z[i - j + 1], j + z[j] - i);
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
        println!("");
    }
}