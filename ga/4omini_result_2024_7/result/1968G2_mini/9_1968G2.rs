use std::io::{self, BufRead};
use std::usize;

const N: usize = 200_000;
const N_: usize = 1 << 18;

fn max(a: usize, b: usize) -> usize {
    if a > b {
        a
    } else {
        b
    }
}

struct SegmentTree {
    st: Vec<usize>,
    n_: usize,
}

impl SegmentTree {
    fn new(size: usize) -> Self {
        let mut n_ = 1;
        while n_ < size {
            n_ <<= 1;
        }
        SegmentTree {
            st: vec![0; n_ * 2],
            n_,
        }
    }
    
    fn build(&mut self, aa: &[usize], n: usize) {
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
                    return temp - self.n_;
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
    let t: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();

    for _ in 0..t {
        let line = lines.next().unwrap().unwrap();
        let parts: Vec<usize> = line.split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        let n = parts[0];
        let kl = parts[1];
        let kr = parts[2];
        
        let cc: Vec<char> = lines.next().unwrap().unwrap().trim().chars().collect();
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

        let mut segment_tree = SegmentTree::new(n);
        segment_tree.build(&zz, n);

        let mut ans = vec![0; n + 1];

        for p in 1..=n {
            let mut k = 0;
            let mut i = 0;
            while i < n {
                i = segment_tree.query(i + p, p);
                k += 1;
            }
            ans[k] = max(ans[k], p);
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