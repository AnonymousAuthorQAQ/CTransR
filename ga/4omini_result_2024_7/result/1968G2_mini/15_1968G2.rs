use std::io::{self, BufRead};
use std::cmp::max;

const N: usize = 200000;
const N_: usize = 1 << 18; // N_ = pow2(ceil(log2(N)))

struct SegmentTree {
    st: Vec<i32>,
    n_: usize,
}

impl SegmentTree {
    fn new(size: usize) -> Self {
        let mut n_ = 1;
        while n_ < size {
            n_ <<= 1;
        }
        SegmentTree {
            st: vec![0; 2 * n_],
            n_,
        }
    }

    fn build(&mut self, aa: &[i32], n: usize) {
        for i in 0..n {
            self.st[self.n_ + i] = aa[i];
        }
        for i in (1..self.n_).rev() {
            self.st[i] = max(self.st[i << 1], self.st[i << 1 | 1]);
        }
    }

    fn query(&mut self, l: usize, a: i32) -> usize {
        let mut r = self.n_ - 1;

        let mut l = l + self.n_;
        let mut r = r + self.n_;
        while l <= r {
            if l & 1 == 1 {
                if self.st[l] >= a {
                    while l < self.n_ {
                        l = if self.st[l << 1] >= a {
                            l << 1
                        } else {
                            l << 1 | 1
                        };
                    }
                    return l - self.n_;
                }
                l += 1;
            }
            l >>= 1;
            r >>= 1;
        }
        self.n_
    }
}

fn main() {
    let stdin = io::stdin();
    let mut reader = stdin.lock();

    let mut t = String::new();
    reader.read_line(&mut t).unwrap();
    let t: usize = t.trim().parse().unwrap();

    for _ in 0..t {
        let mut input = String::new();
        reader.read_line(&mut input).unwrap();
        let parts: Vec<&str> = input.split_whitespace().collect();
        let n: usize = parts[0].parse().unwrap();
        let kl: usize = parts[1].parse().unwrap();
        let kr: usize = parts[2].parse().unwrap();
        let cc: Vec<char> = parts[3].chars().collect();

        let mut zz = vec![0; n];
        let mut l = 0;
        let mut r = 0;

        for i in 1..n {
            if i + zz[i - l] < r {
                zz[i] = zz[i - l];
            } else {
                l = i;
                r = max(r, i);
                while r < n && cc[r] == cc[r - l] {
                    r += 1;
                }
                zz[i] = r - l;
            }
        }

        let mut st = SegmentTree::new(n);
        st.build(&zz, n);

        let mut ans = vec![0; n + 1];

        for p in 1..=n {
            let mut k = 0;
            let mut i = 0;

            while i < n {
                i = st.query(i + p, p);
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