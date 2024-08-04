use std::io::{self, Read};

fn max(a: i32, b: i32) -> i32 {
    if a > b { a } else { b }
}

struct SegmentTree {
    tree: Vec<i32>,
    size: usize,
}

impl SegmentTree {
    fn new(n: usize) -> Self {
        let size = 1 << (32 - (n as f64).log(2.0).ceil() as usize);
        let tree = vec![0; size * 2];
        Self { tree, size }
    }

    fn build(&mut self, aa: &[i32], n: usize) {
        for i in 0..n {
            self.tree[self.size + i] = aa[i];
        }
        for i in (1..self.size).rev() {
            self.tree[i] = max(self.tree[i * 2], self.tree[i * 2 + 1]);
        }
    }

    fn query(&self, l: usize, a: i32) -> usize {
        let mut l = l + self.size;
        let mut r = self.size - 1 + self.size; // Full range
        let mut result = self.size; // Default result

        while l <= r {
            if l & 1 == 1 {
                if self.tree[l] >= a {
                    let mut temp = l;
                    while temp < self.size {
                        temp = if self.tree[temp * 2] >= a { temp * 2 } else { temp * 2 + 1 };
                    }
                    result = temp - self.size;
                }
                l += 1;
            }
            l >>= 1;
            r >>= 1;
        }
        result
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut lines = input.lines();
    let t: usize = lines.next().unwrap().parse().unwrap();

    for _ in 0..t {
        let first_line: Vec<&str> = lines.next().unwrap().split_whitespace().collect();
        let n: usize = first_line[0].parse().unwrap();
        let kl: usize = first_line[1].parse().unwrap();
        let kr: usize = first_line[2].parse().unwrap();
        let cc: String = lines.next().unwrap().to_string();

        let mut zz = vec![0; n];
        let (mut l, mut r): (usize, usize) = (0, 0);

        for i in 1..n {
            if i + zz[i - l] < r {
                zz[i] = zz[i - l];
            } else {
                l = i;
                r = max(r, i);
                while r < n && cc[r..].chars().next() == cc[(r - l)..].chars().next() {
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
            ans[k] = max(ans[k], p as i32);
        }
        for k in (1..=n-1).rev() {
            ans[k] = max(ans[k], ans[k + 1]);
        }

        let output: Vec<i32> = (kl..=kr).map(|k| ans[k]).collect();
        println!("{}", output.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(" "));
    }
}