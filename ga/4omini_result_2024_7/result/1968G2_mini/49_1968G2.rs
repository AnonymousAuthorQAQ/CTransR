use std::io::{self, BufRead};
use std::cmp::max;

const N: usize = 200000;
const N_: usize = 1 << 18; // N_ = pow2(ceil(log2(N))) which is 262144

struct SegmentTree {
    st: Vec<i32>,
    n_: usize,
}

impl SegmentTree {
    fn new(n: usize) -> Self {
        let n_ = (1..).find(|x| *x >= n).unwrap();
        let mut st = vec![0; n_ * 2];
        SegmentTree { st, n_ }
    }
    
    fn build(&mut self, aa: &[i32]) {
        let n = aa.len();
        
        self.st[N_: usize - 1..N_: usize - 1 + n].copy_from_slice(aa);

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
    let iter = reader.lines().filter_map(Result::ok);
    let mut input_iter = iter.take(1);
    
    let t: usize = input_iter.next().unwrap().parse().unwrap();

    for _ in 0..t {
        let line = input_iter.next().unwrap();
        let parts: Vec<_> = line.split_whitespace().collect();
        
        let (n, kl, kr): (usize, usize, usize) = (
            parts[0].parse().unwrap(),
            parts[1].parse().unwrap(),
            parts[2].parse().unwrap(),
        );
        
        let cc: &str = parts[3];
        
        let mut zz = vec![0; n];
        let mut ans = vec![0; n + 1];

        for i in 1 {
                l = 0;
                r = 0;
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
        
        let mut seg_tree = SegmentTree::new(n);
        seg_tree.build(&zz);
        
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
        
        print!("{} ", ans[kl..=kr].iter().cloned().collect::<Vec<_>>().join(" "));
        println!();
    }
}