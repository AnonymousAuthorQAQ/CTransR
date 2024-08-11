use std::io::{self, BufRead};
use std::cmp::max;

const N: usize = 200000;
const N_: usize = 1 << 18;

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
        
        Self {
            st: vec![0; n_ * 2],
            n_,
        }
    }

    fn build(&mut self, aa: &[i32]) {
        self.st[..self.n_].fill(0);
        for (i, &value) in aa.iter().enumerate() {
            self.st[self.n_ + i] = value;
        }
        for i in (1..self.n_).rev() {
            self.st[i] = max(self.st[i << 1], self.st[i << 1 | 1]);
        }
    }

    fn query(&self, mut l: usize, mut a: i32) -> usize {
        let mut r = self.n_ - 1;
        l += self.n_;
        r += self.n_;
        while l <= r {
            if l & 1 == 1 {
                if self.st[l] >= a {
                    let mut cur = l;
                    while cur < self.n_ {
                        cur = if self.st[cur << 1] >= a {
                            cur << 1
                        } else {
                            cur << 1 | 1
                        };
                    }
                    return cur - self.n_;
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

    let input = reader.lines().next().unwrap().unwrap();
    let mut iter = input.split_whitespace();
    let t: usize = iter.next().unwrap().parse().unwrap();

    for _ in 0..t {
        let n: usize = iter.next().unwrap().parse().unwrap();
        let kl: usize = iter.next().unwrap().parse().unwrap();
        let kr: usize = iter.next().unwrap().parse().unwrap();
        let cc: Vec<char> = iter.next().unwrap().chars().collect();

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

        let mut seg_tree = SegmentTree::new(n);
        seg_tree.build(&zz);

        let mut ans = vec![0; n + 1];

        for p in 1..=n {
            let mut k = 0;
            let mut i = 0;
            
            while i < n {
                k += 1;
                i = seg_tree.query(i + p, p);
            }
            ans[k] = max(ans[k], p as i32);
        }

        for k in (1..n).rev() {
            ans[k] = max(ans[k], ans[k + 1]);
        }

        let result: Vec<_> = (kl..=kr).map(|k| ans[k]).collect();
        println!("{}", result.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(" "));
    }
}