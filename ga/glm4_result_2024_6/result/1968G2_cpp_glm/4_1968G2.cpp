use std::cmp::{Ordering, min};
use std::io::{self, BufRead};

const N: usize = 2e5 as usize + 3;
const MAXN: usize = N + 1;

fn find(p: &mut [usize], mut k: usize) -> usize {
    while k <= N {
        if p[k] == k {
            return k;
        }
        k = p[k];
    }
    panic!("Invalid index");
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let t = lines.next().unwrap().unwrap().parse::<usize>().unwrap();

    for _ in 0..t {
        let mut line = lines.next().unwrap().unwrap();
        let n = line.parse::<usize>().unwrap();
        let x = line.parse::<usize>().unwrap();
        let y = line.parse::<usize>().unwrap();
        let a = line.chars().collect::<Vec<char>>();

        let mut p = vec![0; MAXN];
        let mut t = (1..=n).collect::<Vec<usize>>();
        let mut ans = vec![0; MAXN];
        let mut z = vec![0; MAXN];

        z[1] = n;

        for i in 2..=n {
            let mut j = 0;
            while j + z[j] > i {
                z[i] = min(z[i - j + 1], j + z[j] - i);
            }
            while i + z[i] <= n && a[i + z[i]] == a[1 + z[i]] {
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
                j = find(&mut p, j + i);
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