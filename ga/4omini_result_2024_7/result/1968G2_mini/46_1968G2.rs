use std::cmp::max;
use std::io::{self, BufRead};
use std::vec::Vec;

const N: usize = 200000;
const N_: usize = 1 << 18; // N_ = pow2(ceil(log2(N)))

struct SegmentTree {
    st: Vec<usize>,
    n_: usize,
}

impl SegmentTree {
    fn new() -> Self {
        SegmentTree {
            st: vec![0; N_ * 2],
            n_: 1,
        }
    }

    fn build(&mut self, aa: &[usize], n: usize) {
        while self.n_ < n {
            self.n_ <<= 1;
        }
        self.st[0..self.n_ * 2].fill(0); // Initialize segment tree
        for i in 0..n {
            self.st[self.n_ + i] = aa[i];
        }
        for i in (1..self.n_).rev() {
            self.st[i] = max(self.st[i << 1], self.st[i << 1 | 1]);
        }
    }

    fn query(&self, l: usize, a: usize) -> usize {
        let mut l = l + self.n_;
        let mut r = self.n_ - 1 + self.n_;
        
        // Scan for the desired segment
        while l <= r {
            if l & 1 == 1 {
                if self.st[l] >= a {
                    let mut res = l;
                    while res < self.n_ {
                        res = if self.st[res << 1] >= a {
                            res << 1
                        } else {
                            res << 1 | 1
                        };
                    }
                    return res - self.n_;
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
    let reader = handle.lines();

    let mut input = reader.map(|line| line.unwrap()).collect::<Vec<_>>();
    let t: usize = input.remove(0).parse().unwrap();

    for _ in 0..t {
        let first_line: Vec<usize> = input.remove(0)
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        let (n, kl, kr) = (first_line[0], first_line[1], first_line[2]);
        let cc: Vec<char> = input.remove(0).chars().collect();

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

        let mut st = SegmentTree::new();
        st.build(&zz, n);

        let mut ans = vec![0; n + 1];

        for p in 1..=n {
            let mut k = 0;
            let mut i = 0;

            while i < n {
                k += 1;
                i = st.query(i + p, p);
            }

            ans[k] = max(ans[k], p);
        }

        for k in (1..n).rev() {
            ans[k] = max(ans[k], ans[k + 1]);
        }

        let result: Vec<usize> = (kl..=kr).map(|k| ans[k]).collect();
        println!("{}", result.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(" "));
    }
}