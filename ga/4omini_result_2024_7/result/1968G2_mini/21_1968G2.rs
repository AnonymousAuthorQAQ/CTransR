use std::io;
use std::str;

#[derive(Debug)]
struct SegmentTree {
    st: Vec<i32>,
    n_: usize,
}

impl SegmentTree {
    fn new(size: usize) -> Self {
        let mut n_ = 1;
        while n_ < size {
            n_ <<= 1;
        }
        SegmentTree {
            st: vec![0; n_ * 2],
            n_,
        }
    }
    
    fn build(&mut self, aa: &[i32], n: usize) {
        self.st[self.n_..self.n_ + n].copy_from_slice(aa);
        for i in (1..self.n_).rev() {
            self.st[i] = self.st[i << 1].max(self.st[i << 1 | 1]);
        }
    }

    fn query(&self, mut l: usize, mut a: i32) -> usize {
        let mut r = self.n_ - 1;
        l += self.n_;
        r += self.n_;
        
        while l <= r {
            if l & 1 == 1 {
                if self.st[l] >= a {
                    let mut position = l;
                    while position < self.n_ {
                        position = if self.st[position << 1] >= a {
                            position << 1
                        } else {
                            position << 1 | 1
                        };
                    }
                    return position - self.n_;
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
    let stdin = io::stdin();
    let mut input = String::new();
    stdin.read_line(&mut input).unwrap();
    let t: usize = input.trim().parse().unwrap();

    for _ in 0..t {
        input.clear();
        stdin.read_line(&mut input).unwrap();
        let mut iter = input.split_whitespace();
        let n: usize = iter.next().unwrap().parse().unwrap();
        let kl: usize = iter.next().unwrap().parse().unwrap();
        let kr: usize = iter.next().unwrap().parse().unwrap();
        let cc: String = iter.next().unwrap().to_string();

        let mut zz = vec![0; n];
        let mut l = 0;
        let mut r = 0;

        for i in 1..n {
            if i + zz[i - l] < r {
                zz[i] = zz[i - l];
            } else {
                l = i;
                r = l;
                while r < n && cc.chars().nth(r) == cc.chars().nth(r - l) {
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
                i = segment_tree.query(i + p, p as i32);
                k += 1;
            }
            ans[k] = ans[k].max(p);
        }

        for k in (1..n).rev() {
            ans[k] = ans[k].max(ans[k + 1]);
        }

        for k in kl..=kr {
            print!("{} ", ans[k]);
        }
        println!();
    }
}