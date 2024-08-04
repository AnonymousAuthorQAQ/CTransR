use std::io::{self, BufRead};

const N: usize = 200000;

fn rand_() -> u32 {
    static mut Z: u32 = 0;
    Z = (Z.wrapping_mul(3)) >> 1;
    Z
}

fn srand_() {
    let tv = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("Time went backwards");
    let (sec, nsec) = (tv.as_secs(), tv.subsec_nanos());
    unsafe {
        Z = (sec as u32) ^ (nsec as u32) | 1;
    }
}

fn sort(aa: &mut [i32], ii: &mut [usize], l: usize, r: usize) {
    while l < r {
        let i = l;
        let j = l;
        let k = r;
        let i_ = ii[l + rand_() as usize % (r - l)];
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
        sort(aa, ii, l, i);
        l = k;
    }
}

fn search(a: i32, i: usize, aa: &[i32], ii: &[usize]) -> isize {
    let mut lower = -1;
    let mut upper = (N + 1) as isize;

    while upper - lower > 1 {
        let h = (lower + upper) / 2;
        if aa[ii[h as usize]] < a || (aa[ii[h as usize]] == a && ii[h as usize] < i) {
            lower = h;
        } else {
            upper = h;
        }
    }
    lower
}

fn main() {
    srand_();
    let stdin = io::stdin();
    let mut line = String::new();
    stdin.read_line(&mut line).expect("Failed to read line");
    let t: usize = line.trim().parse().expect("Invalid input");

    for _ in 0..t {
        line.clear();
        stdin.read_line(&mut line).expect("Failed to read line");
        let mut iter = line.trim().split_whitespace();
        let n: usize = iter.next().unwrap().parse().expect("Invalid input");
        let q: usize = iter.next().unwrap().parse().expect("Invalid input");

        let mut aa = vec![0; n + 1];
        for i in 1..=n {
            line.clear();
            stdin.read_line(&mut line).expect("Failed to read line");
            aa[i] = line.trim().parse().expect("Invalid input");
            aa[i] ^= aa[i - 1];
        }

        let mut ii = (0..=n).collect::<Vec<usize>>();
        sort(&mut aa, &mut ii, 0, n + 1);

        for _ in 0..q {
            line.clear();
            stdin.read_line(&mut line).expect("Failed to read line");
            let mut iter = line.trim().split_whitespace();
            let l: usize = iter.next().unwrap().parse().expect("Invalid input") - 1;
            let r: usize = iter.next().unwrap().parse().expect("Invalid input");

            let a_l = aa[l];
            let a_r = aa[r];
            let i_l = ii[search(a_l, l, &aa, &ii) as usize + 1];
            let i_r = ii[search(a_r, r, &aa, &ii) as usize + 1];

            println!("{}", if a_l == a_r || i_l < i_r { "YES" } else { "NO" });
        }
    }
}