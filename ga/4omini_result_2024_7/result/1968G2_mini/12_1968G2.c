use std::io::{self, BufRead};

const N: usize = 200000;
const N_: usize = 1 << 18; // N_ = pow2(ceil(log2(N)))

fn max(a: usize, b: usize) -> usize {
    if a > b { a } else { b }
}

struct SegmentTree {
    st: Vec<usize>,
    n_: usize,
}

impl SegmentTree {
    fn new() -> Self {
        let st = vec![0; N_ * 2];
        SegmentTree { st, n_: 1 }
    }

    fn build(&mut self, aa: &[usize], n: usize) {
        self.n_ = 1;
        while self.n_ < n {
            self.n_ <<= 1;
        }
        
        // Initialize segment tree
        self.st.iter_mut().for_each(|x| *x = 0);
        for i in 0..n {
            self.st[self.n_ + i] = aa[i];
        }
        for i in (1..self.n_).rev() {
            self.st[i] = max(self.st[i << 1], self.st[i << 1 | 1]);
        }
    }

    fn query(&self, l: usize, a: usize) -> usize {
        let mut l = l + self.n_;
        let mut r = self.n_ - 1 + self.n_;
        
        while l <= r {
            if l & 1 == 1 {
                if self.st[l] >= a {
                    let mut index = l;
                    while index < self.n_ {
                        index = if self.st[index << 1] >= a {
                            index << 1
                        } else {
                            index << 1 | 1
                        };
                    }
                    return index - self.n_;
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
    
    let t: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
    
    for _ in 0..t {
        let first_line: Vec<usize> = lines.next().unwrap().unwrap().trim().split_whitespace()
            .map(|x| x.parse().unwrap()).collect();
        let (n, kl, kr) = (first_line[0], first_line[1], first_line[2]);
        let cc: Vec<char> = lines.next().unwrap().unwrap().trim().chars().collect();
    
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

        let mut seg_tree = SegmentTree::new();
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

            if k < ans.len() {
                ans[k] = max(ans[k], p);
            }
        }

        for k in (1..n).rev() {
            ans[k] = max(ans[k], ans[k + 1]);
        }

        let result: Vec<usize> = ans[kl..=kr].to_vec();
        println!("{}", result.iter().map(|&x| x.to_string()).collect::<Vec<String>>().join(" "));
    }
}