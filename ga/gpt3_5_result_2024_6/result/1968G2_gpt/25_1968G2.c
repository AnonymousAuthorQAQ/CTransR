use std::io::{self, BufRead};

fn max(a: i32, b: i32) -> i32 {
    if a > b { a } else { b }
}

struct ZAlgorithm {
    z: Vec<i32>,
}

impl ZAlgorithm {
    fn new(s: &str) -> ZAlgorithm {
        let n = s.len();
        let mut z = vec![0; n];
        let (mut l, mut r) = (0, 0);

        for i in 1..n {
            if i + z[i - l] < r {
                z[i] = z[i - l];
            } else {
                l = i;
                r = r.max(i);
                while r < n && s.as_bytes()[r - l] == s.as_bytes()[r] {
                    r += 1;
                }
                z[i] = (r - l) as i32;
            }
        }

        ZAlgorithm { z }
    }

    fn query(&self, l: usize, a: i32) -> usize {
        let n = self.z.len();
        let mut l = l;
        let mut r = n - 1;

        while l <= r {
            if self.z[l] >= a {
                let mut i = l;
                while i < n && self.z[i] >= a {
                    i += 1;
                }
                return i - 1;
            }
            l += 1;
        }

        n
    }
}

fn main() {
    let stdin = io::stdin();
    let mut input = stdin.lock();
    let mut buffer = String::new();

    input.read_line(&mut buffer).unwrap();
    let t: i32 = buffer.trim().parse().unwrap();

    for _ in 0..t {
        buffer.clear();
        input.read_line(&mut buffer).unwrap();
        let mut iter = buffer.split_whitespace();
        let n: usize = iter.next().unwrap().parse().unwrap();
        let kl: i32 = iter.next().unwrap().parse().unwrap();
        let kr: i32 = iter.next().unwrap().parse().unwrap();
        let cc: String = iter.next().unwrap().to_string();

        let z_algo = ZAlgorithm::new(&cc);
        let mut zz = vec![0; n];
        for p in 1..=n {
            let mut k = 0;
            let mut i = 0;

            while i < n {
                i = z_algo.query(i + p, p as i32);
                k += 1;
            }

            zz[k as usize - 1] = p;
        }

        let mut ans = vec![0; n + 1];
        for p in 0..=n {
            let mut k = 0;

            for i in 0..n {
                if zz[i] >= (k + 1) as usize {
                    k += 1;
                }
            }

            ans[k] = ans[k].max(p);
        }

        for k in kl..=kr {
            print!("{} ", ans[k as usize]);
        }
        println!();
    }
}