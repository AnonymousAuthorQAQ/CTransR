use std::io::{self, Read};
use std::cmp::max;

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
        self.st[(self.n_..self.n_ + n)].copy_from_slice(aa);
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
                    let mut l_clone = l;
                    while l_clone < self.n_ {
                        l_clone = if self.st[l_clone << 1] >= a {
                            l_clone << 1
                        } else {
                            l_clone << 1 | 1
                        };
                    }
                    return l_clone - self.n_;
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
    stdin.lock().read_to_string(&mut input).unwrap();
    let mut iter = input.split_whitespace();
    
    let t: usize = iter.next().unwrap().parse().unwrap();
    
    for _ in 0..t {
        let n: usize = iter.next().unwrap().parse().unwrap();
        let kl: usize = iter.next().unwrap().parse().unwrap();
        let kr: usize = iter.next().unwrap().parse().unwrap();
        let cc: String = iter.next().unwrap().to_string();
        
        let mut zz = vec![0; n];
        let mut ans = vec![0; n + 1];
        
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

        let mut seg_tree = SegmentTree::new(n);
        seg_tree.build(&zz, n);

        for p in 1..=n {
            let mut k = 0;
            let mut i = 0;
            while i < n {
                i = seg_tree.query(i + p, p);
                if i < n {
                    k += 1;
                }
            }
            ans[k] = max(ans[k], p);
        }

        for k in (1..=n).rev() {
            ans[k - 1] = max(ans[k - 1], ans[k]);
        }

        let result: Vec<String> = (kl..=kr).map(|k| ans[k].to_string()).collect();
        println!("{}", result.join(" "));
    }
}