use std::io::{self, Read};
use std::cmp;

const N: usize = 200000;
const N_: usize = 1 << 18;  // N_ = pow2(ceil(log2(N)))

fn max(a: i32, b: i32) -> i32 {
    if a > b { a } else { b }
}

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
        let st = vec![0; n_ * 2];  // Initialize segment tree
        SegmentTree { st, n_ }
    }

    fn build(&mut self, aa: &[i32], n: usize) {
        self.st[self.n_..self.n_ + n].copy_from_slice(aa);
        for i in (1..self.n_).rev() {
            self.st[i] = max(self.st[i * 2], self.st[i * 2 + 1]);
        }
    }

    fn query(&self, l: usize, a: i32) -> usize {
        let mut l = l + self.n_;
        let mut r = self.n_ - 1 + self.n_;

        while l <= r {
            if l % 2 == 1 {
                if self.st[l] >= a {
                    let mut pos = l;
                    while pos < self.n_ {
                        pos = if self.st[pos * 2] >= a {
                            pos * 2
                        } else {
                            pos * 2 + 1
                        };
                    }
                    return pos - self.n_;
                }
                l += 1;
            }
            l /= 2;
            r /= 2;
        }
        self.n_
    }
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut input = String::new();
    reader.read_to_string(&mut input).unwrap();
    let mut iter = input.split_whitespace();
    let t: usize = iter.next().unwrap().parse().unwrap();

    for _ in 0..t {
        let n: usize = iter.next().unwrap().parse().unwrap();
        let kl: usize = iter.next().unwrap().parse().unwrap();
        let kr: usize = iter.next().unwrap().parse().unwrap();
        let cc: String = iter.next().unwrap().to_string();
        
        let mut zz = vec![0; n];
        let (mut l, mut r) = (0, 0);
        
        for i in 1..n {
            if i + zz[i - l] < r {
                zz[i] = zz[i - l];
            } else {
                l = i;
                r = cmp::max(r, i);
                while r < n && cc[r..].chars().next().unwrap() == cc[r - l..].chars().next().unwrap() {
                    r += 1;
                }
                zz[i] = r - l;
            }
        }

        let mut seg_tree = SegmentTree::new(n);
        seg_tree.build(&zz.iter().map(|&x| x).collect::<Vec<i32>>(), n);

        let mut ans = vec![0; n + 1];
        
        for p in 1..=n {
            let mut k = 0;
            let mut i = 0;
            while i < n {
                i = seg_tree.query(i + p, p as i32);
                k += 1;
            }
            ans[k] = max(ans[k], p as i32);
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