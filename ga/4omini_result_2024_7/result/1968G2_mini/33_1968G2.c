use std::io::{self, BufRead};
use std::cmp::max;

const N: usize = 200000;
const N_: usize = 1 << 18; // N_ = pow2(ceil(log2(N)))

struct SegmentTree {
    st: [i32; N_ * 2],
}

impl SegmentTree {
    fn new() -> Self {
        Self { st: [0; N_ * 2] }
    }

    fn build(&mut self, aa: &[i32], n: usize) {
        let mut n_ = 1;
        while n_ < n {
            n_ <<= 1;
        }
        self.st[..2*n_].fill(0);
        for i in 0..n {
            self.st[n_ + i] = aa[i];
        }
        for i in (1..n_).rev() {
            self.st[i] = max(self.st[i << 1], self.st[i << 1 | 1]);
        }
    }

    fn query(&self, l: usize, a: i32) -> usize {
        let mut l = l + N_ - 1;
        let mut r = N_ - 1;

        while l <= r {
            if l & 1 == 1 {
                if self.st[l] >= a {
                    let mut temp = l;
                    while temp < N_ {
                        temp = if self.st[temp << 1] >= a {
                            temp << 1
                        } else {
                            temp << 1 | 1
                        };
                    }
                    return temp - N_;
                }
                l += 1;
            }
            l >>= 1;
            r >>= 1;
        }
        N_
    }
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    
    let mut lines = reader.lines();
    let t: usize = lines.next().unwrap().unwrap().parse().unwrap();
    
    for _ in 0..t {
        let input: Vec<_> = lines.next().unwrap().unwrap().split_whitespace().collect();
        let n: usize = input[0].parse().unwrap();
        let kl: usize = input[1].parse().unwrap();
        let kr: usize = input[2].parse().unwrap();
        let cc: Vec<char> = input[3].chars().collect();

        let mut zz: Vec<i32> = vec![0; n];
        let mut ans: Vec<i32> = vec![0; n + 1];

        let mut l = 0;
        let mut r = 0;
        
        for i in 1..n {
            if i + zz[i - l as usize] < r {
                zz[i] = zz[i - l as usize];
            } else {
                l = i;
                r = max(r, i);
                while r < n && cc[r] == cc[(r - l) as usize] {
                    r += 1;
                }
                zz[i] = (r - l) as i32;
            }
        }
        
        let mut st = SegmentTree::new();
        st.build(&zz, n);
        
        for p in 1..=n {
            let mut k = 0;
            let mut i = 0;

            while i < n {
                i = st.query(i + p, p);
                k += 1;
            }
            ans[k as usize] = max(ans[k as usize], p as i32);
        }
        
        for k in (1..n).rev() {
            ans[k] = max(ans[k], ans[k + 1]);
        }
        
        let result: Vec<i32> = ans[kl..=kr].to_vec();
        println!("{}", result.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(" "));
    }
}