use std::io::{self, BufRead};
use std::cmp::Ordering;

const N: usize = 2e5 as usize + 3;

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let mut t = lines.next().unwrap().unwrap().parse::<i32>().unwrap();

    while t > 0 {
        let mut input = lines.next().unwrap().unwrap().split_whitespace();
        let n = input.next().unwrap().parse::<i32>().unwrap();
        let x = input.next().unwrap().parse::<i32>().unwrap();
        let y = input.next().unwrap().parse::<i32>().unwrap();
        let a = input.next().unwrap().chars().map(|c| c as i32 - 'a' as i32).collect::<Vec<i32>>();

        let mut p = vec![0; N];
        for i in 1..=n {
            p[i] = i;
        }

        let mut z = vec![0; N];
        z[1] = n as i32;

        for i in 2..=n {
            let mut j = 0;
            while j + z[j] > i {
                z[i] = std::cmp::min(z[i - j + 1], j + z[j] - i);
            }

            while i + z[i] <= j + z[j] && a[i + z[i]] == a[1 + z[i]] {
                z[i] += 1;
            }

            if i + z[i] > j + z[j] {
                j = i;
            }
        }

        let mut t = (1..=n).collect::<Vec<i32>>();
        t.sort_by(|i, j| z[*i as usize].cmp(&z[*j as usize]));

        let mut ans = vec![0; N];
        let mut k = 1;

        for i in 1..=n {
            let mut c = 0;
            while k <= n as i32 && z[t[k as usize] as usize] < i {
                p[t[k as usize] as usize] = t[k as usize] + 1;
                k += 1;
            }

            for j in 1..=n {
                c += 1;
                while j + i <= n as i32 {
                    j = p[j as usize];
                }
            }

            ans[c as usize] = i;
        }

        for i in (1..=n as usize).rev() {
            ans[i] = std::cmp::max(ans[i], ans[i + 1]);
        }

        for i in x..=y {
            print!("{} ", ans[i as usize]);
        }
        println!("");
        t -= 1;
    }
}