use std::cmp::max;
use std::io::{self, Read};

const N: usize = 200000;
const N_: usize = 1 << 18; // N_ = pow2(ceil(log2(N)))

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
        self.st[..2 * self.n_].fill(0); // Clear the segment tree

        for i in 0..n {
            self.st[self.n_ + i] = aa[i];
        }
        for i in (1..self.n_).rev() {
            self.st[i] = max(self.st[i << 1], self.st[i << 1 | 1]);
        }
    }

    fn query(&self, mut l: usize, a: i32) -> usize {
        let mut r = self.n_ - 1;
        l += self.n_;
        r += self.n_;

        while l <= r {
            if l & 1 == 1 {
                if self.st[l] >= a {
                    // Move down the tree to find the valid leaf
                    let mut x = l;
                    while x < self.n_ {
                        x = if self.st[x << 1] >= a { x << 1 } else { x << 1 | 1 };
                    }
                    return x - self.n_;
                }
                l += 1;
            }
            l >>= 1;
            r >>= 1;
        }
        self.n_
    }
}

fn compute_z_values(cc: &[char], n: usize) -> Vec<usize> {
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
    zz
}

fn main() {
    let stdin = io::stdin();
    let input = stdin.lock().bytes().map(|b| b.unwrap() as char).collect::<Vec<_>>();
    
    let mut iter = input.split(|&c| c == ' ' || c == '\n').filter(|&&c| c != '\0');
    let t: usize = iter.next().unwrap().consume_while(is_char_digit).unwrap().parse().unwrap();

    for _ in 0..t {
        let n: usize = iter.next().unwrap().consume_while(is_char_digit).unwrap().parse().unwrap();
        let kl: usize = iter.next().unwrap().consume_while(is_char_digit).unwrap().parse().unwrap();
        let kr: usize = iter.next().unwrap().consume_while(is_char_digit).unwrap().parse().unwrap();
        let cc: Vec<char> = iter.next().unwrap().iter().copied().filter(|&c| c != '\0').collect();

        let zz = compute_z_values(&cc, n);
        let mut st = SegmentTree::new();
        st.build(&zz.iter().map(|&x| x as i32).collect::<Vec<_>>(), n);

        let mut ans = vec![0; n + 1];

        for p in 1..=n {
            let mut k = 0;
            let mut i = 0;
            while i < n {
                i = st.query(i + p, p as i32);
                if i < n {
                    k += 1;
                } else {
                    break;
                }
            }
            ans[k] = max(ans[k], p);
        }

        for k in (1..n).rev() {
            ans[k] = max(ans[k], ans[k + 1]);
        }

        let result: Vec<_> = (kl..=kr).map(|k| ans[k]).collect();
        println!("{}", result.into_iter().map(|x| x.to_string()).collect::<Vec<_>>().join(" "));
    }
}

// Helper function to consume digits from the input
fn is_char_digit(c: &char) -> bool {
    c.is_digit(10)
}

// Consumes digits from a character iterator and returns the substring
trait ConsumeWhile {
    fn consume_while<F>(&mut self, f: F) -> Option<Self>
    where
        Self: Sized,
        F: Fn(char) -> bool;
}

impl<I> ConsumeWhile for I
where
    I: Iterator<Item = char>,
{
    fn consume_while<F>(&mut self, f: F) -> Option<Self>
    where
        Self: Sized,
        F: Fn(char) -> bool,
    {
        let result: String = self.take_while(|&c| f(c)).collect();
        if result.is_empty() { None } else { Some(result.chars().collect()) }
    }
}