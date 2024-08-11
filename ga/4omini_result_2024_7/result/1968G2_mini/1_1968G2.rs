use std::cmp::max;
use std::io::{self, BufRead};

const N: usize = 200000;
const N_: usize = 1 << 18; // N_ = pow2(ceil(log2(N)))

struct SegmentTree {
    st: Vec<i32>,
    n_: usize,
}

impl SegmentTree {
    fn new(aa: &[i32], n: usize) -> SegmentTree {
        let mut n_ = 1;
        while n_ < n {
            n_ <<= 1;
        }

        let mut st = vec![0; n_ * 2];
        for i in 0..n {
            st[n_ + i] = aa[i];
        }
        for i in (1..n_).rev() {
            st[i] = max(st[i << 1], st[i << 1 | 1]);
        }

        SegmentTree { st, n_ }
    }

    fn query(&self, mut l: usize, a: i32) -> usize {
        let mut r = self.n_ - 1;

        l += self.n_;
        r += self.n_;

        while l <= r {
            if l & 1 == 1 {
                if self.st[l] >= a {
                    let mut pos = l;
                    while pos < self.n_ {
                        pos = if self.st[pos << 1] >= a {
                            pos << 1
                        } else {
                            pos << 1 | 1
                        };
                    }
                    return pos - self.n_;
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
    let handle = stdin.lock();
    let reader = handle.lines();

    let mut first_line = true;
    for line in reader {
        if let Ok(input) = line {
            let values: Vec<&str> = input.split_whitespace().collect();

            if first_line {
                let t: usize = values[0].parse().unwrap();
                first_line = false;

                for _ in 0..t {
                    let n: usize = values[0].parse().unwrap();
                    let kl: usize = values[1].parse().unwrap();
                    let kr: usize = values[2].parse().unwrap();
                    let cc: String = values[3].to_string();

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
                    
                    let st = SegmentTree::new(&zz, n);
                    let mut ans = vec![0; n + 1];

                    for p in 1..=n {
                        let mut k = 0;
                        let mut i = 0;

                        while i < n {
                            i = st.query(i + p, p);
                            k += 1;
                        }
                        ans[k] = max(ans[k], p as i32);
                    }

                    for k in (1..=n-1).rev() {
                        ans[k] = max(ans[k], ans[k + 1]);
                    }

                    for k in kl..=kr {
                        print!("{} ", ans[k]);
                    }
                    println!();
                }
            }
        }
    }
}