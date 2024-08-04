use std::io::{self, BufRead};

const N: usize = 200000;
const N_: usize = 1 << 18; // N_ = pow2(ceil(log2(N)))

fn max(a: usize, b: usize) -> usize {
    if a > b { a } else { b }
}

struct SegmentTree {
    st: [usize; N_ * 2],
    n_: usize,
}

impl SegmentTree {
    fn new() -> SegmentTree {
        SegmentTree {
            st: [0; N_ * 2],
            n_: 0,
        }
    }

    fn build(&mut self, aa: &[usize], n: usize) {
        self.n_ = 1;
        while self.n_ < n {
            self.n_ <<= 1;
        }
        self.st[..2 * self.n_].fill(0);
        self.st[self.n_..self.n_ + n].copy_from_slice(aa);
        for i in (1..self.n_).rev() {
            self.st[i] = max(self.st[i * 2], self.st[i * 2 + 1]);
        }
    }

    fn query(&self, l: usize, a: usize) -> usize {
        let mut l = l + self.n_;
        let mut r = self.n_ - 1 + self.n_;

        while l <= r {
            if l & 1 == 1 {
                if self.st[l] >= a {
                    let mut res = l;
                    while res < self.n_ {
                        res = if self.st[res * 2] >= a {
                            res * 2
                        } else {
                            res * 2 + 1
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
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();
    
    let t: usize = lines.next().unwrap().unwrap().parse().unwrap();
    
    for _ in 0..t {
        let first_line = lines.next().unwrap().unwrap();
        let parts: Vec<&str> = first_line.split_whitespace().collect();
        let n: usize = parts[0].parse().unwrap();
        let kl: usize = parts[1].parse().unwrap();
        let kr: usize = parts[2].parse().unwrap();
        let cc: String = lines.next().unwrap().unwrap();

        let mut zz = vec![0; n];
        let mut ans = vec![0; n + 1];
        
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

        let mut segment_tree = SegmentTree::new();
        segment_tree.build(&zz, n);

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

        let output: Vec<String> = (kl..=kr).map(|k| ans[k].to_string()).collect();
        println!("{}", output.join(" "));
    }
}