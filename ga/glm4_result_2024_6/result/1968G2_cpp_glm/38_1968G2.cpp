use std::cmp::{min, Ordering};
use std::io::{self, Read};

const N: usize = 2e5 as usize + 3;

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut lines = input.lines();
    let t: usize = lines.next().unwrap().parse().unwrap();

    for _ in 0..t {
        let mut line = lines.next().unwrap();
        let n: usize = line.parse::<usize>().unwrap();
        let x: usize = line.parse::<usize>().unwrap();
        let y: usize = line.parse::<usize>().unwrap();
        let a: Vec<char> = line.chars().collect();

        let mut p = vec![0; N];
        for i in 0..n {
            p[i] = i;
        }
        let mut z = vec![0; N];
        let mut t = (0..n).collect::<Vec<usize>>();
        let mut ans = vec![0; N];

        z[0] = n;
        for i in 1..n {
            let mut j = 0;
            while j + z[j] > i {
                z[i] = min(z[i - j], j + z[j] - i);
            }
            while i + z[i] > j + z[j] && a[i + z[i]] == a[1 + z[i]] {
                z[i] += 1;
            }
            if i + z[i] > j + z[j] {
                j = i;
            }
        }

        t.sort_by(|&i, &j| z[i].cmp(&z[j]));
        for i in 0..n {
            let mut k = 0;
            while k < n && z[t[k]] < i {
                p[t[k]] = t[k] + 1;
                k += 1;
            }
            let mut c = 0;
            for j in 0..n {
                let mut j = j;
                while j <= n && p[j] != j {
                    c += 1;
                    j = p[j];
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