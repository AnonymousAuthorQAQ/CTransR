use std::io::{self, Read, Write};
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
            n_: 0,
        }
    }

    fn build(&mut self, aa: &[i32], n: usize) {
        let mut n_ = 1;
        while n_ < n {
            n_ <<= 1;
        }
        self.n_ = n_;
        
        for i in 0..n {
            self.st[n_ + i] = aa[i];
        }
        for i in (1..n_).rev() {
            self.st[i] = max(self.st[i << 1], self.st[i << 1 | 1]);
        }
    }

    fn query(&self, mut l: usize, a: i32) -> usize {
        let mut r = self.n_ - 1;

        l += self.n_; // shift to the second half

        while l <= r {
            if (l & 1) == 1 {
                if self.st[l] >= a {
                    let mut v = l;
                    while v < self.n_ {
                        v = if self.st[v << 1] >= a {
                            v << 1
                        } else {
                            v << 1 | 1
                        };
                    }
                    return v - self.n_;
                }
                l += 1;
            } else {
                l >>= 1;
            }
            r >>= 1;
        }
        self.n_
    }
}

fn preprocess_zz(cc: &[char], n: usize) -> Vec<i32> {
    let mut zz = vec![0; n];
    let mut l = 0;
    let mut r = 0;

    for i in 1..n {
        if i + zz[i - l] < r {
            zz[i] = zz[i - l];
        } else {
            l = i;
            r = r.max(i);

            while r < n && cc[r] == cc[r - l] {
                r += 1;
            }
            zz[i] = r - l;
        }
    }
    zz
}

fn main() {
    let input = io::stdin().lock();
    let reader = input.bytes().filter_map(Result::ok);
    let mut buffer: Vec<u8> = vec![];

    for byte in reader {
        buffer.push(byte);
    }
    
    let data = String::from_utf8(buffer).unwrap();
    let mut lines = data.lines();
    
    let t: usize = lines.next().unwrap().parse().unwrap();
    let mut output = vec![];

    for _ in 0..t {
        // Read each test case in Newline-separated format
        let first_line: Vec<&str> = lines.next().unwrap().split_whitespace().collect();
        let n: usize = first_line[0].parse().unwrap();
        let kl: usize = first_line[1].parse().unwrap();
        let kr: usize = first_line[2].parse().unwrap();
        
        let cc: Vec<char> = lines.next().unwrap().chars().collect();
        
        let zz = preprocess_zz(&cc, n);
        
        let mut segment_tree = SegmentTree::new();
        segment_tree.build(&zz, n);
        
        let mut ans = vec![0; n + 1];

        for p in 1..=n {
            let mut k = 0;
            let mut i = 0;

            while let Some(tf) = if i < n {
                Some(segment_tree.query(i + p, p))
            } else {
                None
            } {
                k += 1;
                if tf >= n {
                    break;
                }
                i = tf;
            }
            ans[k] = max(ans[k], p as i32);
        }

        for k in (1..n).rev() {
            ans[k] = max(ans[k], ans[k + 1]);
        }

        for k in kl..=kr {
            output.push(ans[k].to_string());
        }
        output.push("\n".to_string());
    }
    
    print!("{}", output.join(" "));
}