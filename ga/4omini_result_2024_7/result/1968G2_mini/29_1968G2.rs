use std::cmp::max;
use std::io::{self, BufRead};

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
        self.st[..self.n_ * 2].fill(0);
        for i in 0..n {
            self.st[self.n_ + i] = aa[i];
        }
        for i in (1..self.n_).rev() {
            self.st[i] = max(self.st[i << 1], self.st[i << 1 | 1]);
        }
    }

    fn query(&self, mut l: usize, a: i32) -> usize {
        let mut r = self.n_ - 1;
        let mut result = self.n_; // Default to n_ if not found

        let (mut l, mut r) = (l + self.n_, r + self.n_);
        while l <= r {
            if l & 1 == 1 {
                if self.st[l] >= a {
                    let mut temp = l;
                    while temp < self.n_ {
                        temp = if self.st[temp << 1] >= a {
                            temp << 1
                        } else {
                            temp << 1 | 1
                        };
                    }
                    result = temp - self.n_;
                    break;
                }
                l += 1;
            }
            l >>= 1;
            r >>= 1;
        }
        result
    }
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    let t: usize = lines.next().unwrap().unwrap().parse().unwrap();
    for _ in 0..t {
        let line = lines.next().unwrap().unwrap();
        let parts: Vec<&str> = line.split_whitespace().collect();
        let n: usize = parts[0].parse().unwrap();
        let kl: usize = parts[1].parse().unwrap();
        let kr: usize = parts[2].parse().unwrap();
        let cc: String = lines.next().unwrap().unwrap();

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

        let mut segment_tree = SegmentTree::new();
        segment_tree.build(&zz, n);

        let mut ans = vec![0; n + 1];

        for p in 1..=n {
            let mut k = 0;
            let mut i = 0;
            while i < n {
                i = segment_tree.query(i + p, p);
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