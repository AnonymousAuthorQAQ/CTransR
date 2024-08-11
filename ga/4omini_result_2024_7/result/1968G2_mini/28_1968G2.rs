use std::io::{self, BufRead};

fn max(a: usize, b: usize) -> usize {
    if a > b { a } else { b }
}

struct SegmentTree {
    st: Vec<usize>,
    n_: usize,
}

impl SegmentTree {
    fn new(size: usize) -> Self {
        let n_ = size.next_power_of_two(); // equivalent to pow2(ceil(log2(size)))
        SegmentTree {
            st: vec![0; n_ * 2],
            n_,
        }
    }

    fn build(&mut self, aa: &[usize]) {
        self.st[self.n_..self.n_ + aa.len()].copy_from_slice(aa);
        for i in (1..self.n_).rev() {
            self.st[i] = max(self.st[i * 2], self.st[i * 2 + 1]);
        }
    }

    fn query(&self, l: usize, a: usize) -> usize {
        let mut l = l + self.n_;
        let mut r = self.n_ - 1 + self.n_;

        while l <= r {
            if l & 1 == 1 {
                if self.st[l] >= a {
                    let mut pos = l;
                    // Locate the actual position
                    while pos < self.n_ {
                        if self.st[pos * 2] >= a {
                            pos = pos * 2; // Left child
                        } else {
                            pos = pos * 2 + 1; // Right child
                        }
                    }
                    return pos - self.n_;
                }
                l += 1;
            }
            l >>= 1;
            r >>= 1;
        }
        self.n_ // default to the size of the array
    }
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let lines: Vec<String> = reader.lines().filter_map(Result::ok).collect();
    
    let mut index = 0;
    let t: usize = lines[index].trim().parse().unwrap(); index += 1;

    while t > 0 {
        t -= 1;

        let first_line: Vec<&str> = lines[index].trim().split_whitespace().collect();
        index += 1;

        let n: usize = first_line[0].parse().unwrap();
        let kl: usize = first_line[1].parse().unwrap();
        let kr: usize = first_line[2].parse().unwrap();
        let cc: &str = &lines[index].trim(); index += 1;

        let mut zz = vec![0; n];
        let mut ans = vec![0; n + 1];

        let mut l = 0;
        let mut r = 0;

        for i in 1..n {
            if i + zz[i - l] < r {
                zz[i] = zz[i - l];
            } else {
                l = i;
                r = max(r, i);
                while r < n && cc.as_bytes()[r] == cc.as_bytes()[r - l] {
                    r += 1;
                }
                zz[i] = r - l;
            }
        }

        let mut st = SegmentTree::new(n);
        st.build(&zz);

        for p in 1..=n {
            let mut k = 0;
            let mut i = 0;
            while i < n {
                i = st.query(i + p, p);
                k += 1;
            }
            ans[k] = max(ans[k], p);
        }

        for k in (1..=n).rev() {
            ans[k - 1] = max(ans[k - 1], ans[k]);
        }

        let result: Vec<String> = (kl..=kr).map(|k| ans[k].to_string()).collect();
        println!("{}", result.join(" "));
    }
}