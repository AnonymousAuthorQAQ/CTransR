use std::cmp::max;
use std::io::{self, Write};

const N: usize = 200000;
const N_: usize = 1 << 18; // N_ = pow2(ceil(log2(N)))

struct SegmentTree {
    st: Vec<i32>,
    n_: usize,
}

impl SegmentTree {
    fn new(n: usize) -> Self {
        let mut n_ = 1;
        while n_ < n {
            n_ <<= 1;
        }
        let st = vec![0; n_ * 2];
        SegmentTree { st, n_ }
    }

    fn build(&mut self, aa: &[i32], n: usize) {
        for (i, &val) in aa.iter().enumerate() {
            self.st[self.n_ + i] = val;
        }
        for i in (1..self.n_).rev() {
            self.st[i] = max(self.st[i << 1], self.st[i << 1 | 1]);
        }
    }

    fn query(&self, mut l: usize, a: i32) -> usize {
        let mut r = self.n_ - 1;
        let mut l = l + self.n_;
        let mut r = r + self.n_;
        
        while l <= r {
            if l & 1 == 1 {
                if self.st[l] >= a {
                    let mut temp_l = l;
                    while temp_l < self.n_ {
                        temp_l = if self.st[temp_l << 1] >= a {
                            temp_l << 1
                        } else {
                            temp_l << 1 | 1
                        };
                    }
                    return temp_l - self.n_;
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
    let mut input = String::new();
    
    stdin.read_line(&mut input).unwrap();
    let mut iter = input.split_whitespace();
    let t: usize = iter.next().unwrap().parse().unwrap();

    for _ in 0..t {
        let n: usize;
        let kl: usize;
        let kr: usize;
        let cc: Vec<char>;

        input.clear();
        stdin.read_line(&mut input).unwrap();
        let tokens: Vec<&str> = input.split_whitespace().collect();
        
        n = tokens[0].parse().unwrap();
        kl = tokens[1].parse().unwrap();
        kr = tokens[2].parse().unwrap();
        cc = tokens[3].chars().collect();

        let mut zz = vec![0; n];
        
        let mut l = 0usize;
        let mut r = 0usize;

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
        seg_tree.build(&zz, n);

        let mut ans = vec![0; n + 1];

        for p in 1..=n {
            let mut k = 0;
            let mut i = 0;

            while i < n {
                i = seg_tree.query(i + p, p);
                k += 1;
            }
            ans[k] = max(ans[k], p);
        }

        for k in (1..=n - 1).rev() {
            ans[k] = max(ans[k], ans[k + 1]);
        }

        let res: Vec<String> = (kl..=kr).map(|k| ans[k].to_string()).collect();
        println!("{}", res.join(" "));
    }
}