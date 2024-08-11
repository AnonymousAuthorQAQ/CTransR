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
        self.st[..2 * self.n_].fill(0);
        for i in 0..n {
            self.st[self.n_ + i] = aa[i];
        }
        for i in (1..self.n_).rev() {
            self.st[i] = max(self.st[i << 1], self.st[i << 1 | 1]);
        }
    }

    fn query(&self, l: usize, a: i32) -> usize {
        let mut l = l + self.n_;
        let mut r = self.n_ - 1;

        while l <= r {
            if l & 1 == 1 {
                if self.st[l] >= a {
                    let mut index = l;
                    while index < self.n_ {
                        index = if self.st[index << 1] >= a {
                            index << 1
                        } else {
                            index << 1 | 1
                        };
                    }
                    return index - self.n_;
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
    let handle = stdin.lock();
    let reader = handle.lines().filter_map(Result::ok);
    let mut iter = reader.flat_map(|line| line.split_whitespace());
    
    let t: usize = iter.next().unwrap().parse().unwrap();
    
    for _ in 0..t {
        let n: usize = iter.next().unwrap().parse().unwrap();
        let kl: usize = iter.next().unwrap().parse().unwrap();
        let kr: usize = iter.next().unwrap().parse().unwrap();
        let cc: String = iter.next().unwrap();

        let mut zz = vec![0; n];
        let mut l = 0;
        let mut r = 0;

        for i in 1..n {
            if i + zz[i - l] < r {
                zz[i] = zz[i - l];
            } else {
                l = i;
                r = max(r, i);
                while r < n && cc[r..=r].chars().next() == cc[l..=l].chars().next() {
                    r += 1;
                }
                zz[i] = r - l;
            }
        }
        
        let mut st = SegmentTree::new();
        st.build(&zz, n);

        let mut ans = vec![0; n + 1];

        for p in 1..=n {
            let mut k = 0;
            let mut i = 0;
            while i < n {
                i = st.query(i + p, p);
                k += 1;
            }
            ans[k] = max(ans[k], p as i32);
        }

        for k in (1..=n - 1).rev() {
            ans[k] = max(ans[k], ans[k + 1]);
        }

        for k in kl..=kr {
            print!("{} ", ans[k]);
        }
        println!();
    }
}