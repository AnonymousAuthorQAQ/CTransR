use std::io;

fn max(a: i32, b: i32) -> i32 {
    if a > b { a } else { b }
}

const N: usize = 200000;
const N_: usize = 1 << 18;  // N_ = pow2(ceil(log2(N)))

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
        self.st[..self.n_ * 2].fill(0);
        
        for i in 0..n {
            self.st[self.n_ + i] = aa[i];
        }
        for i in (1..self.n_).rev() {
            self.st[i] = max(self.st[i << 1], self.st[i << 1 | 1]);
        }
    }

    fn query(&self, l: usize, a: i32) -> usize {
        let mut r = self.n_ - 1;
        let mut l = l + self.n_;
        let mut r = r + self.n_;

        while l <= r {
            if l & 1 == 1 {
                if self.st[l] >= a {
                    let mut res = l;
                    while res < self.n_ {
                        res = if self.st[res << 1] >= a {
                            res << 1
                        } else {
                            res << 1 | 1
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
    let mut input = String::new();
    
    stdin.read_line(&mut input).expect("Failed to read line");
    let t: usize = input.trim().parse().expect("Failed to parse");

    for _ in 0..t {
        input.clear();
        stdin.read_line(&mut input).expect("Failed to read line");
        
        let parts: Vec<&str> = input.split_whitespace().collect();
        let n: usize = parts[0].parse().expect("Failed to parse");
        let kl: usize = parts[1].parse().expect("Failed to parse");
        let kr: usize = parts[2].parse().expect("Failed to parse");
        let cc: Vec<char> = parts[3].chars().collect();

        let mut zz = vec![0; n];
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

        let mut st = SegmentTree::new();
        st.build(&zz, n);

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