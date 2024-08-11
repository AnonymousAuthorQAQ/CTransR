use std::io::{self, Read};
use std::cmp::max;

const N: usize = 200_000;
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
        self.st[..2 * self.n_].fill(0);
        self.st[self.n_..self.n_ + n].copy_from_slice(aa);
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
                    let mut res = l;
                    while res < self.n_ {
                        res = if self.st[res << 1] >= a {
                            res << 1
                        } else {
                            res << 1 | 1
                        };
                    }
                    return res - self.n_;
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
    let mut results = vec![];

    for _ in 0..t {
        let first_line: Vec<&str> = lines.next().unwrap().split_whitespace().collect();
        let n: usize = first_line[0].parse().unwrap();
        let kl: usize = first_line[1].parse().unwrap();
        let kr: usize = first_line[2].parse().unwrap();
        let cc: String = lines.next().unwrap().to_string();

        let mut zz = vec![0; n];
        let mut ans = vec![0; n + 1];

        // Compute the Z array
        let mut l = 0;
        let mut r = 0;
        for i in 1..n {
            if i + zz[i - l] < r {
                zz[i] = zz[i - l];
            } else {
                l = i;
                r = max(r, i);
                while r < n && cc.chars().nth(r).unwrap() == cc.chars().nth(r - l).unwrap() {
                    r += 1;
                }
                zz[i] = r - l;
            }
        }

        // Build segment tree
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

        for k in (1..(n + 1)).rev() {
            ans[k - 1] = max(ans[k - 1], ans[k]);
        }

        let result = (kl..=kr).map(|k| ans[k]).collect::<Vec<_>>();
        results.push(result);
    }

    // Print results
    for result in results {
        println!("{}", result.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(" "));
    }
}