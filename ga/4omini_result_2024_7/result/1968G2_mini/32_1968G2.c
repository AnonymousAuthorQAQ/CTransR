use std::io::{self, BufRead};
use std::cmp;

const N: usize = 200000;
const N_: usize = 1 << 18; // N_ = pow2(ceil(log2(N)))

struct SegmentTree {
    st: [i32; N_ * 2],
    n_: usize,
}

impl SegmentTree {
    fn new() -> Self {
        SegmentTree {
            st: [0; N_ * 2],
            n_: 0,
        }
    }

    fn build(&mut self, aa: &[i32], n: usize) {
        self.n_ = 1;
        while self.n_ < n {
            self.n_ <<= 1;
        }
        self.st.fill(0);
        
        for i in 0..n {
            self.st[self.n_ + i] = aa[i];
        }
        for i in (1..self.n_).rev() {
            self.st[i] = cmp::max(self.st[i << 1], self.st[i << 1 | 1]);
        }
    }

    fn query(&self, mut l: usize, mut a: i32) -> usize {
        let mut r = self.n_ - 1;
        l += self.n_;
        r += self.n_;

        while l <= r {
            if (l & 1) == 1 {
                if self.st[l] >= a {
                    let mut res = l;
                    while res < self.n_ {
                        res = if self.st[res << 1] >= a { 
                            res << 1 
                        } else { 
                            res << 1 | 1 
                        }
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
    let reader = stdin.lock();
    let mut lines = reader.lines();

    let t: usize = lines.next().unwrap().unwrap().parse().unwrap();
    
    for _ in 0..t {
        let first_line: Vec<&str> = lines.next().unwrap().unwrap().split_whitespace().collect();
        let n: usize = first_line[0].parse().unwrap();
        let kl: usize = first_line[1].parse().unwrap();
        let kr: usize = first_line[2].parse().unwrap();
        let cc: Vec<char> = lines.next().unwrap().unwrap().chars().collect();

        let mut zz = vec![0; n];
        let mut ans = vec![0; n + 1];
        let (mut l, mut r) = (0, 0);
        
        for i in 1..n {
            if i + zz[i - l] < r {
                zz[i] = zz[i - l];
            } else {
                l = i;
                r = cmp::max(r, i);
                while r < n && cc[r] == cc[r - l] {
                    r += 1;
                }
                zz[i] = r - l;
            }
        }

        let mut st = SegmentTree::new();
        st.build(&zz, n);
        
        for p in 1..=n {
            let mut k = 0;
            let mut i = 0;

            while i < n {
                i = st.query(i + p, p);
                if i < n {
                    k += 1;
                }
            }

            ans[k] = cmp::max(ans[k], p);
        }

        // Fill in the maximum values backward
        for k in (1..n).rev() {
            ans[k] = cmp::max(ans[k], ans[k + 1]);
        }
        
        // Print results for the range [kl, kr]
        for k in kl..=kr {
            print!("{} ", ans[k]);
        }
        println!();
    }
}