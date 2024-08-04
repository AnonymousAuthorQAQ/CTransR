use std::io::{self, BufRead};

const N: usize = 200_000;
const N_: usize = 1 << 18; // N_ = pow2(ceil(log2(N)))

fn max(a: i32, b: i32) -> i32 {
    if a > b { a } else { b }
}

struct SegmentTree {
    st: Vec<i32>,
    n: usize,
}

impl SegmentTree {
    fn new(n: usize) -> Self {
        let mut n_ = 1;
        while n_ < n {
            n_ <<= 1;
        }
        SegmentTree {
            st: vec![0; n_ * 2],
            n: n_,
        }
    }

    fn build(&mut self, aa: &[i32], n: usize) {
        for i in 0..n {
            self.st[self.n + i] = aa[i];
        }
        for i in (1..self.n).rev() {
            self.st[i] = max(self.st[i * 2], self.st[i * 2 + 1]);
        }
    }

    fn query(&self, l: usize, a: i32) -> usize {
        let mut l = l + self.n;
        let mut r = self.n * 2 - 1;

        while l <= r {
            if l % 2 == 1 {
                if self.st[l] >= a {
                    let mut index = l;
                    while index < self.n {
                        index = if self.st[index * 2] >= a {
                            index * 2
                        } else {
                            index * 2 + 1
                        };
                    }
                    return index - self.n;
                }
                l += 1;
            }
            l >>= 1;
            r >>= 1;
        }
        self.n
    }
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    let t: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
    for _ in 0..t {
        let line = lines.next().unwrap().unwrap();
        let values: Vec<i32> = line.split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        
        let n = values[0] as usize;
        let kl = values[1];
        let kr = values[2];

        let cc = lines.next().unwrap().unwrap();
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

        let mut seg_tree = SegmentTree::new(n);
        seg_tree.build(&zz, n);

        let mut ans = vec![0; n + 1];
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

        let result: Vec<i32> = (kl..=kr).map(|k| ans[k as usize]).collect();
        println!("{}", result.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(" "));
    }
}