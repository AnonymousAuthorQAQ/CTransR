use std::io::{self, BufRead};

const N: usize = 200000;

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
        SegmentTree {
            st: vec![0; n_ * 2],
            n_,
        }
    }

    fn build(&mut self, aa: &[i32], n: usize) {
        self.st[n..n + aa.len()].copy_from_slice(aa);
        for i in (1..self.n_).rev() {
            self.st[i] = self.st[i * 2].max(self.st[i * 2 + 1]);
        }
    }

    fn query(&self, mut l: usize, a: i32) -> usize {
        let mut r = self.n_ - 1; // inclusive
        let mut left = l + self.n_;
        let mut right = r + self.n_;

        while left <= right {
            if left % 2 == 1 {
                if self.st[left] >= a {
                    let mut current = left;
                    while current < self.n_ {
                        current = if self.st[current * 2] >= a { current * 2 } else { current * 2 + 1 };
                    }
                    return current - self.n_;
                }
                left += 1;
            }
            left /= 2;
            right /= 2;
        }
        self.n_
    }
}

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let reader = handle.lines().map(|line| line.unwrap());

    let mut iterator = reader.skip(1);
    let queries: Vec<i32> = iterator.map(|line| {
        let parts: Vec<i32> = line.split_whitespace().map(|x| x.parse().unwrap()).collect();
        let n = parts[0] as usize;
        let kl = parts[1];
        let kr = parts[2];
        let cc: Vec<_> = line.chars().collect();
        
        let mut zz = vec![0; n];
        let (mut l, mut r) = (0, 0);

        for i in 1..n {
            if i + zz[i - l] < r {
                zz[i] = zz[i - l];
            } else {
                l = i;
                r = l;
                while r < n && cc[r] == cc[r - l] {
                    r += 1;
                }
                zz[i] = r - l;
            }
        }

        let mut st = SegmentTree::new(n);
        st.build(&zz, n);

        let mut ans = vec![0; n + 1];
        for p in 1..=n {
            let mut k = 0;
            let mut i = 0;
            while i < n {
                i = st.query(i + p, p);
                if i < n {
                    k += 1;
                }
            }
            ans[k] = ans[k].max(p);
        }

        for k in (1..n).rev() {
            ans[k] = ans[k].max(ans[k + 1]);
        }

        let result: Vec<i32> = (kl..=kr).map(|k| ans[k as usize]).collect();
        result.iter().fold(String::new(), |mut acc, &x| {
            acc.push_str(&format!("{} ", x));
            acc
        })
    }).collect();

    for result in queries {
        println!("{}", result);
    }
}