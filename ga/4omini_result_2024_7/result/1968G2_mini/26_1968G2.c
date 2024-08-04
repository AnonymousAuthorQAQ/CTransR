use std::io::{self, BufRead};

const N: usize = 200_000;
const N_: usize = 1 << 18; // N_ = pow2(ceil(log2(N)))

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
        Self {
            st: vec![0; n_ * 2],
            n_,
        }
    }

    fn build(&mut self, aa: &[i32], n: usize) {
        for (i, &value) in aa.iter().enumerate() {
            self.st[self.n_ + i] = value;
        }
        for i in (1..self.n_).rev() {
            self.st[i] = max(self.st[i << 1], self.st[i << 1 | 1]);
        }
    }

    fn query(&self, l: usize, a: i32) -> usize {
        let mut l = l + self.n_;
        let mut r = self.n_ - 1;

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
    let reader = stdin.lock();
    let lines: Vec<String> = reader.lines().filter_map(Result::ok).collect();

    let mut iter = lines.iter();
    let t: usize = iter.next().unwrap().parse().unwrap();

    for _ in 0..t {
        let first_line: Vec<&str> = iter.next().unwrap().split_whitespace().collect();
        let n: usize = first_line[0].parse().unwrap();
        let kl: usize = first_line[1].parse().unwrap();
        let kr: usize = first_line[2].parse().unwrap();
        let cc: Vec<char> = first_line[3].chars().collect();

        let mut zz = vec![0; n];
        let mut ans = vec![0; n + 1];

        let (mut l, mut r) = (0, 0);
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

        let mut st = SegmentTree::new(n);
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
            ans[k] = max(ans[k], p as i32);
        }

        for k in (1..n).rev() {
            ans[k] = max(ans[k], ans[k + 1]);
        }

        let result: Vec<String> = (kl..=kr).map(|k| ans[k] as usize).map(|a| a.to_string()).collect();
        println!("{}", result.join(" "));
    }
}