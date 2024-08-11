use std::cmp::max;
use std::io::{self, BufRead};

const N: usize = 200_000;
const N_: usize = 1 << 18; // N_ = pow2(ceil(log2(N)))

struct SegmentTree {
    st: Vec<i32>,
    n_: usize,
}

impl SegmentTree {
    fn new(n: usize) -> SegmentTree {
        let mut n_ = 1;
        while n_ < n {
            n_ <<= 1;
        }
        SegmentTree {
            st: vec![0; n_ * 2],
            n_,
        }
    }

    fn build(&mut self, aa: &[i32]) {
        let n = aa.len();
        for i in 0..n {
            self.st[self.n_ + i] = aa[i];
        }
        for i in (1..self.n_).rev() {
            self.st[i] = max(self.st[i << 1], self.st[i << 1 | 1]);
        }
    }

    fn query(&self, l: usize, a: i32) -> usize {
        let mut l = l + self.n_;
        let mut r = self.n_ - 1; // inclusive

        while l <= r {
            if l & 1 == 1 {
                if self.st[l] >= a {
                    let mut idx = l;
                    while idx < self.n_ {
                        idx = if self.st[idx << 1] >= a {
                            idx << 1
                        } else {
                            idx << 1 | 1
                        }
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

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines().map(|line| line.unwrap());
    
    let t: usize = lines.next().unwrap().parse().unwrap();
    
    for _ in 0..t {
        let first_line: Vec<usize> = lines.next().unwrap()
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        let (n, kl, kr) = (first_line[0], first_line[1], first_line[2]);
        
        let cc: String = lines.next().unwrap();
        let mut zz = vec![0; n];
        
        let (mut l, mut r) = (0, 0);
        for i in 1..n {
            if i + zz[i - l] < r {
                zz[i] = zz[i - l];
            } else {
                l = i;
                r = max(r, i);
                while r < n && cc[r..].chars().next() == cc[(r - l)..].chars().next() {
                    r += 1;
                }
                zz[i] = r - l;
            }
        }

        let mut st = SegmentTree::new(n);
        st.build(&zz);
        
        let mut ans = vec![0; n + 1];
        for p in 1..=n {
            let mut k = 0;
            let mut i = 0;
            while i < n {
                i = st.query(i + p, p as i32);
                k += 1;
            }
            ans[k] = max(ans[k], p as i32);
        }
        
        for k in (1..=n-1).rev() {
            ans[k] = max(ans[k], ans[k + 1]);
        }
        
        let result: Vec<String> = (kl..=kr).map(|k| ans[k].to_string()).collect();
        println!("{}", result.join(" "));
    }
}