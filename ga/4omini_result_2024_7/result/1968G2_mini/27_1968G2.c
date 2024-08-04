use std::io::{self, Read};
use std::cmp::max;

const N: usize = 200000;
const N_: usize = 1 << 18; // N_ = pow2(ceil(log2(N)))

struct SegmentTree {
    st: [i32; N_ * 2],
    n_: usize,
}

impl SegmentTree {
    fn new() -> Self {
        SegmentTree {
            st: [0; N_ * 2],
            n_: 1,
        }
    }

    fn build(&mut self, aa: &[i32], n: usize) {
        self.n_ = 1;
        while self.n_ < n {
            self.n_ <<= 1;
        }
        self.st.iter_mut().for_each(|x| *x = 0);
        for i in 0..n {
            self.st[self.n_ + i] = aa[i];
        }
        for i in (1..self.n_).rev() {
            self.st[i] = max(self.st[i << 1], self.st[i << 1 | 1]);
        }
    }

    fn query(&self, mut l: usize, mut a: i32) -> usize {
        let mut r = self.n_ - 1;
        l += self.n_;
        r += self.n_;

        while l <= r {
            if (l & 1) == 1 {
                if self.st[l] >= a {
                    let mut result = l;
                    while result < self.n_ {
                        result = if self.st[result << 1] >= a {
                            result << 1
                        } else {
                            result << 1 | 1
                        };
                    }
                    return result - self.n_;
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
    stdin.read_to_string(&mut input).unwrap();

    let mut lines = input.lines();
    let t: usize = lines.next().unwrap().parse().unwrap();

    for _ in 0..t {
        let first_line = lines.next().unwrap();
        let mut parts = first_line.split_whitespace();
        let n: usize = parts.next().unwrap().parse().unwrap();
        let kl: usize = parts.next().unwrap().parse().unwrap();
        let kr: usize = parts.next().unwrap().parse().unwrap();
        let cc: Vec<char> = lines.next().unwrap().chars().collect();

        let mut zz = vec![0; n];
        let mut ans = vec![0; n + 1];
        let mut l = 0;
        let mut r = 0;
        
        // Calculate Z-values
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

        let mut seg_tree = SegmentTree::new();
        seg_tree.build(&zz.iter().map(|&x| x as i32).collect::<Vec<_>>(), n);
        
        for p in 1..=n {
            let mut k = 0;
            let mut i = 0;
            while i < n {
                i = seg_tree.query(i + p, p);
                k += 1;
            }
            ans[k] = max(ans[k], p);
        }

        for k in (1..n).rev() {
            ans[k] = max(ans[k], ans[k + 1]);
        }

        let result: Vec<i32> = (kl..=kr).map(|k| ans[k]).collect();
        println!("{}", result.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(" "));
    }
}