use std::io::{self, Read};

const N: usize = 200000;
const N_: usize = 1 << 18; // N_ = pow2(ceil(log2(N)))

fn max(a: i32, b: i32) -> i32 {
    if a > b { a } else { b }
}

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
        self.n_ = 1;
        while self.n_ < n {
            self.n_ <<= 1;
        }
        self.st.fill(0);
        
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
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    
    let mut lines = input.lines();
    let t: usize = lines.next().unwrap().parse().unwrap();
    
    for _ in 0..t {
        let line = lines.next().unwrap();
        let params: Vec<&str> = line.split_whitespace().collect();
        
        let n: usize = params[0].parse().unwrap();
        let kl: usize = params[1].parse().unwrap();
        let kr: usize = params[2].parse().unwrap();
        let cc: Vec<char> = params[3].chars().collect();

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
        segment_tree.build(&zz, n);

        let mut ans = vec![0; n + 1];
        
        for p in 1..=n {
            let mut k = 0;
            let mut i = 0;
            
            while i < n {
                i = segment_tree.query(i + p, p as i32);
                k += 1;
            }
            ans[k] = max(ans[k], p as i32);
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