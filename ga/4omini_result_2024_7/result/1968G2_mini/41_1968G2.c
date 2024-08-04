use std::cmp::max;
use std::io::{self, BufRead};

const N: usize = 200000;
const N_: usize = 1 << 18; // N_ = pow2(ceil(log2(N)))

struct SegmentTree {
    st: Vec<usize>,
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

    fn build(&mut self, aa: &[usize]) {
        let n = aa.len();
        for i in 0..n {
            self.st[self.n_ + i] = aa[i];
        }
        for i in (1..self.n_).rev() {
            self.st[i] = max(self.st[i * 2], self.st[i * 2 + 1]);
        }
    }

    fn query(&self, mut l: usize, a: usize) -> usize {
        let mut r = self.n_ - 1;
        let mut res = self.n_;

        l += self.n_;
        r += self.n_;

        while l <= r {
            if l % 2 == 1 {
                if self.st[l] >= a {
                    let mut current = l;
                    while current < self.n_ {
                        if self.st[current * 2] >= a {
                            current = current * 2;
                        } else {
                            current = current * 2 + 1;
                        }
                    }
                    res = current - self.n_;
                    break;
                }
                l += 1;
            }
            l >>= 1;
            r >>= 1;
        }

        res
    }
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    let mut t = String::new();
    reader.read_line(&mut t).unwrap();
    let t: usize = t.trim().parse().unwrap();

    for _ in 0..t {
        let mut input = String::new();
        reader.read_line(&mut input).unwrap();
        let parts: Vec<&str> = input.split_whitespace().collect();
        let n: usize = parts[0].parse().unwrap();
        let kl: usize = parts[1].parse().unwrap();
        let kr: usize = parts[2].parse().unwrap();
        
        let mut cc = String::new();
        reader.read_line(&mut cc).unwrap();
        let cc: Vec<char> = cc.trim().chars().collect();

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
                let next_i = seg_tree.query(i + p, p);
                if next_i == n {
                    break;
                }
                k += 1;
                i = next_i;
            }
            ans[k] = max(ans[k], p);
        }

        // Make sure we have the maximum lengths for each count
        for k in (1..n).rev() {
            ans[k] = max(ans[k], ans[k + 1]);
        }

        for k in kl..=kr {
            print!("{} ", ans[k]);
        }
        println!();
    }
}