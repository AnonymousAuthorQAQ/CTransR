use std::io::{self, BufRead};

fn max(a: usize, b: usize) -> usize {
    if a > b { a } else { b }
}

struct SegmentTree {
    st: Vec<usize>,
    n: usize,
}

impl SegmentTree {
    fn new(size: usize) -> Self {
        let n = size.next_power_of_two();
        SegmentTree {
            st: vec![0; n * 2],
            n,
        }
    }

    fn build(&mut self, aa: &[usize]) {
        self.st[self.n..self.n + aa.len()].copy_from_slice(aa);
        for i in (1..self.n).rev() {
            self.st[i] = max(self.st[i * 2], self.st[i * 2 + 1]);
        }
    }

    fn query(&self, mut l: usize, mut a: usize) -> usize {
        let mut r = self.n - 1;
        let (mut l, mut r) = (l + self.n, r + self.n);

        while l <= r {
            if l & 1 == 1 {
                if self.st[l] >= a {
                    let mut result = l;
                    while result < self.n {
                        result = if self.st[result * 2] >= a {
                            result * 2
                        } else {
                            result * 2 + 1
                        };
                    }
                    return result - self.n;
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
    let reader = stdin.lock();

    let mut inputs = reader.lines().map(|x| x.unwrap());
    let t: usize = inputs.next().unwrap().parse().unwrap();

    for _ in 0..t {
        let header: Vec<&str> = inputs.next().unwrap().split_whitespace().collect();
        let n: usize = header[0].parse().unwrap();
        let kl: usize = header[1].parse().unwrap();
        let kr: usize = header[2].parse().unwrap();
        let cc: String = inputs.next().unwrap();

        let mut zz = vec![0; n];
        let (mut l, mut r) = (0, 0);
        
        for i in 1..n {
            if i + zz[i - l] < r {
                zz[i] = zz[i - l];
            } else {
                l = i;
                r = max(r, i);
                while r < n && cc.chars().nth(r) == cc.chars().nth(r - l) {
                    r += 1;
                }
                zz[i] = r - l;
            }
        }

        let mut segment_tree = SegmentTree::new(n);
        segment_tree.build(&zz);

        let mut ans = vec![0; n + 1];
        for p in 1..=n {
            let mut k = 0;
            let mut i = 0;

            while i < n {
                i = segment_tree.query(i + p, p);
                k += 1;
            }
            ans[k] = max(ans[k], p);
        }

        for k in (1..=n - 1).rev() {
            ans[k] = max(ans[k], ans[k + 1]);
        }

        let result: Vec<String> = (kl..=kr).map(|k| ans[k].to_string()).collect();
        println!("{}", result.join(" "));
    }
}