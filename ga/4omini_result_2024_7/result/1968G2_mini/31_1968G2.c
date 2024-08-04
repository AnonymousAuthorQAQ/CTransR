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
        let st = vec![0; 2 * n_];
        SegmentTree { st, n_ }
    }

    fn build(&mut self, aa: &[usize], n: usize) {
        for i in 0..n {
            self.st[self.n_ + i] = aa[i];
        }
        for i in (1..self.n_).rev() {
            self.st[i] = max(self.st[2 * i], self.st[2 * i + 1]);
        }
    }

    fn query(&mut self, l: usize, a: usize) -> usize {
        let mut r = self.n_ - 1;

        let mut l_idx = l + self.n_;
        let mut r_idx = r + self.n_;

        while l_idx <= r_idx {
            if l_idx % 2 == 1 {
                if self.st[l_idx] >= a {
                    while l_idx < self.n_ {
                        l_idx = if self.st[2 * l_idx] >= a {
                            2 * l_idx
                        } else {
                            2 * l_idx + 1
                        };
                    }
                    return l_idx - self.n_;
                }
                l_idx += 1;
            }
            l_idx /= 2;
            r_idx /= 2;
        }
        self.n_
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
                while r < n && cc.as_bytes()[r] == cc.as_bytes()[r - l] {
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
                if i < n {
                    k += 1;
                }
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