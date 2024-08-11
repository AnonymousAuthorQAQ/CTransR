use std::io;
use std::io::prelude::*;
use std::cmp::max;

const N: usize = 200000;
const N_: usize = 1 << 18; // N_ = pow2(ceil(log2(N)))

fn max(a: usize, b: usize) -> usize {
    a.max(b)
}

#[derive(Clone)]
struct SegmentTree {
    tree: Vec<usize>,
    n: usize,
}

impl SegmentTree {
    fn new(n: usize) -> Self {
        let mut tree = vec![0; N_ * 2];
        tree
    }

    fn build(&mut self, arr: &[usize]) {
        self.n = 1;
        while self.n < arr.len() {
            self.n *= 2;
        }
        for i in 0..arr.len() {
            self.tree[self.n + i] = arr[i];
        }
        for i in (1..self.n).rev() {
            self.tree[i] = max(self.tree[i << 1], self.tree[i << 1 | 1]);
        }
    }

    fn query(&self, l: usize, a: usize) -> usize {
        let mut l = l + self.n;
        let mut r = self.n - 1 + self.n;
        while l <= r {
            if l & 1 == 1 {
                if self.tree[l] >= a {
                    while l < self.n {
                        l = if self.tree[l << 1] >= a { l << 1 } else { l << 1 | 1 };
                    }
                    return l - self.n;
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
    let stdout = io::stdout();
    let mut stdin = stdin.lock();
    let mut stdout = stdout.lock();

    let mut line = String::new();
    stdin.read_line(&mut line).expect("Failed to read line");
    let t: usize = line.trim().parse().expect("Failed to parse t");

    for _ in 0..t {
        line.clear();
        stdin.read_line(&mut line).expect("Failed to read line");
        let mut parts = line.trim().split_whitespace();
        let n: usize = parts.next().expect("Failed to parse n").parse().expect("Failed to parse n");
        let kl: usize = parts.next().expect("Failed to parse kl").parse().expect("Failed to parse kl");
        let kr: usize = parts.next().expect("Failed to parse kr").parse().expect("Failed to parse kr");
        let cc: String = parts.next().expect("Failed to parse cc").to_string();

        let mut zz = vec![0; n];
        let mut l = 0;
        let mut r = 0;
        for i in 1..n {
            if i + zz[i - l] < r {
                zz[i] = zz[i - l];
            } else {
                l = i;
                r = max(r, i);
                while r < n && cc.chars().nth(r).unwrap() == cc.chars().nth(r - l).unwrap() {
                    r += 1;
                }
                zz[i] = r - l;
            }
        }

        let mut st = SegmentTree::new(n);
        st.build(&zz);

        let mut ans = vec![0; n + 1];
        for p in 1..=n {
            let mut k = 0;
            let mut i = 0;
            while i < n {
                i = st.query(i + p, p);
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