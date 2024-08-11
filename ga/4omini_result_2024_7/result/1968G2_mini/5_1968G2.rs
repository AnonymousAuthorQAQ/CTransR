use std::io::{self, BufRead};
use std::cmp;

const N: usize = 200000;
const N_: usize = 1 << 18; // N_ = pow2(ceil(log2(N)))

struct SegmentTree {
    st: [i32; N_ * 2],
}

impl SegmentTree {
    fn new() -> Self {
        SegmentTree { st: [0; N_ * 2] }
    }

    fn build(&mut self, aa: &[i32], n: usize) {
        let mut n_ = 1;
        while n_ < n {
            n_ <<= 1;
        }
        self.st[0..n_ * 2].fill(0);
        for i in 0..n {
            self.st[n_ + i] = aa[i];
        }
        for i in (1..n_).rev() {
            self.st[i] = cmp::max(self.st[i << 1], self.st[i << 1 | 1]);
        }
    }

    fn query(&self, l: usize, a: i32) -> usize {
        let mut l = l + N_;
        let mut r = N_ - 1;

        while l <= r {
            if l & 1 == 1 {
                if self.st[l] >= a {
                    let mut x = l;
                    while x < N_ {
                        x = if self.st[x << 1] >= a {
                            x << 1
                        } else {
                            x << 1 | 1
                        };
                    }
                    return x - N_;
                }
                l += 1;
            }
            l >>= 1;
            r >>= 1;
        }
        N_
    }
}

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut input = handle.lines().map(|line| line.unwrap());
    
    let t: usize = input.next().unwrap().parse().unwrap();
    
    for _ in 0..t {
        let first_line: Vec<usize> = input.next().unwrap().split_whitespace()
            .map(|x| x.parse().unwrap()).collect();
        let (n, kl, kr) = (first_line[0], first_line[1], first_line[2]);
        let cc: String = input.next().unwrap();
        
        let mut zz = vec![0; n];
        let mut ans = vec![0; n + 1];
        
        let mut l = 0;
        let mut r = 0;
        
        for i in 1..n {
            if i + zz[i - l] < r {
                zz[i] = zz[i - l];
            } else {
                l = i;
                r = cmp::max(r, i);
                while r < n && cc.as_bytes()[r] == cc.as_bytes()[r - l] {
                    r += 1;
                }
                zz[i] = r - l;
            }
        }

        let mut segment_tree = SegmentTree::new();
        segment_tree.build(&zz, n);
        
        for p in 1..=n {
            let mut k = 0;
            let mut i = 0;
            while i < n {
                i = segment_tree.query(i + p, p as i32);
                k += 1;
            }
            ans[k] = cmp::max(ans[k], p as i32);
        }

        for k in (1..n).rev() {
            ans[k] = cmp::max(ans[k], ans[k + 1]);
        }

        for k in kl..=kr {
            print!("{} ", ans[k]);
        }
        println!();
    }
}