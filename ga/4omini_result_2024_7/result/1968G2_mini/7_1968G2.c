use std::io::{self, BufRead};

const N: usize = 200_000;
const N_: usize = 1 << 18; // N_ = pow2(ceil(log2(N)))

fn max(a: i32, b: i32) -> i32 {
    if a > b { a } else { b }
}

struct SegmentTree {
    st: [i32; N_ * 2],
    size: usize,
}

impl SegmentTree {
    fn new() -> Self {
        SegmentTree {
            st: [0; N_ * 2],
            size: N_,
        }
    }

    fn build(&mut self, aa: &[i32], n: usize) {
        for i in 0..n {
            self.st[self.size + i] = aa[i];
        }
        for i in (1..self.size).rev() {
            self.st[i] = max(self.st[i * 2], self.st[i * 2 + 1]);
        }
    }

    fn query(&self, mut l: usize, mut r: usize, a: i32) -> usize {
        let mut res = self.size - 1;

        l += self.size;
        r += self.size;

        while l <= r {
            if l % 2 == 1 {
                if self.st[l] >= a {
                    let mut tmp = l;
                    while tmp < self.size {
                        tmp = if self.st[tmp * 2] >= a {
                            tmp * 2
                        } else {
                            tmp * 2 + 1
                        }
                    }
                    return tmp - self.size;
                }
                l += 1;
            }
            l /= 2;
            r /= 2;
        }
        res
    }
}

fn main() {
    let stdin = io::stdin();
    let stdin_locked = stdin.lock();
    let reader = stdin_locked.lines();

    let mut iterator = reader.filter_map(Result::ok);

    let t: usize = iterator.next().unwrap().parse().unwrap();
    for _ in 0..t {
        let line = iterator.next().unwrap();
        let parts: Vec<_> = line.split_whitespace().collect();
        let n: usize = parts[0].parse().unwrap();
        let kl: usize = parts[1].parse().unwrap();
        let kr: usize = parts[2].parse().unwrap();
        let cc: String = iterator.next().unwrap();

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

        let mut st = SegmentTree::new();
        st.build(&zz, n);
        
        for p in 1..=n {
            let mut k = 0;
            let mut i = 0;
            while i < n {
                i = st.query(i + p, p as i32);
                if i >= n {
                    break;
                }
                k += 1;
            }
            ans[k] = max(ans[k], p as i32);
        }

        for k in (1..n).rev() {
            ans[k] = max(ans[k], ans[k + 1]);
        }

        let result: Vec<String> = (kl..=kr).map(|k| ans[k].to_string()).collect();
        println!("{}", result.join(" "));
    }
}