use std::io::{self, BufRead};
use std::cmp::max;

const N: usize = 200000;
const N_: usize = 1 << 18; // N_ = pow2(ceil(log2(N)))

struct SegmentTree {
    st: [i32; N_ * 2],
    n_: usize,
}

impl SegmentTree {
    fn new() -> Self {
        Self {
            st: [0; N_ * 2],
            n_: 0,
        }
    }

    fn build(&mut self, aa: &[i32], n: usize) {
        self.n_ = 1;
        while self.n_ < n {
            self.n_ <<= 1;
        }
        self.st[0..self.n_ * 2].fill(0);
        for i in 0..n {
            self.st[self.n_ + i] = aa[i];
        }
        for i in (1..self.n_).rev() {
            self.st[i] = max(self.st[i << 1], self.st[i << 1 | 1]);
        }
    }

    fn query(&self, mut l: usize, mut a: i32) -> usize {
        let mut r = self.n_ - 1;

        l += self.n_;
        r += self.n_;
        while l <= r {
            if l & 1 == 1 {
                if self.st[l] >= a {
                    let mut idx = l;
                    while idx < self.n_ {
                        idx = if self.st[idx << 1] >= a {
                            idx << 1
                        } else {
                            idx << 1 | 1
                        };
                    }
                    return idx - self.n_;
                }
                l += 1;
            }
            l >>= 1;
            r >>= 1;
        }
        self.n_
    }
}

fn z_algorithm(cc: &str, n: usize) -> Vec<i32> {
    let mut zz = vec![0; n];
    let mut l = 0;
    let mut r = 0;

    for i in 1..n {
        if i + zz[i - l] < r {
            zz[i] = zz[i - l];
        } else {
            l = i;
            r = max(r, i);
            while r < n && cc.as_bytes()[r] == cc.as_bytes()[r - l] {
                r += 1;
            }
            zz[i] = r - l;
        }
    }
    zz
}

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let reader = handle.lines().filter_map(Result::ok);

    let mut t: usize = reader.take(1).map(|line| line.trim().parse().unwrap()).next().unwrap();
    
    for _ in 0..t {
        let input = reader.take(1).map(|line| line.trim().to_string()).next().unwrap();
        let mut parts = input.split_whitespace();
        let n: usize = parts.next().unwrap().parse().unwrap();
        let kl: usize = parts.next().unwrap().parse().unwrap();
        let kr: usize = parts.next().unwrap().parse().unwrap();
        let cc: String = parts.next().unwrap().into();

        let zz = z_algorithm(&cc, n);
        let mut st = SegmentTree::new();
        st.build(&zz, n);

        let mut ans = vec![0; n + 1];

        for p in 1..=n {
            let mut k = 0;
            let mut i = 0;

            while i < n {
                i = st.query(i + p, p as i32);
                if i < n {
                    k += 1;
                }
            }
            ans[k] = max(ans[k], p);
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