use std::io::{self, BufRead};

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
        // Initialize segment tree
        self.st.fill(0);
        for i in 0..n {
            self.st[self.n_ + i] = aa[i];
        }
        for i in (1..self.n_).rev() {
            self.st[i] = max(self.st[i << 1], self.st[i << 1 | 1]);
        }
    }

    fn query(&self, mut l: usize, a: i32) -> usize {
        let mut r = self.n_ - 1;

        l += self.n_; // Shift `l` to leaf index
        r += self.n_; // Shift `r` to leaf index

        while l <= r {
            if (l & 1) == 1 {
                if self.st[l] >= a {
                    let mut temp_l = l;
                    while temp_l < self.n_ {
                        temp_l = if self.st[temp_l << 1] >= a { 
                                       temp_l << 1 
                                   } else { 
                                       temp_l << 1 | 1 
                                   };
                    }
                    return temp_l - self.n_; // Remote back to original index
                }
                l += 1;
            }
            l >>= 1;
            r >>= 1;
        }
        self.n_ // indicates that no index meets the criteria
    }
}

fn main() {
    let stdin = io::stdin(); 
    let reader = stdin.lock();
    let mut lines = reader.lines();

    let t: usize = lines.next().unwrap().unwrap().parse().unwrap();

    for _ in 0..t {
        let first_line: Vec<&str> = lines.next().unwrap().unwrap().split_whitespace().collect();
        let n: usize = first_line[0].parse().unwrap();
        let kl: usize = first_line[1].parse().unwrap();
        let kr: usize = first_line[2].parse().unwrap();
        let cc: Vec<char> = first_line[3].chars().collect();

        let mut zz = vec![0; n];
        let (mut l, mut r) = (0, 0); // l and r corresponding to indexes

        for i in 1..n {
            if i + zz[i - l] < r {
                zz[i] = zz[i - l];
            } else {
                l = i as i32;
                r = max(r, i);
                while r < n && cc[r] == cc[r - l as usize] {
                    r += 1; // Increment to compare next characters
                }
                zz[i] = (r - l) as i32; // Length of matching prefix
            }
        }

        let mut segment_tree = SegmentTree::new();
        segment_tree.build(&zz, n); 

        let mut ans = vec![0; n + 1];

        for p in 1..=n {
            let mut k = 0;
            let mut i = 0;

            while i < n {
                i = segment_tree.query(i + p as usize, p as i32);
                k += 1;
            }
            ans[k] = max(ans[k], p as i32);
        }

        for k in (1..=n).rev() {
            ans[k - 1] = max(ans[k - 1], ans[k]);
        }

        let mut output: Vec<String> = Vec::new();

        for k in kl..=kr {
            output.push(ans[k].to_string());
        }

        println!("{}", output.join(" "));
    }
}