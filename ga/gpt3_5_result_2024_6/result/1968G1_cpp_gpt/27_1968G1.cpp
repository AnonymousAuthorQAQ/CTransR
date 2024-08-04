use std::io::stdin;
use std::io::Read;

fn z_function(s: &str) -> Vec<usize> {
    let n = s.len();
    let mut z = vec![0; n];
    let (mut l, mut r) = (0, 0);
    for i in 1..n {
        if i <= r && z[i - l] < r - i + 1 {
            z[i] = z[i - l];
        } else {
            z[i] = i.max(r) - i;
            while i + z[i] < n && s.as_bytes()[z[i]] == s.as_bytes()[i + z[i]] {
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
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut iter = input.split_whitespace();

    let t: usize = iter.next().unwrap().parse().unwrap();
    
    for _ in 0..t {
        let n: usize = iter.next().unwrap().parse().unwrap();
        let l: usize = iter.next().unwrap().parse().unwrap();
        let r: usize = iter.next().unwrap().parse().unwrap();
        let s: &str = iter.next().unwrap();

        let mut ls = 0;
        let mut rs = n + 1;

        let z = z_function(s);
        
        let mut f = |x: usize| -> bool {
            if x > n / l {
                return false;
            }
            let mut cnt = 0;
            for i in (x..z.len()).step_by(x) {
                if z[i] >= x {
                    cnt += 1;
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