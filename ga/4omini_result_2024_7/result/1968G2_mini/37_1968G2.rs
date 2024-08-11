use std::io::{self, BufRead};

fn max(a: usize, b: usize) -> usize {
    if a > b { a } else { b }
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
        let st = vec![0; n_ * 2];
        SegmentTree { st, n_ }
    }

    fn build(&mut self, aa: &[usize]) {
        let n = aa.len();
        for i in 0..n {
            self.st[self.n_ + i] = aa[i];
        }
        for i in (1..self.n_).rev() {
            self.st[i] = max(self.st[i << 1], self.st[i << 1 | 1]);
        }
    }

    fn query(&self, mut l: usize, mut a: usize) -> usize {
        let mut r = self.n_ - 1;
        let mut result = self.n_; // Default to n_ (out of bounds)

        while l <= r {
            if l & 1 == 1 {
                if self.st[l] >= a {
                    let mut temp_l = l;
                    while temp_l < self.n_ {
                        temp_l = if self.st[temp_l << 1] >= a {
                            temp_l << 1
                        } else {
                            temp_l << 1 | 1
                        }
                    }
                    result = temp_l - self.n_;
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
        let first_line = lines.next().unwrap().unwrap();
        let mut parts = first_line.split_whitespace();
        let n: usize = parts.next().unwrap().parse().unwrap();
        let kl: usize = parts.next().unwrap().parse().unwrap();
        let kr: usize = parts.next().unwrap().parse().unwrap();
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
                while r < n && cc.chars().nth(r) == cc.chars().nth(r - l) {
                    r += 1;
                }
                zz[i] = r - l;
            }
        }

        let mut segment_tree = SegmentTree::new(n);
        segment_tree.build(&zz);

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

        for k in (1..n).rev() {
            ans[k] = max(ans[k], ans[k + 1]);
        }

        for k in kl..=kr {
            print!("{} ", ans[k]);
        }
        println!();
    }
}