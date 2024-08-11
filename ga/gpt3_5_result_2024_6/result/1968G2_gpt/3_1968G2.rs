use std::cmp;

const N: usize = 200000;
const N_: usize = 1 << 18; // N_ = pow2(ceil(log2(N)))

struct SegTree {
    st: [i32; N_ * 2],
    n_: usize,
}

impl SegTree {
    fn new() -> SegTree {
        SegTree {
            st: [0; N_ * 2],
            n_: 0,
        }
    }

    fn max(a: i32, b: i32) -> i32 {
        if a > b { a } else { b }
    }

    fn build(&mut self, aa: &[i32], n: usize) {
        let mut i: usize;

        self.n_ = 1;
        while self.n_ < n {
            self.n_ <<= 1;
        }
        self.st.iter_mut().for_each(|val| *val = 0);
        for i in 0..n {
            self.st[self.n_ + i] = aa[i];
        }
        for i in (1..self.n_).rev() {
            self.st[i] = SegTree::max(self.st[i << 1], self.st[i << 1 | 1]);
        }
    }

    fn query(&self, l: usize, a: i32) -> usize {
        let mut r = self.n_ - 1;
        let mut l = l + self.n_;

        for (mut l, mut r) in (l..=r).zip((r..=r)) {
            while l <= r {
                if l & 1 == 1 {
                    if self.st[l] >= a {
                        let mut pos = l;
                        while pos < self.n_ {
                            pos = if self.st[pos << 1] >= a { pos << 1 } else { pos << 1 | 1 };
                        }
                        return pos - self.n_;
                    }
                    l += 1;
                }
                l >>= 1;
                r >>= 1;
            }
        }
        self.n_
    }
}

fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let t: usize = input.trim().parse().unwrap();

    let mut st = SegTree::new();

    for _ in 0..t {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let parts: Vec<&str> = input.split_whitespace().collect();
        let n: usize = parts[0].parse().unwrap();
        let kl: usize = parts[1].parse().unwrap();
        let kr: usize = parts[2].parse().unwrap();
        let cc = parts[3].trim();

        let mut zz = vec![0; n];
        let mut l = 0;
        let mut r = 0;

        for i in 1..n {
            if i + zz[i - l] < r {
                zz[i] = zz[i - l];
            } else {
                l = i;
                r = cmp::max(r, i);
                while r < n && cc.chars().nth(r - l) == cc.chars().nth(r) {
                    r += 1;
                }
                zz[i] = (r - l) as i32;
            }
        }

        st.build(&zz, n);

        let mut ans = vec![0; n + 1];
        for p in 1..=n {
            let mut k = 0;
            let mut i = 0;
            loop {
                i = st.query(i + p, p);
                if i < n {
                    k += 1;
                } else {
                    break;
                }
            }
            ans[k as usize] = cmp::max(ans[k as usize], p);
        }

        for k in (1..=n - 1).rev() {
            ans[k] = cmp::max(ans[k], ans[k + 1]);
        }

        for k in kl..=kr {
            print!("{} ", ans[k]);
        }
        println!();
    }
}