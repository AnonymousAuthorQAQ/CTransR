use std::io::{self, BufRead};
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};

fn z_function(s: &str) -> Vec<usize> {
    let n = s.len();
    let mut z = vec![0; n];
    let (mut l, mut r) = (0, 0);
    for i in 1..n {
        if i <= r && z[i - l] < r - i + 1 {
            z[i] = z[i - l];
        } else {
            z[i] = std::cmp::max(0, r - i + 1);
            while i + z[i] < n && s.chars().nth(z[i]).unwrap() == s.chars().nth(i + z[i]).unwrap() {
                z[i] += 1;
            }
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
    let mut lines = stdin.lock().lines().map(Result::unwrap);
    
    let t: usize = lines.next().unwrap().parse().unwrap();
    
    for _ in 0..t {
        let inputs: Vec<usize> = lines.next().unwrap().split_whitespace()
            .map(|x| x.parse().unwrap()).collect();
        let n = inputs[0];
        let l = inputs[1];
        let r = inputs[2];

        let s = lines.next().unwrap().to_string();

        let mut ls = 0;
        let mut rs = n + 1;
        let z = z_function(&s);

        let mut f = |x: usize| -> bool {
            if x > n / l {
                return false;
            }
            let mut cnt = 0;
            for i in (x..s.len()).step_by(x) {
                if z[i] >= x {
                    cnt += 1;
                }
            }
            cnt >= l - 1
        };

        while ls + 1 < rs {
            let mid = (ls + rs) / 2;
            match f(mid) {
                true => ls = mid,
                false => rs = mid,
            }
        }

        println!("{}", ls);
    }
}