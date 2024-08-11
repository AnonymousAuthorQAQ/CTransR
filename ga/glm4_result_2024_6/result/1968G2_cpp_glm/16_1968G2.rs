use std::io::{self, BufRead};
use std::cmp::min;

const N: usize = 2 * 10_usize.pow(5) + 3;
const MAX: usize = N + 1;

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let mut t = 0;
    let mut n = 0;
    let mut x = 0;
    let mut y = 0;
    let mut a = vec![0; MAX];
    let mut p = vec![0; MAX];
    let mut z = vec![0; MAX];
    let mut t = vec![0; MAX];
    let mut ans = vec![0; MAX];

    loop {
        for i in 0..MAX {
            p[i] = i;
            t[i] = i;
            ans[i] = 0;
            z[i] = 0;
        }

        let mut line = lines.next().unwrap().unwrap();
        t = line.trim().parse::<usize>().unwrap();

        if t == 0 {
            break;
        }

        for _ in 0..t {
            n = lines.next().unwrap().unwrap().trim().parse::<usize>().unwrap();
            x = lines.next().unwrap().unwrap().trim().parse::<usize>().unwrap();
            y = lines.next().unwrap().unwrap().trim().parse::<usize>().unwrap();
            a = lines.next().unwrap().unwrap().chars().map(|c| c as usize).collect();

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

            t[1..=n].sort_by(|&i, &j| z[i].cmp(&z[j]));

            for i in 1..=n {
                let mut c = 0;
                while let Some(&k) = t.get(i) {
                    if z[k] < i {
                        p[k] = k + 1;
                        break;
                    }
                }
                for j in 1..=n {
                    let mut j = j;
                    while j <= n {
                        j = p[j];
                        c += 1;
                    }
                }
                ans[c] = i;
            }

            for i in (1..=n).rev() {
                ans[i] = max(ans[i], ans[i + 1]);
            }

            for i in x..=y {
                print!("{} ", ans[i]);
            }
            println!("");
        }
    }
}