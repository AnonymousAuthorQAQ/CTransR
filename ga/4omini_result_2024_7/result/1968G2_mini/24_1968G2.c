use std::io::{self, BufRead};

const N: usize = 200000;
const N_: usize = 1 << 18; // 262144

fn max(a: i32, b: i32) -> i32 {
    if a > b { a } else { b }
}

struct SegmentTree {
    st: Vec<i32>,
    n_: usize,
}

impl SegmentTree {
    fn new() -> Self {
        SegmentTree {
            st: vec![0; N_ * 2],
            n_: 0,
        }
    }

    fn build(&mut self, aa: &[i32], n: usize) {
        self.n_ = 1;
        while self.n_ < n {
            self.n_ <<= 1;
        }
        self.st[..(self.n_ * 2)].fill(0); // equivalent to memset in C

        for i in 0..n {
            self.st[self.n_ + i] = aa[i];
        }
        for i in (1..self.n_).rev() {
            self.st[i] = max(self.st[i * 2], self.st[i * 2 + 1]);
        }
    }

    fn query(&self, l: usize, a: i32) -> usize {
        let mut r = self.n_ - 1;
        let mut l = l + self.n_;
        let mut r = r + self.n_;

        while l <= r {
            if (l & 1) == 1 {
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
            l >>= 1;
            r >>= 1;
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
        let params: Vec<&str> = lines.next().unwrap().unwrap().split_whitespace().collect();
        let n: usize = params[0].parse().unwrap();
        let kl: usize = params[1].parse().unwrap();
        let kr: usize = params[2].parse().unwrap();
        let cc: &str = params[3];

        let mut zz: Vec<i32> = vec![0; n];
        let (mut l, mut r) = (0, 0);

        for i in 1..n {
            if i + zz[i - l as usize] < r {
                zz[i] = zz[i - l as usize];
            } else {
                l = i;
                r = max(r as usize, i);
                while r < n && cc.as_bytes()[r] == cc.as_bytes()[r - l] {
                    r += 1;
                }
                zz[i] = (r - l) as i32;
            }
        }

        let mut st = SegmentTree::new();
        st.build(&zz, n);

        let mut ans: Vec<i32> = vec![0; n + 1];

        for p in 1..=n {
            let mut k = 0;
            let mut i = 0;
            while i < n {
                i = st.query(i + p, p as i32);
                k += 1;
            }
            ans[k as usize] = max(ans[k as usize], p as i32);
        }

        for k in (1..n).rev() {
            ans[k as usize] = max(ans[k as usize], ans[(k + 1) as usize]);
        }

        for k in kl..=kr {
            print!("{} ", ans[k]);
        }
        println!();
    }
}