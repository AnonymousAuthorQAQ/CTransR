use std::io::{self, BufRead};

fn max(a: i32, b: i32) -> i32 {
    a.max(b)
}

struct SegmentTree {
    tree: Vec<i32>,
}

impl SegmentTree {
    fn new(n: usize) -> SegmentTree {
        let capacity = 1;
        while capacity < n {
            capacity <<= 1;
        }
        SegmentTree { tree: vec![0; 2 * capacity] }
    }

    fn build(&mut self, data: &[i32], n: usize) {
        let mut n = n;
        while n > 0 {
            for i in (n..n + n).rev() {
                self.tree[i] = max(data[i - n], self.tree[i - n]);
                if i % 2 == 0 {
                    self.tree[i / 2] = max(self.tree[i], self.tree[i + 1]);
                }
            }
            n /= 2;
        }
    }

    fn query(&self, l: usize, r: usize, mut a: i32) -> i32 {
        let mut l = l + r;
        let mut r = l + r;
        while l < r {
            if l % 2 == 1 {
                if self.tree[l] >= a {
                    while l < r {
                        l = 2 * l;
                        if self.tree[l] < a {
                            l += 1;
                        }
                    }
                    return l as i32 - r as i32;
                }
                l += 1;
            }
            l /= 2;
            r /= 2;
        }
        r as i32
    }
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let t = lines.next().unwrap().unwrap().parse::<i32>().unwrap();
    for _ in 0..t {
        let mut line = lines.next().unwrap().unwrap();
        let n: i32 = line.find(|c: char| c.is_digit(10)).unwrap().to_digit(10).unwrap() as i32;
        let kl: i32 = line.find(|c: char| c.is_digit(10)).unwrap().to_digit(10).unwrap() as i32;
        let kr: i32 = line.find(|c: char| c.is_digit(10)).unwrap().to_digit(10).unwrap() as i32;
        let cc = &line[line.find(|c: char| c.is_alphabetic()).unwrap()..];

        let mut zz: Vec<i32> = vec![0; n as usize];
        let mut l = 0;
        let mut r = 0;

        for i in 1..n as usize {
            if i + zz[i - l] < r {
                zz[i] = zz[i - l];
            } else {
                l = i;
                r = std::cmp::max(r, i);
                while r < n as usize && cc.chars().nth(r).unwrap() == cc.chars().nth(r - l).unwrap() {
                    r += 1;
                }
                zz[i] = r - l;
            }
        }

        let mut tree = SegmentTree::new(n as usize);
        tree.build(&zz, n as usize);

        let mut ans = vec![0; (n + 1) as usize];
        for p in 1..=n {
            let mut k = 0;
            let mut i = 0;
            while i < n as usize {
                i = tree.query(i, n as usize, p) as usize;
                k += 1;
            }
            ans[k as usize] = max(ans[k as usize], p);
        }

        for k in (1..n).rev() {
            ans[k as usize] = max(ans[k as usize], ans[(k + 1) as usize]);
        }

        for k in kl..=kr {
            print!("{} ", ans[k as usize]);
        }
        println!();
    }
}