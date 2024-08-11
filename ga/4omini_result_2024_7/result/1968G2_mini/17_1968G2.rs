use std::io::{self, Read};
use std::cmp::max;

const N: usize = 200000;
const N_: usize = 1 << 18; // N_ = pow2(ceil(log2(N)))

struct SegmentTree {
    st: [i32; N_ * 2],
    n: usize,
}

impl SegmentTree {
    fn new() -> Self {
        SegmentTree {
            st: [0; N_ * 2],
            n: 0,
        }
    }

    fn build(&mut self, aa: &[i32]) {
        let n = aa.len();
        self.n = 1;
        while self.n < n {
            self.n <<= 1;
        }
        self.st[0..self.n * 2].fill(0);
        for i in 0..n {
            self.st[self.n + i] = aa[i];
        }
        for i in (1..self.n).rev() {
            self.st[i] = max(self.st[i << 1], self.st[i << 1 | 1]);
        }
    }

    fn query(&self, mut l: usize, a: i32) -> usize {
        let mut r = self.n - 1;
        let mut l = l + self.n;
        let mut r = r + self.n;

        while l <= r {
            if l & 1 == 1 {
                if self.st[l] >= a {
                    let mut temp_l = l;
                    while temp_l < self.n {
                        temp_l = if self.st[temp_l << 1] >= a {
                            temp_l << 1
                        } else {
                            temp_l << 1 | 1
                        };
                    }
                    return temp_l - self.n;
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
    let input: Vec<u8> = stdin.lock().bytes().filter_map(Result::ok).collect();
    let mut iter = input.split(|&b| b == b'\n').map(|line| {
        std::str::from_utf8(line).unwrap().trim().to_string()
    });

    let t: usize = iter.next().unwrap().parse().unwrap();
    for _ in 0..t {
        let first_line = iter.next().unwrap();
        let parts: Vec<&str> = first_line.split_whitespace().collect();

        let n: usize = parts[0].parse().unwrap();
        let kl: usize = parts[1].parse().unwrap();
        let kr: usize = parts[2].parse().unwrap();
        let cc: Vec<char> = iter.next().unwrap().chars().collect();

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

        let mut segment_tree = SegmentTree::new();
        segment_tree.build(&zz);

        let mut ans = vec![0; n + 1];
        for p in 1..=n {
            let mut k = 0;
            let mut i = 0;
            while i < n {
                i = segment_tree.query(i + p, p);
                k += 1;
            }
            ans[k] = max(ans[k], p as i32);
        }

        for k in (1..n).rev() {
            ans[k] = max(ans[k], ans[k + 1]);
        }

        let result: Vec<String> = (kl..=kr).map(|k| ans[k] as i32).map(|x| x.to_string()).collect();
        println!("{}", result.join(" "));
    }
}