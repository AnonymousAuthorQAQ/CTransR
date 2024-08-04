use std::io::{self, Read};

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
        let st = vec![0; n * 2];
        SegmentTree { st, n }
    }

    fn build(&mut self, aa: &[usize]) {
        let n = aa.len();
        for (i, &value) in aa.iter().enumerate() {
            self.st[self.n + i] = value;
        }
        for i in (1..self.n).rev() {
            self.st[i] = max(self.st[i * 2], self.st[i * 2 + 1]);
        }
    }

    fn query(&self, mut l: usize, mut r: usize, a: usize) -> usize {
        l += self.n;
        r += self.n;
        while l <= r {
            if l % 2 == 1 {
                if self.st[l] >= a {
                    let mut pos = l;
                    while pos < self.n {
                        pos = if self.st[pos * 2] >= a {
                            pos * 2
                        } else {
                            pos * 2 + 1
                        }
                    }
                    return pos - self.n;
                }
                l += 1;
            }
            l /= 2;
            r /= 2;
        }
        self.n
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut lines = input.lines();

    let t: usize = lines.next().unwrap().parse().unwrap();
    for _ in 0..t {
        let first_line: Vec<usize> = lines.next().unwrap().split_whitespace()
            .map(|x| x.parse().unwrap()).collect();
        let n = first_line[0];
        let kl = first_line[1];
        let kr = first_line[2];
        let cc: String = lines.next().unwrap().to_string();

        let mut zz = vec![0; n];
        let mut l = 0;
        let mut r = 0;

        for i in 1..n {
            if i + zz[i - l] < r {
                zz[i] = zz[i - l];
            } else {
                l = i;
                r = max(r, i);
                while r < n && cc[r..=r].chars().next().unwrap() == cc[r - l..=r - l].chars().next().unwrap() {
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
                i = segment_tree.query(i + p, n - 1, p);
                k += 1;
            }
            ans[k] = max(ans[k], p);
        }

        for k in (1..=n - 1).rev() {
            ans[k] = max(ans[k], ans[k + 1]);
        }

        let result: Vec<usize> = (kl..=kr).map(|k| ans[k]).collect();
        println!("{}", result.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(" "));
    }
}