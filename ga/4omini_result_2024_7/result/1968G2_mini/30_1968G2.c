use std::io::{self, BufRead};

fn max(a: usize, b: usize) -> usize {
    if a > b {
        a
    } else {
        b
    }
}

struct SegmentTree {
    st: Vec<usize>,
    n: usize,
}

impl SegmentTree {
    fn new(size: usize) -> Self {
        let n = size.next_power_of_two();
        let st = vec![0; 2 * n];
        SegmentTree { st, n }
    }

    fn build(&mut self, aa: &[usize], n: usize) {
        self.st[self.n..self.n + n].copy_from_slice(aa);
        for i in (1..self.n).rev() {
            self.st[i] = max(self.st[i << 1], self.st[i << 1 | 1]);
        }
    }

    fn query(&self, mut l: usize, a: usize) -> usize {
        let mut r = self.n - 1;
        let mut l = l + self.n;
        let mut r = r + self.n;

        while l <= r {
            if l & 1 == 1 {
                if self.st[l] >= a {
                    let mut index = l;
                    while index < self.n {
                        index = if self.st[index << 1] >= a {
                            index << 1
                        } else {
                            index << 1 | 1
                        };
                    }
                    return index - self.n;
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
    let lines: Vec<Vec<String>> = reader.lines()
        .map(|line| line.unwrap().split_whitespace().map(String::from).collect())
        .collect();

    let t: usize = lines[0][0].parse().unwrap();

    for case in 0..t {
        let n: usize = lines[case + 1][0].parse().unwrap();
        let kl: usize = lines[case + 1][1].parse().unwrap();
        let kr: usize = lines[case + 1][2].parse().unwrap();
        let cc: Vec<char> = lines[case + 1][3].chars().collect();

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

        let mut segment_tree = SegmentTree::new(n);
        segment_tree.build(&zz, n);
        
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

        for k in (1..n).rev() {
            ans[k] = max(ans[k], ans[k + 1]);
        }

        let result: Vec<String> = (kl..=kr).map(|k| ans[k].to_string()).collect();
        println!("{}", result.join(" "));
    }
}