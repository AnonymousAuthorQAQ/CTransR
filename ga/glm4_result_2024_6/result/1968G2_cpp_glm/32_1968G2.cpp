use std::io;
use std::cmp::Ordering;

const N: usize = 2e5 as usize + 3;

fn find(p: &mut [usize], k: usize) -> usize {
    if k > p.len() - 1 || p[k] == k {
        k
    } else {
        p[k] = find(p, p[k]);
        p[k]
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let t: usize = input.trim().parse().unwrap();

    for _ in 0..t {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let mut nums = input.trim().split_whitespace().map(|s| s.parse().unwrap());
        let n: usize = nums.next().unwrap();
        let x: usize = nums.next().unwrap();
        let y: usize = nums.next().unwrap();
        let mut a = [0; N];
        a[1..n + 1].copy_from_slice(input.trim().as_bytes());

        let mut p = [0; N];
        let mut t = [0; N];
        let mut ans = [0; N];
        let mut z = [0; N];

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

        t[1..=n].copy_from_slice(&z[1..=n]);
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
                j = find(&mut p, j + i);
            }
            ans[c] = i;
        }

        for i in (0..n).rev() {
            ans[i] = std::cmp::max(ans[i], ans[i + 1]);
        }

        for i in x..=y {
            print!("{} ", ans[i]);
        }
        println!("");
    }
}