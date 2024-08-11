use std::time::{SystemTime, UNIX_EPOCH};
use std::io::{self, BufRead};
use rand::RngCore;

const N: usize = 200000;

fn main() {
    let mut rng = rand::thread_rng();
    let t: usize;
    io::stdin().read_line(&mut String::new()).unwrap();
    io::stdin().read_line(&mut String::new()).unwrap();
    loop {
        let mut input = String::new();
        if io::stdin().read_line(&mut input).unwrap() == 0 {
            break;
        }
        let mut tokens = input.trim().split_whitespace();
        t = tokens.next().unwrap().parse().unwrap();
        for _ in 0..t {
            let mut n = 0;
            let mut q = 0;
            let mut aa = [0; N + 1];
            let mut ii = [0; N + 1];
            for i in 1..N {
                ii[i] = i;
            }
            io::stdin().read_line(&mut input).unwrap();
            let mut tokens = input.trim().split_whitespace();
            n = tokens.next().unwrap().parse().unwrap();
            q = tokens.next().unwrap().parse().unwrap();
            for i in 1..n + 1 {
                aa[i] = rng.gen();
            }
            for i in 1..n + 1 {
                aa[i] ^= aa[i - 1];
            }
            sort(&mut ii, 0, n + 1);
            for _ in 0..q {
                let mut input = String::new();
                io::stdin().read_line(&mut input).unwrap();
                let mut tokens = input.trim().split_whitespace();
                let l = tokens.next().unwrap().parse().unwrap() - 1;
                let r = tokens.next().unwrap().parse().unwrap();
                println!("{}", if aa[l] == aa[r] || ii[search(&aa, l, &ii, n) + 1] < ii[search(&aa, r, &ii, n)] {
                    "YES"
                } else {
                    "NO"
                });
            }
        }
    }
}

fn sort(ii: &mut [usize], l: usize, r: usize) {
    while l < r {
        let i = l;
        let j = l;
        let k = r;
        let i_ = ii[l + rand::thread_rng().gen_range(l, r)];
        let mut tmp;
        while j < k {
            let c = if aa[ii[j]] != aa[i_] {
                aa[ii[j]] - aa[i_]
            } else {
                ii[j] - i_
            };
            if c == 0 {
                j += 1;
            } else if c < 0 {
                tmp = ii[i];
                ii[i] = ii[j];
                ii[j] = tmp;
                i += 1;
                j += 1;
            } else {
                k -= 1;
                tmp = ii[j];
                ii[j] = ii[k];
                ii[k] = tmp;
            }
        }
        sort(ii, l, i);
        l = k;
    }
}

fn search(a: &Vec<usize>, i: usize, ii: &Vec<usize>, n: usize) -> usize {
    let mut lower = -1;
    let mut upper = n + 1;
    while upper - lower > 1 {
        let h = (lower + upper) / 2;
        if a[ii[h]] < a[i] || a[ii[h]] == a[i] && ii[h] < i {
            lower = h;
        } else {
            upper = h;
        }
    }
    return lower;
}