use std::io::{self, BufRead};

fn z_function(s: &str) -> Vec<usize> {
    let n = s.len();
    let mut z: Vec<usize> = vec![0; n];
    let s_bytes = s.as_bytes();

    for i in 1..n {
        let mut l = 0;
        let mut r = 0;

        if i <= r {
            z[i] = std::cmp::min(r - i + 1, z[i - l]);
        }

        while i + z[i] < n && s_bytes[z[i]] == s_bytes[i + z[i]] {
            z[i] += 1;
        }

        if i + z[i] - 1 > r {
            l = i;
            r = i + z[i] - 1;
        }
    }

    z[0] = n;
    z
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines().map(|x| x.unwrap());
    let t: usize = lines.next().unwrap().parse().unwrap();

    for _ in 0..t {
        let input_line: String = lines.next().unwrap();
        let mut input_iter = input_line.split_whitespace().map(|x| x.parse().unwrap());
        let n: usize = input_iter.next().unwrap();
        let l: usize = input_iter.next().unwrap();
        let r: usize = input_iter.next().unwrap();
        let s: String = lines.next().unwrap();

        let z = z_function(&s);
        
        let mut ls = 0;
        let mut rs = n + 1;

        let f = |x: usize| -> bool {
            if x > n / l {
                return false;
            }

            let mut cnt = 0;
            let mut i = x;
            while i < s.len() {
                if z[i] >= x {
                    cnt += 1;
                    i += x;
                } else {
                    i += 1;
                }
            }
            
            cnt >= l - 1
        };

        while ls + 1 < rs {
            let mid = (ls + rs) / 2;
            if f(mid) {
                ls = mid;
            } else {
                rs = mid;
            }
        }

        println!("{}", ls);
    }
}