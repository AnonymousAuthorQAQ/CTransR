use std::io::{self, BufRead};

fn max(a: i32, b: i32) -> i32 {
    a.max(b)
}

fn build(aa: &mut [i32], n: usize) {
    let mut n_ = 1;
    while n_ < n {
        n_ <<= 1;
    }
    let mut st = vec![0; n_ * 2];
    for i in 0..n {
        st[n_ + i] = aa[i];
    }
    for i in (1..n_).rev() {
        st[i] = max(st[i << 1], st[i << 1 | 1]);
    }
    aa.copy_from_slice(&st[n..n_]);
}

fn query(aa: &[i32], l: usize, a: i32) -> usize {
    let mut l = l + aa.len();
    let mut r = aa.len() - 1;
    while l <= r {
        if (l & 1) == 1 {
            if aa[l] >= a {
                while l < aa.len() {
                    l = if aa[l << 1] >= a { l << 1 } else { l << 1 | 1 };
                }
                return l - aa.len();
            }
            l += 1;
        }
        l >>= 1;
        r >>= 1;
    }
    aa.len()
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let t: usize = lines.next().unwrap().unwrap().parse().unwrap();

    for _ in 0..t {
        let mut line = lines.next().unwrap().unwrap();
        let n: usize = line.parse().unwrap();
        let kl: usize = line.parse().unwrap();
        let kr: usize = line.parse().unwrap();
        let mut cc: Vec<char> = line.chars().collect();
        let mut zz: Vec<i32> = vec![0; n];

        for i in 1..n {
            if i + zz[i - 1] < n {
                zz[i] = zz[i - 1];
            } else {
                let l = i;
                let mut r = i;
                while r < n && cc[r] == cc[r - l] {
                    r += 1;
                }
                zz[i] = r - l;
            }
        }

        build(&mut zz, n);
        let mut ans = vec![0; n + 1];

        for p in 1..=n {
            let mut k = 0;
            for i in 0..n {
                let l = query(&zz, i, p as i32);
                k += n - l;
            }
            ans[k] = max(ans[k], p as i32);
        }

        for k in (1..n).rev() {
            ans[k] = max(ans[k], ans[k + 1]);
        }

        for k in kl..=kr {
            print!("{} ", ans[k]);
        }
        println!();
    }
}