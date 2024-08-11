use std::io::{self, BufRead};

const N: usize = 200000;
const N_: usize = 1 << 18; // N_ = pow2(ceil(log2(N)))

fn max(a: i32, b: i32) -> i32 {
    if a > b { a } else { b }
}

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
        self.st[..self.n_ * 2].fill(0);
        for i in 0..n {
            self.st[self.n_ + i] = aa[i];
        }
        for i in (1..self.n_).rev() {
            self.st[i] = max(self.st[i << 1], self.st[i << 1 | 1]);
        }
    }

    fn query(&self, l: usize, a: i32) -> usize {
        let mut l = l + self.n_;
        let mut r = self.n_ - 1 + self.n_;
        while l <= r {
            if (l & 1) == 1 {
                if self.st[l] >= a {
                    let mut original_l = l;
                    while original_l < self.n_ {
                        original_l = if self.st[original_l << 1] >= a {
                            original_l << 1
                        } else {
                            original_l << 1 | 1
                        };
                    }
                    return original_l - self.n_;
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

    let mut line = String::new();
    reader.read_line(&mut line).unwrap();
    let t: i32 = line.trim().parse().unwrap();

    for _ in 0..t {
        line.clear();
        reader.read_line(&mut line).unwrap();
        let parts: Vec<&str> = line.trim().split_whitespace().collect();
        let n: usize = parts[0].parse().unwrap();
        let kl: usize = parts[1].parse().unwrap();
        let kr: usize = parts[2].parse().unwrap();
        let cc: String = parts[3].to_string();

        let mut zz = vec![0; n];
        let mut ans = vec![0; n + 1];

        let (mut l, mut r) = (0, 0);
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

        let mut st = SegmentTree::new();
        st.build(&zz, n);

        for p in 1..=n {
            let mut k = 0;
            let mut i = 0;
            while i < n {
                i = st.query(i + p, p);
                if i < n {
                    k += 1;
                }
            }
            ans[k] = max(ans[k], p);
        }

        for k in (1..n).rev() {
            ans[k] = max(ans[k], ans[k + 1]);
        }

        let result: Vec<_> = (kl..=kr).map(|k| ans[k]).collect();
        println!("{}", result.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(" "));
    }
}