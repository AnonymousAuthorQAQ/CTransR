use std::io::{self, Read};
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

        l += self.n_;
        r += self.n_;

        while l <= r {
            if l & 1 == 1 {
                if self.st[l] >= a {
                    let mut temp = l;
                    while temp < self.n_ {
                        temp = if self.st[temp << 1] >= a {
                            temp << 1
                        } else {
                            temp << 1 | 1
                        }
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
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut lines = input.lines();
    
    let t: usize = lines.next().unwrap().parse().unwrap();
    
    for _ in 0..t {
        let line = lines.next().unwrap();
        let mut iter = line.split_whitespace();
        let n: usize = iter.next().unwrap().parse().unwrap();
        let kl: usize = iter.next().unwrap().parse().unwrap();
        let kr: usize = iter.next().unwrap().parse().unwrap();
        let cc: Vec<char> = iter.next().unwrap().chars().collect();
        
        let mut zz = vec![0; n];
        let mut ans = vec![0; n + 1];
        let (mut l, mut r) = (0, 0);
        
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
        
        let mut seg_tree = SegmentTree::new();
        seg_tree.build(&zz, n);
        
        for p in 1..=n {
            let mut k = 0;
            let mut i = 0;
            while i < n {
                i = seg_tree.query(i + p, p);
                k += 1;
            }
            ans[k] = max(ans[k], p);
        }

        for k in (1..n).rev() {
            ans[k] = max(ans[k], ans[k + 1]);
        }

        let result: Vec<String> = (kl..=kr).map(|k| ans[k].to_string()).collect();
        println!("{}", result.join(" "));
    }
}